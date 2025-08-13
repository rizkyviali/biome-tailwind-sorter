use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Custom class order (overrides default Tailwind order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_order: Option<Vec<String>>,
    
    /// File extensions to process
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,
    
    /// Whether to preserve multiline formatting
    #[serde(default = "default_preserve_multiline")]
    pub preserve_multiline: bool,
    
    /// Directories/files to ignore
    #[serde(default)]
    pub ignore: Vec<String>,
    
    /// Whether to process files recursively in directories
    #[serde(default = "default_recursive")]
    pub recursive: bool,
    
    /// Maximum file size to process (in bytes)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: usize,
    
    /// Additional custom classes that should be treated as Tailwind classes
    #[serde(default)]
    pub custom_classes: Vec<String>,
    
    /// Whether to sort custom classes
    #[serde(default = "default_sort_custom_classes")]
    pub sort_custom_classes: bool,
}

fn default_extensions() -> Vec<String> {
    vec![
        "html".to_string(),
        "js".to_string(),
        "jsx".to_string(),
        "ts".to_string(),
        "tsx".to_string(),
        "vue".to_string(),
        "astro".to_string(),
    ]
}

fn default_preserve_multiline() -> bool {
    true
}

fn default_recursive() -> bool {
    true
}

fn default_max_file_size() -> usize {
    10 * 1024 * 1024 // 10MB
}

fn default_sort_custom_classes() -> bool {
    false
}

impl Default for Config {
    fn default() -> Self {
        Self {
            custom_order: None,
            extensions: default_extensions(),
            preserve_multiline: default_preserve_multiline(),
            ignore: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
                "target".to_string(),
            ],
            recursive: default_recursive(),
            max_file_size: default_max_file_size(),
            custom_classes: Vec::new(),
            sort_custom_classes: default_sort_custom_classes(),
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        
        // Try JSON first, then TOML
        if let Ok(config) = serde_json::from_str::<Config>(&content) {
            Ok(config)
        } else {
            // For future TOML support
            Err("Only JSON configuration is currently supported".into())
        }
    }
    
    /// Find and load configuration from common locations
    pub fn load() -> Config {
        let config_files = [
            ".tailwindsorterrc",
            ".tailwindsorterrc.json",
            "tailwindsorter.config.json",
            "biome-tailwind-sorter.json",
        ];
        
        for config_file in &config_files {
            if let Ok(config) = Self::load_from_file(config_file) {
                return config;
            }
        }
        
        // Try package.json
        if let Ok(package_json) = fs::read_to_string("package.json") {
            if let Ok(package_data) = serde_json::from_str::<serde_json::Value>(&package_json) {
                if let Some(config_data) = package_data.get("biome-tailwind-sorter") {
                    if let Ok(config) = serde_json::from_value::<Config>(config_data.clone()) {
                        return config;
                    }
                }
            }
        }
        
        Config::default()
    }
    
    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// Check if a file should be processed based on extension
    pub fn should_process_file(&self, file_path: &str) -> bool {
        if let Some(extension) = Path::new(file_path).extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.extensions.iter().any(|ext| ext == ext_str);
            }
        }
        false
    }
    
    /// Check if a path should be ignored
    pub fn should_ignore_path(&self, path: &str) -> bool {
        self.ignore.iter().any(|ignore_pattern| {
            // Simple pattern matching - could be enhanced with glob patterns
            path.contains(ignore_pattern)
        })
    }
    
    /// Get the custom class order if defined
    pub fn get_custom_order(&self) -> Option<&[String]> {
        self.custom_order.as_deref()
    }
    
    /// Check if a class is considered a custom class
    pub fn is_custom_class(&self, class_name: &str) -> bool {
        self.custom_classes.iter().any(|custom| custom == class_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.extensions.contains(&"html".to_string()));
        assert!(config.extensions.contains(&"tsx".to_string()));
        assert_eq!(config.preserve_multiline, true);
        assert_eq!(config.recursive, true);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.extensions, deserialized.extensions);
        assert_eq!(config.preserve_multiline, deserialized.preserve_multiline);
    }

    #[test]
    fn test_load_config_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"{{
                "extensions": ["html", "jsx"],
                "preserve_multiline": false,
                "custom_classes": ["my-custom-class"]
            }}"#
        ).unwrap();
        
        let config = Config::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.extensions, vec!["html", "jsx"]);
        assert_eq!(config.preserve_multiline, false);
        assert_eq!(config.custom_classes, vec!["my-custom-class"]);
    }

    #[test]
    fn test_should_process_file() {
        let config = Config::default();
        assert!(config.should_process_file("test.html"));
        assert!(config.should_process_file("component.tsx"));
        assert!(!config.should_process_file("styles.css"));
        assert!(!config.should_process_file("README.md"));
    }

    #[test]
    fn test_should_ignore_path() {
        let config = Config::default();
        assert!(config.should_ignore_path("node_modules/package/file.js"));
        assert!(config.should_ignore_path("dist/bundle.js"));
        assert!(!config.should_ignore_path("src/components/Button.tsx"));
    }

    #[test]
    fn test_is_custom_class() {
        let mut config = Config::default();
        config.custom_classes = vec!["my-custom".to_string(), "another-custom".to_string()];
        
        assert!(config.is_custom_class("my-custom"));
        assert!(config.is_custom_class("another-custom"));
        assert!(!config.is_custom_class("text-red-500"));
    }
}