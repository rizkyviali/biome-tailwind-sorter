mod tailwind_order;
mod class_extractor;
mod formatter;
mod config;

use clap::{Arg, Command};
use formatter::{TailwindFormatter, CursorPosition};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

fn main() {
    let matches = Command::new("biome-tailwind-sorter")
        .version("0.2.9")
        .about("A high-performance Rust CLI tool for sorting Tailwind CSS classes according to official order")
        .arg(
            Arg::new("files")
                .help("Files to process (use '-' for stdin)")
                .value_name("FILES")
                .num_args(0..)
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
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Enable debug output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("max-size")
                .long("max-size")
                .help("Maximum file size in MB (default: 10)")
                .value_name("SIZE")
                .value_parser(clap::value_parser!(f64))
        )
        .arg(
            Arg::new("doctor")
                .long("doctor")
                .help("Check installation and configuration")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let files: Vec<&String> = matches.get_many::<String>("files").map(|f| f.collect()).unwrap_or_default();
    let write = matches.get_flag("write");
    let check = matches.get_flag("check");
    let verbose = matches.get_flag("verbose");
    let debug = matches.get_flag("debug");
    let preserve_cursor = matches.get_flag("preserve-cursor");
    let max_size_mb = matches.get_one::<f64>("max-size").copied().unwrap_or(10.0);
    let doctor = matches.get_flag("doctor");

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

    // Handle doctor mode
    if doctor {
        run_doctor_check(debug);
        return;
    }

    // Handle stdin input
    if files.is_empty() || (files.len() == 1 && files[0] == "-") {
        if debug {
            eprintln!("Debug: Processing stdin input");
            if let Some(ref cursor) = cursor_position {
                eprintln!("Debug: Cursor position - line: {}, column: {}, offset: {}", 
                         cursor.line, cursor.column, cursor.offset);
            }
        }
        if let Err(err) = process_stdin(&TailwindFormatter::new(preserve_cursor), cursor_position, write, debug) {
            eprintln!("Error processing stdin: {err}");
            if debug {
                eprintln!("Debug: Error details - {err:#?}");
            }
            process::exit(1);
        }
        return;
    }

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

    if debug {
        eprintln!("Debug: Found {} files to process", expanded_files.len());
        for file in &expanded_files {
            eprintln!("Debug: Will process: {file}");
        }
    }

    for file_path in &expanded_files {
        match process_file(&formatter, file_path, write, verbose || debug, cursor_position.clone(), max_size_mb) {
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

fn run_doctor_check(debug: bool) {
    println!("🔍 Biome Tailwind Sorter - Installation Check\n");
    
    // Check binary version
    println!("✅ Binary Version: {}", env!("CARGO_PKG_VERSION"));
    
    // Check current directory
    match std::env::current_dir() {
        Ok(dir) => println!("📁 Current Directory: {}", dir.display()),
        Err(e) => println!("❌ Cannot access current directory: {e}"),
    }
    
    // Check for configuration files
    println!("\n🔧 Configuration Files:");
    let config_files = [
        "tailwind.config.js",
        "tailwind.config.mjs", 
        "tailwind.config.ts",
        "tailwind.config.json",
        ".tailwindsorterrc",
        ".tailwindsorterrc.json",
        "biome-tailwind-sorter.json",
        "package.json",
    ];
    
    let mut found_configs = 0;
    for config_file in &config_files {
        if fs::metadata(config_file).is_ok() {
            println!("  ✅ Found: {config_file}");
            found_configs += 1;
            
            if debug && config_file.starts_with("tailwind.config") {
                if let Ok(content) = fs::read_to_string(config_file) {
                    println!("    📄 Size: {} bytes", content.len());
                    if content.contains("content:") {
                        println!("    ✅ Contains content configuration");
                    }
                }
            }
        }
    }
    
    if found_configs == 0 {
        println!("  ⚠️  No configuration files found");
    }
    
    // Test with sample content
    println!("\n🧪 Testing with sample content:");
    let test_html = r#"<div class="text-red-500 p-4 flex bg-blue-200">Test</div>"#;
    let formatter = formatter::TailwindFormatter::new(false);
    let result = formatter.format_document(test_html, None);
    
    if result.changed {
        println!("  ✅ Sorting works correctly");
        println!("  📝 Before: {test_html}");
        println!("  📝 After:  {}", result.content);
    } else {
        println!("  ⚠️  No changes detected (classes may already be sorted)");
    }
    
    // Check for supported files in current directory
    println!("\n📄 Supported files in current directory:");
    let mut file_count = 0;
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            if let Some(path_str) = entry.path().to_str() {
                if should_process_file(path_str) {
                    println!("  ✅ {}", entry.file_name().to_string_lossy());
                    file_count += 1;
                    if file_count >= 5 {
                        println!("  ... and more");
                        break;
                    }
                }
            }
        }
    }
    
    if file_count == 0 {
        println!("  ⚠️  No supported files found (.js, .jsx, .ts, .tsx, .html, .vue, .astro)");
    }
    
    // Performance test
    println!("\n⚡ Performance Test:");
    let large_test = format!("{} ", test_html).repeat(1000);
    let start = std::time::Instant::now();
    let _result = formatter.format_document(&large_test, None);
    let duration = start.elapsed();
    println!("  ✅ Processed {} bytes in {:.2}ms", large_test.len(), duration.as_secs_f64() * 1000.0);
    
    println!("\n🎉 Installation check complete!");
    
    if found_configs > 0 && file_count > 0 {
        println!("\n💡 Try running: biome-tailwind-sorter --write .");
    }
}

fn process_stdin(
    formatter: &TailwindFormatter,
    cursor_position: Option<CursorPosition>,
    write: bool,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    if debug {
        eprintln!("Debug: Input length: {} bytes", input.len());
        eprintln!("Debug: Input contains newlines: {}", input.contains('\n'));
    }
    
    let result = formatter.format_document(&input, cursor_position);
    
    if debug {
        eprintln!("Debug: Content changed: {}", result.changed);
        eprintln!("Debug: Output length: {} bytes", result.content.len());
        if let Some(ref cursor) = result.cursor_position {
            eprintln!("Debug: New cursor position - line: {}, column: {}, offset: {}", 
                     cursor.line, cursor.column, cursor.offset);
        }
    }
    
    if write || !result.changed {
        // Output formatted content to stdout
        io::stdout().write_all(result.content.as_bytes())?;
        io::stdout().flush()?;
        
        // Output cursor position to stderr if available
        if let Some(cursor) = result.cursor_position {
            eprintln!("CURSOR_POSITION:{}:{}:{}", cursor.line, cursor.column, cursor.offset);
        }
    } else {
        // In check mode, just return exit code
        if result.changed {
            process::exit(1);
        }
    }
    
    Ok(())
}

fn process_file(
    formatter: &TailwindFormatter,
    file_path: &str,
    write: bool,
    verbose: bool,
    cursor_position: Option<CursorPosition>,
    max_size_mb: f64,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Validate file exists and is readable
    let metadata = fs::metadata(file_path)
        .map_err(|e| format!("Cannot access file '{file_path}': {e}"))?;
    
    if !metadata.is_file() {
        return Err(format!("'{file_path}' is not a regular file").into());
    }
    
    // Check file size with configurable limits and better error messages
    let max_file_size = (max_size_mb * 1024.0 * 1024.0) as u64;
    let warn_file_size = (max_size_mb * 0.1 * 1024.0 * 1024.0) as u64; // Warn at 10% of max
    
    if metadata.len() > max_file_size {
        return Err(format!("File '{}' is too large ({:.1} MB). Maximum size is {:.1} MB. Use --max-size to increase limit.", 
                          file_path, 
                          metadata.len() as f64 / (1024.0 * 1024.0),
                          max_size_mb).into());
    }
    
    if verbose && metadata.len() > warn_file_size {
        eprintln!("Warning: Large file detected ({:.1} MB): {}", 
                 metadata.len() as f64 / (1024.0 * 1024.0), file_path);
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
                    // Recursive directory traversal
                    collect_files_recursively(pattern, &mut files);
                }
            }
            Err(_) => {
                eprintln!("Warning: Cannot access path '{pattern}'");
            }
        }
    }
    
    files
}

fn collect_files_recursively(dir_path: &str, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                if path.is_file() {
                    if should_process_file(path_str) {
                        files.push(path_str.to_string());
                    }
                } else if path.is_dir() {
                    // Skip common directories that should be ignored
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !should_ignore_directory(dir_name) {
                        collect_files_recursively(path_str, files);
                    }
                }
            }
        }
    }
}

fn should_ignore_directory(dir_name: &str) -> bool {
    matches!(dir_name, 
        "node_modules" | ".git" | ".svn" | ".hg" | 
        "target" | "build" | "dist" | ".next" | 
        ".nuxt" | ".cache" | ".temp" | ".tmp" |
        "__pycache__" | ".pytest_cache" | ".coverage" |
        ".idea" | ".vscode" | ".DS_Store"
    )
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
            10.0, // max_size_mb
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
            10.0, // max_size_mb
        ).unwrap();
        
        assert!(result); // Changes needed
    }
}