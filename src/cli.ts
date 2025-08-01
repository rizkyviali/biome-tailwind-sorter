#!/usr/bin/env node

/**
 * CLI tool for sorting Tailwind CSS classes
 * This can be used as a standalone formatter or integrated with Biome
 */

import * as fs from 'fs';
import * as path from 'path';
import { formatDocument } from './tailwind-formatter';

interface CliOptions {
  write: boolean;
  check: boolean;
  verbose: boolean;
}

/**
 * Parse command line arguments
 */
function parseArgs(): { files: string[]; options: CliOptions } {
  const args = process.argv.slice(2);
  const options: CliOptions = {
    write: false,
    check: false,
    verbose: false,
  };
  const files: string[] = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    
    switch (arg) {
      case '--write':
      case '-w':
        options.write = true;
        break;
      case '--check':
      case '-c':
        options.check = true;
        break;
      case '--verbose':
      case '-v':
        options.verbose = true;
        break;
      case '--help':
      case '-h':
        printHelp();
        process.exit(0);
        break;
      default:
        if (!arg.startsWith('-')) {
          files.push(arg);
        }
        break;
    }
  }

  return { files, options };
}

/**
 * Print help message
 */
function printHelp(): void {
  console.log(`
Tailwind CSS Class Sorter

Usage: npx biome-tailwind-sorter [options] <files...>

Options:
  --write, -w      Write sorted classes back to files
  --check, -c      Check if files need sorting (exit code 1 if changes needed)
  --verbose, -v    Verbose output
  --help, -h       Show this help message

Examples:
  npx biome-tailwind-sorter src/**/*.{js,jsx,ts,tsx,html,vue}
  npx biome-tailwind-sorter --write src/components/Button.tsx
  npx biome-tailwind-sorter --check src/
`);
}

/**
 * Check if file should be processed
 */
function shouldProcessFile(filePath: string): boolean {
  const supportedExtensions = ['.js', '.jsx', '.ts', '.tsx', '.html', '.vue'];
  const ext = path.extname(filePath);
  return supportedExtensions.includes(ext);
}

/**
 * Get all files matching the pattern
 */
function getFiles(patterns: string[]): string[] {
  const files: string[] = [];
  
  for (const pattern of patterns) {
    if (fs.existsSync(pattern)) {
      const stat = fs.statSync(pattern);
      
      if (stat.isFile()) {
        if (shouldProcessFile(pattern)) {
          files.push(pattern);
        }
      } else if (stat.isDirectory()) {
        // Simple directory traversal (for basic use case)
        const dirFiles = fs.readdirSync(pattern);
        for (const file of dirFiles) {
          const fullPath = path.join(pattern, file);
          if (fs.statSync(fullPath).isFile() && shouldProcessFile(fullPath)) {
            files.push(fullPath);
          }
        }
      }
    }
  }
  
  return files;
}

/**
 * Format a single file
 */
function formatFile(filePath: string, options: CliOptions): { changed: boolean; error?: string } {
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    const formatted = formatDocument(content);
    const changed = content !== formatted;
    
    if (changed) {
      if (options.write) {
        fs.writeFileSync(filePath, formatted, 'utf8');
        if (options.verbose) {
          console.log(`✓ Formatted ${filePath}`);
        }
      } else {
        if (options.verbose) {
          console.log(`⚠ ${filePath} needs formatting`);
        }
      }
    } else {
      if (options.verbose) {
        console.log(`✓ ${filePath} is already formatted`);
      }
    }
    
    return { changed };
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    if (options.verbose) {
      console.error(`✗ Error processing ${filePath}: ${errorMessage}`);
    }
    return { changed: false, error: errorMessage };
  }
}

/**
 * Main CLI function
 */
function main(): void {
  const { files: patterns, options } = parseArgs();
  
  if (patterns.length === 0) {
    console.error('Error: No files specified');
    printHelp();
    process.exit(1);
  }
  
  const files = getFiles(patterns);
  
  if (files.length === 0) {
    console.error('Error: No supported files found');
    process.exit(1);
  }
  
  let changedFiles = 0;
  let errorFiles = 0;
  
  for (const file of files) {
    const result = formatFile(file, options);
    
    if (result.error) {
      errorFiles++;
    } else if (result.changed) {
      changedFiles++;
    }
  }
  
  // Summary
  if (options.verbose || !options.write) {
    console.log(`\nProcessed ${files.length} files:`);
    console.log(`  ${changedFiles} files ${options.write ? 'formatted' : 'need formatting'}`);
    console.log(`  ${files.length - changedFiles - errorFiles} files already formatted`);
    
    if (errorFiles > 0) {
      console.log(`  ${errorFiles} files had errors`);
    }
  }
  
  // Exit codes
  if (errorFiles > 0) {
    process.exit(2); // Errors occurred
  } else if (options.check && changedFiles > 0) {
    process.exit(1); // Files need formatting
  } else {
    process.exit(0); // Success
  }
}

// Run CLI if this file is executed directly
if (require.main === module) {
  main();
}