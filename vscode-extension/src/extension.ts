import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

interface CursorPosition {
    line: number;
    column: number;
    offset: number;
}

class TailwindSorter {
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
                    reject(new Error(`biome-tailwind-sorter exited with code ${code}: ${stderr}`));
                }
            });
            
            child.on('error', (error) => {
                reject(new Error(`Failed to spawn biome-tailwind-sorter: ${error.message}`));
            });
            
            // Send document content to stdin
            child.stdin.write(document.getText());
            child.stdin.end();
        });
    }
}

export function activate(context: vscode.ExtensionContext) {
    const sorter = new TailwindSorter();
    
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
                const result = await sorter.sortTailwindClasses(document, true, cursorPosition);
                
                await editor.edit(editBuilder => {
                    const fullRange = new vscode.Range(
                        document.positionAt(0),
                        document.positionAt(document.getText().length)
                    );
                    editBuilder.replace(fullRange, result.content);
                });
                
                // Restore cursor position
                if (result.newCursorPosition) {
                    editor.selection = new vscode.Selection(
                        result.newCursorPosition,
                        result.newCursorPosition
                    );
                }
                
                vscode.window.showInformationMessage('Tailwind classes sorted!');
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to sort Tailwind classes: ${error}`);
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