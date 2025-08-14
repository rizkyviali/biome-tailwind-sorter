import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

// CursorPosition interface removed as it's not used

class TailwindSorter {
    private statusBarItem: vscode.StatusBarItem;
    
    constructor() {
        this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
        this.statusBarItem.command = 'biome-tailwind-sorter.sortClasses';
        this.statusBarItem.text = '$(symbol-class) Sort TW';
        this.statusBarItem.tooltip = 'Sort Tailwind CSS Classes';
    }
    
    public showStatusBar(show: boolean = true) {
        if (show) {
            this.statusBarItem.show();
        } else {
            this.statusBarItem.hide();
        }
    }
    
    private getBinaryPath(): string {
        const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
        const configuredPath = config.get<string>('binaryPath');
        
        if (configuredPath && fs.existsSync(configuredPath)) {
            return configuredPath;
        }
        
        // Try to find in node_modules
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
        if (workspaceFolder) {
            const localBinary = path.join(workspaceFolder, 'node_modules', '.bin', 'biome-tailwind-sorter');
            if (fs.existsSync(localBinary)) {
                return localBinary;
            }
            
            const localBinaryTarget = path.join(workspaceFolder, 'target', 'release', 'biome-tailwind-sorter');
            if (fs.existsSync(localBinaryTarget)) {
                return localBinaryTarget;
            }
        }
        
        // Fallback to global installation
        return 'biome-tailwind-sorter';
    }
    
    private calculateOffset(document: vscode.TextDocument, position: vscode.Position): number {
        let offset = 0;
        for (let i = 0; i < position.line; i++) {
            offset += document.lineAt(i).text.length + 1; // +1 for newline
        }
        offset += position.character;
        return offset;
    }
    
    private parseNewCursorPosition(stderr: string): vscode.Position | null {
        const match = stderr.match(/CURSOR_POSITION:(\d+):(\d+):(\d+)/);
        if (match) {
            const line = parseInt(match[1], 10);
            const column = parseInt(match[2], 10);
            return new vscode.Position(line, column);
        }
        return null;
    }
    
    public async sortTailwindClasses(
        document: vscode.TextDocument, 
        preserveCursor: boolean = true,
        cursorPosition?: vscode.Position
    ): Promise<{ content: string; newCursorPosition?: vscode.Position }> {
        return new Promise((resolve, reject) => {
            const binaryPath = this.getBinaryPath();
            const args = ['--write'];
            
            if (preserveCursor && cursorPosition) {
                const offset = this.calculateOffset(document, cursorPosition);
                args.push('--preserve-cursor', '--cursor-offset', offset.toString());
            }
            
            // Use stdin/stdout instead of file for better performance
            args.push('-'); // Read from stdin
            
            const child = cp.spawn(binaryPath, args, {
                stdio: ['pipe', 'pipe', 'pipe']
            });
            
            let stdout = '';
            let stderr = '';
            
            child.stdout.on('data', (data) => {
                stdout += data.toString();
            });
            
            child.stderr.on('data', (data) => {
                stderr += data.toString();
            });
            
            child.on('close', (code) => {
                if (code === 0) {
                    const newCursorPosition = this.parseNewCursorPosition(stderr);
                    resolve({
                        content: stdout,
                        newCursorPosition: newCursorPosition || cursorPosition
                    });
                } else {
                    let errorMessage = `Tailwind sorter exited with code ${code}`;
                    if (stderr.trim()) {
                        errorMessage += `: ${stderr.trim()}`;
                    }
                    
                    // Provide helpful error messages
                    if (code === 127) {
                        errorMessage = 'Binary not found. Please install biome-tailwind-sorter: npm install biome-tailwind-sorter';
                    } else if (code === 126) {
                        errorMessage = 'Permission denied. Please check binary permissions or set a custom path in settings';
                    }
                    
                    reject(new Error(errorMessage));
                }
            });
            
            child.on('error', (error) => {
                let errorMessage = `Failed to execute biome-tailwind-sorter`;
                
                if (error.message.includes('ENOENT')) {
                    errorMessage = 'Binary not found. Please install biome-tailwind-sorter or set custom path in settings';
                } else if (error.message.includes('EACCES')) {
                    errorMessage = 'Permission denied. Please check binary permissions';
                } else {
                    errorMessage += `: ${error.message}`;
                }
                
                reject(new Error(errorMessage));
            });
            
            // Add timeout to prevent hanging
            const timeout = setTimeout(() => {
                child.kill();
                reject(new Error('Operation timed out. The file might be too large or complex'));
            }, 30000); // 30 seconds timeout
            
            child.on('close', () => {
                clearTimeout(timeout);
            });
            
            // Send document content to stdin
            child.stdin.write(document.getText());
            child.stdin.end();
        });
    }
}

export function activate(context: vscode.ExtensionContext) {
    const sorter = new TailwindSorter();
    
    // Show status bar for supported files
    const updateStatusBar = () => {
        const editor = vscode.window.activeTextEditor;
        if (editor) {
            const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
            const languages = config.get<string[]>('languages', [
                'html', 'javascript', 'javascriptreact', 'typescript', 'typescriptreact', 'vue', 'astro'
            ]);
            
            if (languages.includes(editor.document.languageId)) {
                sorter.showStatusBar(true);
            } else {
                sorter.showStatusBar(false);
            }
        } else {
            sorter.showStatusBar(false);
        }
    };
    
    // Update status bar on editor change
    vscode.window.onDidChangeActiveTextEditor(updateStatusBar);
    updateStatusBar();
    
    // Register commands
    const sortClassesCommand = vscode.commands.registerCommand(
        'biome-tailwind-sorter.sortClasses',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                return;
            }
            
            const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
            if (!config.get<boolean>('enable', true)) {
                return;
            }
            
            const document = editor.document;
            const cursorPosition = editor.selection.active;
            
            try {
                // Show progress for longer operations
                await vscode.window.withProgress({
                    location: vscode.ProgressLocation.Notification,
                    title: "Sorting Tailwind classes...",
                    cancellable: false
                }, async (progress) => {
                    progress.report({ increment: 0 });
                    
                    const result = await sorter.sortTailwindClasses(document, true, cursorPosition);
                    
                    progress.report({ increment: 50 });
                    
                    await editor.edit(editBuilder => {
                        const fullRange = new vscode.Range(
                            document.positionAt(0),
                            document.positionAt(document.getText().length)
                        );
                        editBuilder.replace(fullRange, result.content);
                    });
                    
                    progress.report({ increment: 100 });
                    
                    // Restore cursor position
                    if (result.newCursorPosition) {
                        editor.selection = new vscode.Selection(
                            result.newCursorPosition,
                            result.newCursorPosition
                        );
                    }
                });
                
                // Only show success message if classes were actually changed
                const originalContent = document.getText();
                const newContent = document.getText();
                if (originalContent !== newContent) {
                    vscode.window.showInformationMessage('✓ Tailwind classes sorted!');
                }
            } catch (error) {
                const errorMessage = error instanceof Error ? error.message : String(error);
                vscode.window.showErrorMessage(`Failed to sort Tailwind classes: ${errorMessage}`, 'Open Settings')
                    .then(selection => {
                        if (selection === 'Open Settings') {
                            vscode.commands.executeCommand('workbench.action.openSettings', 'biome-tailwind-sorter');
                        }
                    });
            }
        }
    );
    
    const sortClassesInFileCommand = vscode.commands.registerCommand(
        'biome-tailwind-sorter.sortClassesInFile',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                return;
            }
            
            const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
            if (!config.get<boolean>('enable', true)) {
                return;
            }
            
            const document = editor.document;
            
            try {
                const result = await sorter.sortTailwindClasses(document, false);
                
                await editor.edit(editBuilder => {
                    const fullRange = new vscode.Range(
                        document.positionAt(0),
                        document.positionAt(document.getText().length)
                    );
                    editBuilder.replace(fullRange, result.content);
                });
                
                vscode.window.showInformationMessage('Tailwind classes sorted in file!');
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to sort Tailwind classes: ${error}`);
            }
        }
    );
    
    // Register format on save
    const formatOnSaveDisposable = vscode.workspace.onWillSaveTextDocument(async (event) => {
        const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
        
        if (!config.get<boolean>('enable', true) || !config.get<boolean>('formatOnSave', false)) {
            return;
        }
        
        const languages = config.get<string[]>('languages', [
            'html', 'javascript', 'javascriptreact', 'typescript', 'typescriptreact', 'vue', 'astro'
        ]);
        
        if (!languages.includes(event.document.languageId)) {
            return;
        }
        
        try {
            const result = await sorter.sortTailwindClasses(event.document, false);
            
            event.waitUntil(
                vscode.window.activeTextEditor?.edit(editBuilder => {
                    const fullRange = new vscode.Range(
                        event.document.positionAt(0),
                        event.document.positionAt(event.document.getText().length)
                    );
                    editBuilder.replace(fullRange, result.content);
                }) || Promise.resolve(true)
            );
        } catch (error) {
            console.error('Failed to sort Tailwind classes on save:', error);
        }
    });
    
    context.subscriptions.push(
        sortClassesCommand,
        sortClassesInFileCommand,
        formatOnSaveDisposable
    );
}

export function deactivate() {}