"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const assert = require("assert");
const vscode = require("vscode");
suite('Extension Test Suite', () => {
    vscode.window.showInformationMessage('Start all tests.');
    test('Extension should be present', () => {
        assert.ok(vscode.extensions.getExtension('rizkyviali.biome-tailwind-sorter-vscode'));
    });
    test('Extension should activate', async () => {
        const extension = vscode.extensions.getExtension('rizkyviali.biome-tailwind-sorter-vscode');
        if (extension) {
            await extension.activate();
            assert.strictEqual(extension.isActive, true);
        }
    });
    test('Commands should be registered', async () => {
        const commands = await vscode.commands.getCommands(true);
        assert.ok(commands.includes('biome-tailwind-sorter.sortClasses'));
        assert.ok(commands.includes('biome-tailwind-sorter.sortClassesInFile'));
    });
    test('Configuration should have default values', () => {
        const config = vscode.workspace.getConfiguration('biome-tailwind-sorter');
        assert.strictEqual(config.get('enable'), true);
        assert.strictEqual(config.get('formatOnSave'), false);
        assert.strictEqual(config.get('binaryPath'), '');
        const languages = config.get('languages');
        assert.ok(Array.isArray(languages));
        assert.ok(languages.includes('html'));
        assert.ok(languages.includes('javascript'));
        assert.ok(languages.includes('typescript'));
    });
});
//# sourceMappingURL=extension.test.js.map