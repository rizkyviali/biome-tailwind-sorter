use regex::Regex;

#[derive(Debug, Clone)]
pub struct ClassAttribute {
    pub value: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub is_multiline: bool,
    pub quotes: QuoteType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteType {
    Double,
    Single,
    None,
}

pub fn extract_class_names(class_string: &str) -> Vec<String> {
    class_string
        .trim()
        .split_whitespace()
        .filter(|cls| !cls.is_empty())
        .map(|cls| cls.to_string())
        .collect()
}

pub fn reconstruct_class_string(
    class_names: &[String],
    original_string: &str,
    preserve_multiline: bool,
) -> String {
    if !preserve_multiline || !original_string.contains('\n') {
        return class_names.join(" ");
    }
    
    // For multiline, try to preserve the original formatting structure
    let lines: Vec<&str> = original_string.split('\n').collect();
    if lines.len() <= 1 {
        return class_names.join(" ");
    }
    
    // Simple heuristic: distribute classes evenly across lines
    let classes_per_line = (class_names.len() + lines.len() - 1) / lines.len(); // Ceiling division
    let mut result = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        let line_start = i * classes_per_line;
        let line_end = std::cmp::min((i + 1) * classes_per_line, class_names.len());
        
        if line_start < class_names.len() {
            let line_classes = &class_names[line_start..line_end];
            
            if !line_classes.is_empty() {
                // Extract the indentation from the original line
                let indent = line.chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();
                result.push(format!("{}{}", indent, line_classes.join(" ")));
            }
        }
    }
    
    result.join("\n")
}

pub fn parse_class_attribute(attribute_value: &str, attribute_name: &str) -> Option<ClassAttribute> {
    if !matches!(attribute_name, "class" | "className") {
        return None;
    }
    
    // Determine quote type and clean value
    let (quotes, clean_value) = if attribute_value.starts_with('"') && attribute_value.ends_with('"') {
        (QuoteType::Double, &attribute_value[1..attribute_value.len() - 1])
    } else if attribute_value.starts_with('\'') && attribute_value.ends_with('\'') {
        (QuoteType::Single, &attribute_value[1..attribute_value.len() - 1])
    } else {
        (QuoteType::None, attribute_value)
    };
    
    let is_multiline = clean_value.contains('\n') || clean_value.contains("\\n");
    
    Some(ClassAttribute {
        value: clean_value.to_string(),
        start_pos: 0, // Will be set by the caller
        end_pos: clean_value.len(),
        is_multiline,
        quotes,
    })
}

pub fn is_tailwind_class(class_name: &str) -> bool {
    // Check if it's a known exact Tailwind class first
    let exact_tailwind_classes = [
        "container", "flex", "grid", "block", "inline", "hidden", "visible", "invisible",
        "absolute", "relative", "fixed", "static", "sticky"
    ];
    
    // Remove modifiers to get base class
    let base_class = class_name.split(':').last().unwrap_or("");
    
    if exact_tailwind_classes.contains(&base_class) {
        return true;
    }
    
    // Basic heuristics for Tailwind classes using regex
    let patterns = [
        r"^(p|px|py|pt|pr|pb|pl)-",
        r"^m-\d+",
        r"^(mx|mt|mr|mb|ml)-",
        r"^my-\d+",
        r"^(w|h|min-w|min-h|max-w|max-h)-",
        r"^(text|font|leading|tracking)-",
        r"^(bg|border|rounded)-",
        r"^(top|right|bottom|left|inset)-",
        r"^z-",
        r"^(opacity|shadow)-",
        r"^(hover|focus|active|disabled):",
        r"^(sm|md|lg|xl|2xl):",
        r"^(dark|light):",
        r"^justify-",
        r"^items-",
    ];
    
    patterns.iter().any(|pattern| {
        Regex::new(pattern).unwrap().is_match(class_name)
    })
}

pub fn filter_tailwind_classes(class_names: &[String]) -> Vec<String> {
    class_names
        .iter()
        .filter(|class_name| is_tailwind_class(class_name))
        .cloned()
        .collect()
}

pub fn contains_tailwind_classes(class_names: &[String]) -> bool {
    class_names.iter().any(|class_name| is_tailwind_class(class_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_class_names() {
        let input = "flex items-center justify-between";
        let result = extract_class_names(input);
        assert_eq!(result, vec!["flex", "items-center", "justify-between"]);
    }

    #[test]
    fn test_extract_class_names_with_extra_spaces() {
        let input = "  flex   items-center  justify-between  ";
        let result = extract_class_names(input);
        assert_eq!(result, vec!["flex", "items-center", "justify-between"]);
    }

    #[test]
    fn test_parse_class_attribute_with_quotes() {
        let result = parse_class_attribute("\"flex items-center\"", "class").unwrap();
        assert_eq!(result.value, "flex items-center");
        assert_eq!(result.quotes, QuoteType::Double);
        assert!(!result.is_multiline);
    }

    #[test]
    fn test_parse_class_attribute_single_quotes() {
        let result = parse_class_attribute("'flex items-center'", "className").unwrap();
        assert_eq!(result.value, "flex items-center");
        assert_eq!(result.quotes, QuoteType::Single);
    }

    #[test]
    fn test_is_tailwind_class() {
        assert!(is_tailwind_class("flex"));
        assert!(is_tailwind_class("p-4"));
        assert!(is_tailwind_class("text-red-500"));
        assert!(is_tailwind_class("hover:bg-blue-500"));
        assert!(is_tailwind_class("md:text-lg"));
        assert!(!is_tailwind_class("custom-class"));
        assert!(!is_tailwind_class("my-component"));
    }

    #[test]
    fn test_contains_tailwind_classes() {
        let classes = vec![
            "custom-class".to_string(),
            "flex".to_string(),
            "another-custom".to_string(),
        ];
        assert!(contains_tailwind_classes(&classes));
        
        let no_tailwind = vec![
            "custom-class".to_string(),
            "another-custom".to_string(),
        ];
        assert!(!contains_tailwind_classes(&no_tailwind));
    }

    #[test]
    fn test_reconstruct_class_string_single_line() {
        let classes = vec!["flex".to_string(), "items-center".to_string()];
        let original = "items-center flex";
        let result = reconstruct_class_string(&classes, original, true);
        assert_eq!(result, "flex items-center");
    }

    #[test]
    fn test_reconstruct_class_string_multiline() {
        let classes = vec![
            "flex".to_string(),
            "items-center".to_string(),
            "justify-between".to_string(),
            "p-4".to_string(),
        ];
        let original = "  items-center\n  justify-between\n  flex\n  p-4";
        let result = reconstruct_class_string(&classes, original, true);
        
        // Should preserve indentation and distribute classes
        assert!(result.contains("  flex"));
        assert!(result.contains('\n'));
    }
}