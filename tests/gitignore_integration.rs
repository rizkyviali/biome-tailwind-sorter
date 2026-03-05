use std::fs;
use std::process::Command;
use tempfile::tempdir;
use std::path::PathBuf;

fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // deps
    path.pop(); // debug
    path.join("biome-tailwind-sorter")
}

#[test]
fn test_gitignore_basic_filtering() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // root/
    //   .git/
    //   .gitignore (contains "ignored.html")
    //   included.html
    //   ignored.html
    
    fs::create_dir(root.join(".git")).unwrap();
    fs::write(root.join(".gitignore"), "ignored.html").unwrap();
    fs::write(root.join("included.html"), r#"<div class="p-4 flex"></div>"#).unwrap();
    fs::write(root.join("ignored.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let output = Command::new(get_binary_path())
        .arg("--debug")
        .arg(root)
        .output()
        .expect("Failed to execute biome-tailwind-sorter");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(stderr.contains("included.html"), "Should include included.html");
    assert!(!stderr.contains("ignored.html"), "Should ignore ignored.html");
}

#[test]
fn test_no_gitignore_uses_built_in_defaults() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // root/ (no .gitignore)
    //   included.html
    //   node_modules/
    //     any.html
    //   target/
    //     any.html
    //   .git/
    //     config
    
    fs::write(root.join("included.html"), r#"<div class="p-4 flex"></div>"#).unwrap();
    
    let node_modules = root.join("node_modules");
    fs::create_dir(&node_modules).unwrap();
    fs::write(node_modules.join("any.html"), r#"<div class="p-4 flex"></div>"#).unwrap();
    
    let target = root.join("target");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("any.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let dot_git = root.join(".git");
    fs::create_dir(&dot_git).unwrap();
    fs::write(dot_git.join("config"), "something").unwrap();

    let output = Command::new(get_binary_path())
        .arg("--debug")
        .arg(root)
        .output()
        .expect("Failed to execute biome-tailwind-sorter");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(stderr.contains("included.html"), "Should include included.html");
    assert!(!stderr.contains("node_modules/any.html"), "Should ignore node_modules");
    assert!(!stderr.contains("target/any.html"), "Should ignore target");
    assert!(!stderr.contains(".git/config"), "Should ignore .git");
}

#[test]
fn test_nested_gitignore_overrides() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // root/
    //   .git/
    //   .gitignore (contains "ignored_at_root.html")
    //   included.html
    //   ignored_at_root.html
    //   subdir/
    //     .gitignore (contains "ignored_at_sub.html")
    //     included_sub.html
    //     ignored_at_sub.html

    fs::create_dir(root.join(".git")).unwrap();
    fs::write(root.join(".gitignore"), "ignored_at_root.html").unwrap();
    fs::write(root.join("included.html"), r#"<div class="p-4 flex"></div>"#).unwrap();
    fs::write(root.join("ignored_at_root.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let subdir = root.join("subdir");
    fs::create_dir(&subdir).unwrap();
    fs::write(subdir.join(".gitignore"), "ignored_at_sub.html").unwrap();
    fs::write(subdir.join("included_sub.html"), r#"<div class="p-4 flex"></div>"#).unwrap();
    fs::write(subdir.join("ignored_at_sub.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let output = Command::new(get_binary_path())
        .arg("--debug")
        .arg(root)
        .output()
        .expect("Failed to execute biome-tailwind-sorter");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(stderr.contains("included.html"));
    assert!(stderr.contains("included_sub.html"));
    assert!(!stderr.contains("ignored_at_root.html"));
    assert!(!stderr.contains("ignored_at_sub.html"));
}

#[test]
fn test_hidden_files_ignored_by_default() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // root/
    //   included.html
    //   .hidden.html
    
    fs::write(root.join("included.html"), r#"<div class="p-4 flex"></div>"#).unwrap();
    fs::write(root.join(".hidden.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let output = Command::new(get_binary_path())
        .arg("--debug")
        .arg(root)
        .output()
        .expect("Failed to execute biome-tailwind-sorter");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(stderr.contains("included.html"));
    assert!(!stderr.contains(".hidden.html"), "Hidden files should be ignored by default");
}

#[test]
fn test_multiple_input_paths() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // dir1/a.html
    // dir2/b.html
    // node_modules/ignored.html
    
    let dir1 = root.join("dir1");
    fs::create_dir(&dir1).unwrap();
    fs::write(dir1.join("a.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let dir2 = root.join("dir2");
    fs::create_dir(&dir2).unwrap();
    fs::write(dir2.join("b.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let node_modules = root.join("node_modules");
    fs::create_dir(&node_modules).unwrap();
    fs::write(node_modules.join("ignored.html"), r#"<div class="p-4 flex"></div>"#).unwrap();

    let output = Command::new(get_binary_path())
        .arg("--debug")
        .arg(&dir1)
        .arg(&dir2)
        .arg(&node_modules)
        .output()
        .expect("Failed to execute biome-tailwind-sorter");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(stderr.contains("dir1/a.html"));
    assert!(stderr.contains("dir2/b.html"));
    assert!(!stderr.contains("node_modules/ignored.html"), "node_modules should be ignored even if passed explicitly as a directory to walk");
}
