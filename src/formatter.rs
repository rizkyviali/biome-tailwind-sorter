use crate::class_extractor::{
    extract_class_names, reconstruct_class_string, contains_tailwind_classes,
};
use crate::tailwind_order::sort_tailwind_classes;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct FormatResult {
    pub content: String,
    pub cursor_position: Option<CursorPosition>,
    pub changed: bool,
}

#[derive(Debug, Clone)]
pub struct ClassMatch {
    pub start: usize,
    pub end: usize,
    pub prefix: String,
    pub classes: String,
    pub suffix: String,
    pub _line_start: usize,
    pub _line_end: usize,
}

pub struct TailwindFormatter {
    preserve_cursor: bool,
}

impl TailwindFormatter {
    pub fn new(preserve_cursor: bool) -> Self {
        Self { preserve_cursor }
    }

    pub fn format_document(&self, source: &str, cursor_pos: Option<CursorPosition>) -> FormatResult {
        static DOUBLE_QUOTE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"(class(?:Name)?=")([^"]*?)""#).unwrap()
        });
        static SINGLE_QUOTE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"(class(?:Name)?=')([^']*?)'"#).unwrap()
        });
        
        let double_quote_regex = &*DOUBLE_QUOTE_REGEX;
        let single_quote_regex = &*SINGLE_QUOTE_REGEX;
        let mut result = source.to_string();
        let mut offset_adjustment = 0i32;
        let mut changed = false;
        let mut new_cursor_pos = cursor_pos.clone();

        // Find all class matches and collect them
        let mut matches: Vec<ClassMatch> = Vec::new();
        
        // Find double-quoted class attributes
        for m in double_quote_regex.find_iter(source) {
            if let Some(captures) = double_quote_regex.captures(m.as_str()) {
                let prefix = captures.get(1).unwrap().as_str();
                let classes = captures.get(2).unwrap().as_str();

                matches.push(ClassMatch {
                    start: m.start(),
                    end: m.end(),
                    prefix: prefix.to_string(),
                    classes: classes.to_string(),
                    suffix: "\"".to_string(),
                    _line_start: self.get_line_from_offset(source, m.start()),
                    _line_end: self.get_line_from_offset(source, m.end()),
                });
            }
        }
        
        // Find single-quoted class attributes
        for m in single_quote_regex.find_iter(source) {
            if let Some(captures) = single_quote_regex.captures(m.as_str()) {
                let prefix = captures.get(1).unwrap().as_str();
                let classes = captures.get(2).unwrap().as_str();

                matches.push(ClassMatch {
                    start: m.start(),
                    end: m.end(),
                    prefix: prefix.to_string(),
                    classes: classes.to_string(),
                    suffix: "'".to_string(),
                    _line_start: self.get_line_from_offset(source, m.start()),
                    _line_end: self.get_line_from_offset(source, m.end()),
                });
            }
        }
        
        // Sort matches by start position
        matches.sort_by(|a, b| a.start.cmp(&b.start));

        // Process matches in reverse order to maintain correct offsets
        for class_match in matches.into_iter().rev() {
            let class_names = extract_class_names(&class_match.classes);
            
            if !contains_tailwind_classes(&class_names) {
                continue;
            }

            let sorted_classes = sort_tailwind_classes(&class_names);
            
            // Check if sorting is needed
            if class_names == sorted_classes {
                continue;
            }

            let sorted_class_string = reconstruct_class_string(
                &sorted_classes,
                &class_match.classes,
                class_match.classes.contains('\n')
            );

            let new_attribute = format!(
                "{}{}{}",
                class_match.prefix,
                sorted_class_string,
                class_match.suffix
            );

            // Calculate the difference in length
            let old_length = class_match.end - class_match.start;
            let new_length = new_attribute.len();
            let length_diff = new_length as i32 - old_length as i32;

            // Adjust cursor position if needed
            if let (Some(cursor), true) = (&mut new_cursor_pos, self.preserve_cursor) {
                cursor.offset = self.adjust_cursor_offset(
                    cursor.offset,
                    class_match.start,
                    class_match.end,
                    &class_match.classes,
                    &sorted_class_string,
                    offset_adjustment,
                );
            }

            // Replace the text
            let adjusted_start = (class_match.start as i32 + offset_adjustment) as usize;
            let adjusted_end = (class_match.end as i32 + offset_adjustment) as usize;
            
            result.replace_range(adjusted_start..adjusted_end, &new_attribute);
            
            offset_adjustment += length_diff;
            changed = true;
        }

        // Recalculate line and column from offset if cursor was adjusted
        if let Some(cursor) = &mut new_cursor_pos {
            let (line, column) = self.get_line_column_from_offset(&result, cursor.offset);
            cursor.line = line;
            cursor.column = column;
        }

        FormatResult {
            content: result,
            cursor_position: new_cursor_pos,
            changed,
        }
    }

    fn adjust_cursor_offset(
        &self,
        cursor_offset: usize,
        match_start: usize,
        match_end: usize,
        original_classes: &str,
        sorted_classes: &str,
        current_adjustment: i32,
    ) -> usize {
        let adjusted_match_start = (match_start as i32 + current_adjustment) as usize;
        let adjusted_match_end = (match_end as i32 + current_adjustment) as usize;

        // If cursor is before this match, no adjustment needed
        if cursor_offset < adjusted_match_start {
            return cursor_offset;
        }

        // If cursor is after this match, adjust by the length difference
        if cursor_offset >= adjusted_match_end {
            let length_diff = sorted_classes.len() as i32 - original_classes.len() as i32;
            return (cursor_offset as i32 + length_diff).max(0) as usize;
        }

        // Cursor is within the match - try to preserve relative position
        let relative_pos = cursor_offset - adjusted_match_start;
        let class_content_start = original_classes.find(|c: char| !c.is_whitespace()).unwrap_or(0);
        
        // If cursor is in the prefix (class= part), keep it there
        if relative_pos < class_content_start {
            return cursor_offset;
        }

        // Try to map cursor position within the class content
        let original_class_names = extract_class_names(original_classes);
        let sorted_class_names = sort_tailwind_classes(&original_class_names);
        
        // Find which class the cursor was closest to
        let cursor_class_index = self.find_closest_class_index(
            &original_class_names,
            original_classes,
            relative_pos - class_content_start,
        );

        if let Some(class_index) = cursor_class_index {
            // Find the position of this class in the sorted string
            if let Some(target_class) = original_class_names.get(class_index) {
                if let Some(new_pos) = self.find_class_position_in_sorted(
                    target_class,
                    &sorted_class_names,
                    sorted_classes,
                ) {
                    return adjusted_match_start + class_content_start + new_pos;
                }
            }
        }

        // Fallback: place cursor at the start of the sorted classes
        adjusted_match_start + class_content_start
    }

    fn find_closest_class_index(
        &self,
        class_names: &[String],
        class_string: &str,
        cursor_pos: usize,
    ) -> Option<usize> {
        let mut current_pos = 0;
        
        for (index, class_name) in class_names.iter().enumerate() {
            if let Some(class_start) = class_string[current_pos..].find(class_name) {
                let absolute_start = current_pos + class_start;
                let absolute_end = absolute_start + class_name.len();
                
                if cursor_pos >= absolute_start && cursor_pos <= absolute_end {
                    return Some(index);
                }
                
                current_pos = absolute_end;
            }
        }
        
        None
    }

    fn find_class_position_in_sorted(
        &self,
        target_class: &str,
        sorted_classes: &[String],
        _sorted_string: &str,
    ) -> Option<usize> {
        if let Some(index) = sorted_classes.iter().position(|c| c == target_class) {
            let mut pos = 0;
            for (i, class) in sorted_classes.iter().enumerate() {
                if i == index {
                    return Some(pos);
                }
                pos += class.len();
                if i < sorted_classes.len() - 1 {
                    // Account for space separator
                    pos += 1;
                }
            }
        }
        None
    }

    fn get_line_from_offset(&self, content: &str, offset: usize) -> usize {
        content[..offset.min(content.len())].lines().count().saturating_sub(1)
    }

    fn get_line_column_from_offset(&self, content: &str, offset: usize) -> (usize, usize) {
        let offset = offset.min(content.len());
        let lines: Vec<&str> = content[..offset].lines().collect();
        let line = lines.len().saturating_sub(1);
        let column = lines.last().map(|l| l.len()).unwrap_or(0);
        (line, column)
    }
}

// Convenience function for basic formatting without cursor preservation
#[allow(dead_code)]
pub fn format_document(source: &str) -> String {
    let formatter = TailwindFormatter::new(false);
    formatter.format_document(source, None).content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_simple_classes() {
        let input = r#"<div class="text-red-500 p-4 flex">content</div>"#;
        let result = format_document(input);
        assert!(result.contains(r#"class="flex p-4 text-red-500""#));
    }

    #[test]
    fn test_format_with_cursor_preservation() {
        let formatter = TailwindFormatter::new(true);
        let input = r#"<div class="text-red-500 p-4 flex">content</div>"#;
        let cursor = CursorPosition {
            line: 0,
            column: 25, // In the middle of the class attribute
            offset: 25,
        };
        
        let result = formatter.format_document(input, Some(cursor));
        assert!(result.changed);
        assert!(result.cursor_position.is_some());
    }

    #[test]
    fn test_no_change_when_already_sorted() {
        let input = r#"<div class="flex p-4 text-red-500">content</div>"#;
        let formatter = TailwindFormatter::new(false);
        let result = formatter.format_document(input, None);
        assert!(!result.changed);
    }

    #[test]
    fn test_multiline_classes() {
        let input = r#"<div class="
            text-red-500 
            p-4 
            flex
        ">content</div>"#;
        let result = format_document(input);
        assert!(result.contains("flex"));
        assert!(result.contains("p-4"));
        assert!(result.contains("text-red-500"));
    }

    #[test]
    fn test_non_tailwind_classes_ignored() {
        let input = r#"<div class="custom-class my-component">content</div>"#;
        let result = format_document(input);
        assert_eq!(input, result); // Should remain unchanged
    }
}