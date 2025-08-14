use std::process::Command;
use std::fs;
use tempfile::TempDir;

fn get_binary_path() -> String {
    let output = Command::new("cargo")
        .args(["build", "--release", "--message-format=json"])
        .output()
        .expect("Failed to build binary");
        
    // Parse the output to find the binary path
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if json["reason"] == "compiler-artifact" 
                && json["target"]["name"] == "biome-tailwind-sorter"
                && json["target"]["kind"].as_array().unwrap().contains(&serde_json::Value::String("bin".to_string())) {
                return json["executable"].as_str().unwrap().to_string();
            }
        }
    }
    
    // Fallback to standard path
    "./target/release/biome-tailwind-sorter".to_string()
}

fn create_test_file(content: &str, temp_dir: &TempDir) -> String {
    let file_path = temp_dir.path().join("test.html");
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path.to_string_lossy().to_string()
}

#[test]
fn test_cli_check_mode_needs_formatting() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = create_test_file(
        r#"<div class="text-red-500 p-4 flex">test</div>"#,
        &temp_dir
    );
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--check", &file_path])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 1, "Should exit with code 1 when formatting needed");
}

#[test]
fn test_cli_check_mode_already_formatted() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = create_test_file(
        r#"<div class="flex p-4 text-red-500">test</div>"#,
        &temp_dir
    );
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--check", &file_path])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0 when already formatted");
}

#[test]
fn test_cli_write_mode_formats_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = create_test_file(
        r#"<div class="text-red-500 p-4 flex">test</div>"#,
        &temp_dir
    );
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--write", &file_path])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0 on successful write");
    
    let formatted_content = fs::read_to_string(&file_path).expect("Failed to read formatted file");
    assert!(formatted_content.contains(r#"class="flex p-4 text-red-500""#), 
            "File should be formatted with correct order. Got: {formatted_content}");
}

#[test]
fn test_cli_cursor_preservation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = create_test_file(
        r#"<div class="text-red-500 p-4 flex">test</div>"#,
        &temp_dir
    );
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--write", "--preserve-cursor", "--cursor-offset", "20", &file_path])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CURSOR_POSITION:"), 
            "Should output cursor position. Got stderr: {stderr}");
}

#[test]
fn test_cli_verbose_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = create_test_file(
        r#"<div class="text-red-500 p-4 flex">test</div>"#,
        &temp_dir
    );
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--write", "--verbose", &file_path])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Formatted") || stdout.contains("formatted"), 
            "Should show formatted message in verbose mode. Got: {stdout}");
}

#[test]
fn test_cli_multiple_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file1 = create_test_file(
        r#"<div class="text-red-500 p-4 flex">test1</div>"#,
        &temp_dir
    );
    let file2_path = temp_dir.path().join("test2.html");
    fs::write(&file2_path, r#"<div class="text-blue-500 m-2 inline">test2</div>"#)
        .expect("Failed to write test file 2");
    let file2 = file2_path.to_string_lossy().to_string();
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--write", &file1, &file2])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0");
    
    // Check both files are formatted
    let content1 = fs::read_to_string(&file1).expect("Failed to read file 1");
    let content2 = fs::read_to_string(&file2).expect("Failed to read file 2");
    
    assert!(content1.contains(r#"class="flex p-4 text-red-500""#));
    assert!(content2.contains(r#"class="inline m-2 text-blue-500""#));
}

#[test]
fn test_cli_directory_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create multiple files in directory
    let file1_path = temp_dir.path().join("component1.tsx");
    fs::write(&file1_path, r#"<div className="text-red-500 p-4 flex">test</div>"#)
        .expect("Failed to write tsx file");
        
    let file2_path = temp_dir.path().join("component2.vue");
    fs::write(&file2_path, r#"<div class="text-blue-500 m-2 inline">test</div>"#)
        .expect("Failed to write vue file");
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--write", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0");
    
    // Check files are formatted
    let content1 = fs::read_to_string(&file1_path).expect("Failed to read tsx file");
    let content2 = fs::read_to_string(&file2_path).expect("Failed to read vue file");
    
    assert!(content1.contains(r#"className="flex p-4 text-red-500""#));
    assert!(content2.contains(r#"class="inline m-2 text-blue-500""#));
}

#[test]
fn test_cli_unsupported_files_ignored() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create unsupported file
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "some text content").expect("Failed to write txt file");
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--check", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");
        
    // Should exit with error since no supported files found
    assert_ne!(output.status.code().unwrap(), 0, "Should exit with error when no supported files");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No supported files found"));
}

#[test]
fn test_cli_multiline_classes() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = create_test_file(
        r#"<div
  class="
    text-white
    bg-red-500
    p-4
    hover:bg-red-600
    rounded-lg
    font-semibold
  "
>
  Button
</div>"#,
        &temp_dir
    );
    
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(["--write", &file_path])
        .output()
        .expect("Failed to execute command");
        
    assert_eq!(output.status.code().unwrap(), 0, "Should exit with code 0");
    
    let formatted_content = fs::read_to_string(&file_path).expect("Failed to read formatted file");
    // Should preserve multiline structure but sort classes
    assert!(formatted_content.contains("p-4"));
    assert!(formatted_content.contains("font-semibold"));
    assert!(formatted_content.contains("bg-red-500"));
    // Classes should be in correct order (p-4 before font-semibold before bg-red-500)
    let p4_pos = formatted_content.find("p-4").unwrap();
    let font_pos = formatted_content.find("font-semibold").unwrap();
    let bg_pos = formatted_content.find("bg-red-500").unwrap();
    assert!(p4_pos < font_pos && font_pos < bg_pos, 
            "Classes should be in correct order. Content: {formatted_content}");
}