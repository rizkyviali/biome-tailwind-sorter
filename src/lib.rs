pub mod class_extractor;
pub mod config;
pub mod formatter;
pub mod tailwind_order;

pub use class_extractor::{
    contains_tailwind_classes, extract_class_names, is_tailwind_class, reconstruct_class_string,
};
pub use config::Config;
pub use formatter::{CursorPosition, FormatResult, TailwindFormatter};
pub use tailwind_order::{parse_tailwind_class, sort_tailwind_classes, TailwindClass};

// Re-export the main formatting function for convenience
pub fn format_document(source: &str) -> String {
    formatter::format_document(source)
}
