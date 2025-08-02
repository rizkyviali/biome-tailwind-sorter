import * as vscode from 'vscode';
import { spawn } from 'child_process';
import * as path from 'path';

let formatOnSaveEnabled = true;

export function activate(context: vscode.ExtensionContext) {
    // Load configuration
    const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
    formatOnSaveEnabled = config.get('formatOnSave', true);

    // Register commands
    const formatCommand = vscode.commands.registerCommand('biome-tailwind-sorter.formatDocument', () => {
        formatCurrentDocument();
    });

    const toggleFormatOnSaveCommand = vscode.commands.registerCommand('biome-tailwind-sorter.toggleFormatOnSave', () => {
        formatOnSaveEnabled = !formatOnSaveEnabled;
        vscode.window.showInformationMessage(`Format on save: ${formatOnSaveEnabled ? 'enabled' : 'disabled'}`);
    });

    // Register format on save
    const onSaveDisposable = vscode.workspace.onDidSaveTextDocument((document) => {
        if (formatOnSaveEnabled && shouldProcessDocument(document)) {
            formatDocument(document);
        }
    });

    context.subscriptions.push(formatCommand, toggleFormatOnSaveCommand, onSaveDisposable);
}

export function deactivate() {}

function shouldProcessDocument(document: vscode.TextDocument): boolean {
    const supportedLanguages = ['javascript', 'typescript', 'javascriptreact', 'typescriptreact', 'html', 'vue'];
    return supportedLanguages.includes(document.languageId);
}

async function formatCurrentDocument() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        return;
    }
    
    await formatDocument(editor.document);
}

async function formatDocument(document: vscode.TextDocument) {
    const editor = vscode.window.visibleTextEditors.find(e => e.document === document);
    if (!editor) {
        return;
    }

    const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
    const preserveCursor = config.get('preserveCursor', true);

    if (!preserveCursor) {
        // Simple formatting without cursor preservation
        await formatDocumentSimple(document);
        return;
    }

    // Get current cursor position
    const cursorPosition = editor.selection.active;
    const cursorOffset = document.offsetAt(cursorPosition);

    try {
        // Get original content
        const originalContent = document.getText();
        
        // Call the Rust binary with cursor preservation - but don't write to file yet
        const result = await runFormatterInMemory(originalContent, cursorOffset);
        
        if (result.changed) {
            // Apply the formatted content using VS Code edit
            const edit = new vscode.WorkspaceEdit();
            const fullRange = new vscode.Range(
                document.positionAt(0),
                document.positionAt(document.getText().length)
            );
            edit.replace(document.uri, fullRange, result.content);
            
            // Apply edit and wait for completion
            const success = await vscode.workspace.applyEdit(edit);
            
            if (success && result.newCursorOffset !== undefined) {
                // Wait for document to be fully updated
                await new Promise(resolve => {
                    const disposable = vscode.workspace.onDidChangeTextDocument(event => {
                        if (event.document === document) {
                            disposable.dispose();
                            resolve(undefined);
                        }
                    });
                    // Fallback timeout in case the event doesn't fire
                    setTimeout(() => {
                        disposable.dispose();
                        resolve(undefined);
                    }, 100);
                });
                
                // Now set cursor position
                const newPosition = document.positionAt(result.newCursorOffset);
                editor.selection = new vscode.Selection(newPosition, newPosition);
                editor.revealRange(new vscode.Range(newPosition, newPosition));
            }
        }
    } catch (error) {
        vscode.window.showErrorMessage(`Biome Tailwind Sorter error: ${error}`);
    }
}

async function formatDocumentSimple(document: vscode.TextDocument) {
    try {
        const result = await runFormatter(document.uri.fsPath);
        
        if (result.changed) {
            const edit = new vscode.WorkspaceEdit();
            const fullRange = new vscode.Range(
                document.positionAt(0),
                document.positionAt(document.getText().length)
            );
            edit.replace(document.uri, fullRange, result.content);
            await vscode.workspace.applyEdit(edit);
        }
    } catch (error) {
        vscode.window.showErrorMessage(`Biome Tailwind Sorter error: ${error}`);
    }
}

interface FormatResult {
    content: string;
    changed: boolean;
    newCursorOffset?: number;
}

function runFormatterInMemory(content: string, cursorOffset?: number): Promise<FormatResult> {
    return new Promise((resolve, reject) => {
        // We'll create a temp file, format it, then read the result
        const fs = require('fs');
        const os = require('os');
        const tmpPath = require('path').join(os.tmpdir(), `biome-tailwind-sorter-${Date.now()}.html`);
        
        try {
            // Write content to temp file
            fs.writeFileSync(tmpPath, content);
            
            // Now run the formatter on the temp file
            runFormatter(tmpPath, cursorOffset).then(result => {
                // Clean up temp file
                try {
                    fs.unlinkSync(tmpPath);
                } catch (e) {
                    // Ignore cleanup errors
                }
                resolve(result);
            }).catch(error => {
                // Clean up temp file on error
                try {
                    fs.unlinkSync(tmpPath);
                } catch (e) {
                    // Ignore cleanup errors
                }
                reject(error);
            });
        } catch (error) {
            reject(`Failed to create temporary file: ${error}`);
        }
    });
}

function runFormatter(filePath: string, cursorOffset?: number): Promise<FormatResult> {
    return new Promise((resolve, reject) => {
        // Find the binary path - first try the package binary, then fall back to npx
        const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
        let binaryPath = 'npx';
        let args = ['biome-tailwind-sorter'];

        // Check if we're in the package directory and can use the local binary
        if (workspaceRoot) {
            const localBinaryPath = path.join(workspaceRoot, 'target', 'release', 'biome-tailwind-sorter');
            try {
                require('fs').accessSync(localBinaryPath);
                binaryPath = localBinaryPath;
                args = [];
            } catch {
                // Fall back to npx
            }
        }

        // Build command arguments
        args.push('--write');
        
        if (cursorOffset !== undefined) {
            args.push('--preserve-cursor', '--cursor-offset', cursorOffset.toString());
        }
        
        args.push(filePath);

        let stdout = '';
        let stderr = '';

        const process = spawn(binaryPath, args, {
            cwd: workspaceRoot || path.dirname(filePath)
        });

        process.stdout.on('data', (data) => {
            stdout += data.toString();
        });

        process.stderr.on('data', (data) => {
            stderr += data.toString();
        });

        process.on('close', (code) => {
            if (code === 0) {
                // Read the formatted file content
                try {
                    const fs = require('fs');
                    const content = fs.readFileSync(filePath, 'utf8');
                    
                    // Parse cursor position from stderr if available
                    let newCursorOffset: number | undefined;
                    const cursorMatch = stderr.match(/CURSOR_POSITION:\d+:\d+:(\d+)/);
                    if (cursorMatch) {
                        newCursorOffset = parseInt(cursorMatch[1], 10);
                    }

                    // For temp files, we assume changes happened if the formatter ran successfully
                    const actuallyChanged = true;

                    resolve({
                        content,
                        changed: actuallyChanged,
                        newCursorOffset
                    });
                } catch (error) {
                    reject(`Failed to read formatted file: ${error}`);
                }
            } else {
                // If exit code is 1, it might mean no changes were needed
                if (code === 1) {
                    try {
                        const fs = require('fs');
                        const content = fs.readFileSync(filePath, 'utf8');
                        resolve({
                            content,
                            changed: false,
                            newCursorOffset: cursorOffset // Keep original cursor position
                        });
                    } catch (error) {
                        reject(`Failed to read file: ${error}`);
                    }
                } else {
                    reject(`Formatter exited with code ${code}: ${stderr}`);
                }
            }
        });

        process.on('error', (error) => {
            reject(`Failed to run formatter: ${error.message}`);
        });
    });
}