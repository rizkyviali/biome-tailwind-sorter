mod tailwind_order;
mod class_extractor;
mod formatter;
mod config;

use clap::{Arg, Command};
use formatter::{TailwindFormatter, CursorPosition};
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let matches = Command::new("biome-tailwind-sorter")
        .version("0.2.7")
        .about("A high-performance Rust CLI tool for sorting Tailwind CSS classes according to official order")
        .arg(
            Arg::new("files")
                .help("Files to process")
                .value_name("FILES")
                .num_args(1..)
                .required(true)
        )
        .arg(
            Arg::new("write")
                .long("write")
                .short('w')
                .help("Write sorted classes back to files")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("check")
                .long("check")
                .short('c')
                .help("Check if files need sorting (exit code 1 if changes needed)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("preserve-cursor")
                .long("preserve-cursor")
                .help("Preserve cursor position (for editor integration)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("cursor-line")
                .long("cursor-line")
                .help("Current cursor line (0-based)")
                .value_name("LINE")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("cursor-column")
                .long("cursor-column")
                .help("Current cursor column (0-based)")
                .value_name("COLUMN")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("cursor-offset")
                .long("cursor-offset")
                .help("Current cursor offset")
                .value_name("OFFSET")
                .value_parser(clap::value_parser!(usize))
        )
        .get_matches();

    let files: Vec<&String> = matches.get_many::<String>("files").unwrap().collect();
    let write = matches.get_flag("write");
    let check = matches.get_flag("check");
    let verbose = matches.get_flag("verbose");
    let preserve_cursor = matches.get_flag("preserve-cursor");

    // Parse cursor position if provided
    let cursor_position = if preserve_cursor {
        let line = matches.get_one::<usize>("cursor-line").copied();
        let column = matches.get_one::<usize>("cursor-column").copied();
        let offset = matches.get_one::<usize>("cursor-offset").copied();
        
        match (line, column, offset) {
            (Some(line), Some(column), Some(offset)) => Some(CursorPosition {
                line,
                column,
                offset,
            }),
            (None, None, Some(offset)) => Some(CursorPosition {
                line: 0, // Will be calculated
                column: 0, // Will be calculated
                offset,
            }),
            _ => None,
        }
    } else {
        None
    };

    let expanded_files = get_files(&files);
    
    if expanded_files.is_empty() {
        eprintln!("Error: No supported files found in the specified paths.");
        eprintln!("Supported extensions: .js, .jsx, .ts, .tsx, .html, .vue, .astro");
        eprintln!("Try specifying a directory or file with supported extensions.");
        process::exit(1);
    }

    let formatter = TailwindFormatter::new(preserve_cursor);
    let mut changed_files = 0;
    let mut error_files = 0;

    for file_path in &expanded_files {
        match process_file(&formatter, file_path, write, verbose, cursor_position.clone()) {
            Ok(changed) => {
                if changed {
                    changed_files += 1;
                }
            }
            Err(err) => {
                eprintln!("✗ Error processing {file_path}: {err}");
                if let Some(source) = err.source() {
                    eprintln!("   Caused by: {source}");
                }
                error_files += 1;
            }
        }
    }

    // Summary
    if verbose || !write {
        println!("\nProcessed {} files:", expanded_files.len());
        println!("  {} files {}", changed_files, if write { "formatted" } else { "need formatting" });
        println!("  {} files already formatted", expanded_files.len() - changed_files - error_files);
        
        if error_files > 0 {
            println!("  {error_files} files had errors");
        }
    }

    // Exit codes
    if error_files > 0 {
        process::exit(2); // Errors occurred
    } else if check && changed_files > 0 {
        process::exit(1); // Files need formatting
    }
}

fn process_file(
    formatter: &TailwindFormatter,
    file_path: &str,
    write: bool,
    verbose: bool,
    cursor_position: Option<CursorPosition>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Validate file exists and is readable
    let metadata = fs::metadata(file_path)
        .map_err(|e| format!("Cannot access file '{file_path}': {e}"))?;
    
    if !metadata.is_file() {
        return Err(format!("'{file_path}' is not a regular file").into());
    }
    
    // Check file size (prevent processing very large files)
    const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!("File '{}' is too large ({} bytes). Maximum size is {} bytes.", 
                          file_path, metadata.len(), MAX_FILE_SIZE).into());
    }
    
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{file_path}': {e}"))?;
    let result = formatter.format_document(&content, cursor_position);
    
    if result.changed {
        if write {
            // Create backup before writing (optional safety measure)
            fs::write(file_path, &result.content)
                .map_err(|e| format!("Failed to write to file '{file_path}': {e}"))?;
            
            // Output cursor position if requested and available
            if let Some(cursor) = result.cursor_position {
                // Write cursor position to stderr so it doesn't interfere with file content
                eprintln!("CURSOR_POSITION:{}:{}:{}", cursor.line, cursor.column, cursor.offset);
            }
            
            if verbose {
                println!("✓ Formatted {file_path}");
            }
        } else if verbose {
            println!("⚠ {file_path} needs formatting");
        }
    } else if verbose {
        println!("✓ {file_path} is already formatted");
    }
    
    Ok(result.changed)
}

fn get_files(patterns: &[&String]) -> Vec<String> {
    let mut files = Vec::new();
    
    for pattern in patterns {
        match fs::metadata(pattern) {
            Ok(metadata) => {
                if metadata.is_file() {
                    if should_process_file(pattern) {
                        files.push(pattern.to_string());
                    }
                } else if metadata.is_dir() {
                    // Simple directory traversal
                    if let Ok(entries) = fs::read_dir(pattern) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(path_str) = path.to_str() {
                                    if should_process_file(path_str) {
                                        files.push(path_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("Warning: Cannot access path '{pattern}'");
            }
        }
    }
    
    files
}

fn should_process_file(file_path: &str) -> bool {
    let supported_extensions = [".js", ".jsx", ".ts", ".tsx", ".html", ".vue", ".astro"];
    if let Some(extension) = Path::new(file_path).extension() {
        if let Some(ext_str) = extension.to_str() {
            return supported_extensions.contains(&format!(".{ext_str}").as_str());
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_should_process_file() {
        assert!(should_process_file("test.js"));
        assert!(should_process_file("test.jsx"));
        assert!(should_process_file("test.ts"));
        assert!(should_process_file("test.tsx"));
        assert!(should_process_file("test.html"));
        assert!(should_process_file("test.vue"));
        assert!(should_process_file("test.astro"));
        assert!(!should_process_file("test.txt"));
        assert!(!should_process_file("test.css"));
    }

    #[test]
    fn test_process_file_no_changes() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"<div class="flex p-4 text-red-500">test</div>"#).unwrap();
        
        let formatter = TailwindFormatter::new(false);
        let result = process_file(
            &formatter,
            temp_file.path().to_str().unwrap(),
            false,
            false,
            None,
        ).unwrap();
        
        assert!(!result); // No changes needed
    }

    #[test]
    fn test_process_file_with_changes() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"<div class="text-red-500 p-4 flex">test</div>"#).unwrap();
        
        let formatter = TailwindFormatter::new(false);
        let result = process_file(
            &formatter,
            temp_file.path().to_str().unwrap(),
            false,
            false,
            None,
        ).unwrap();
        
        assert!(result); // Changes needed
    }
}