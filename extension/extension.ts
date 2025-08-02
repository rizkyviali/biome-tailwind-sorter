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
        // Call the Rust binary with cursor preservation
        const result = await runFormatter(document.uri.fsPath, cursorOffset);
        
        if (result.changed) {
            // Apply the formatted content
            const edit = new vscode.WorkspaceEdit();
            const fullRange = new vscode.Range(
                document.positionAt(0),
                document.positionAt(document.getText().length)
            );
            edit.replace(document.uri, fullRange, result.content);
            await vscode.workspace.applyEdit(edit);

            // Restore cursor position if available
            if (result.newCursorOffset !== undefined) {
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

                    resolve({
                        content,
                        changed: true, // Assume changed if formatter ran successfully
                        newCursorOffset
                    });
                } catch (error) {
                    reject(`Failed to read formatted file: ${error}`);
                }
            } else {
                reject(`Formatter exited with code ${code}: ${stderr}`);
            }
        });

        process.on('error', (error) => {
            reject(`Failed to run formatter: ${error.message}`);
        });
    });
}