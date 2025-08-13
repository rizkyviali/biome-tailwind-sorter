pub mod tailwind_order;
pub mod class_extractor;
pub mod formatter;
pub mod config;

pub use formatter::{TailwindFormatter, CursorPosition, FormatResult};
pub use tailwind_order::{sort_tailwind_classes, parse_tailwind_class, TailwindClass};
pub use class_extractor::{
    extract_class_names, 
    reconstruct_class_string, 
    contains_tailwind_classes, 
    is_tailwind_class,
    ClassAttribute,
    QuoteType
};
pub use config::Config;

// Re-export the main formatting function for convenience
pub fn format_document(source: &str) -> String {
    formatter::format_document(source)
}