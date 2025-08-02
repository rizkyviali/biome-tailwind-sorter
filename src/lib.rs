pub mod tailwind_order;
pub mod class_extractor;
pub mod formatter;

pub use formatter::{TailwindFormatter, CursorPosition, FormatResult};
pub use tailwind_order::{sort_tailwind_classes, are_classes_sorted, parse_tailwind_class, TailwindClass};
pub use class_extractor::{
    extract_class_names, 
    reconstruct_class_string, 
    contains_tailwind_classes, 
    is_tailwind_class,
    filter_tailwind_classes,
    ClassAttribute,
    QuoteType
};

// Re-export the main formatting function for convenience
pub fn format_document(source: &str) -> String {
    formatter::format_document(source)
}