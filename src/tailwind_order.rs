use phf::phf_map;
use std::cmp::Ordering;
use std::sync::LazyLock;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct TailwindClass {
    pub name: String,
    pub order: u32,
    pub modifier: Option<String>,
}

static TAILWIND_ORDER_MAP: phf::Map<&'static str, u32> = phf_map! {
    // Layout
    "container" => 0,
    "box-border" => 10,
    "box-content" => 10,
    "block" => 20,
    "inline-block" => 20,
    "inline" => 20,
    "flex" => 20,
    "inline-flex" => 20,
    "table" => 20,
    "inline-table" => 20,
    "table-caption" => 20,
    "table-cell" => 20,
    "table-column" => 20,
    "table-column-group" => 20,
    "table-footer-group" => 20,
    "table-header-group" => 20,
    "table-row-group" => 20,
    "table-row" => 20,
    "flow-root" => 20,
    "grid" => 20,
    "inline-grid" => 20,
    "contents" => 20,
    "list-item" => 20,
    "hidden" => 20,

    // Overflow
    "overflow" => 25,
    "overscroll" => 25,

    // Position
    "static" => 30,
    "fixed" => 30,
    "absolute" => 30,
    "relative" => 30,
    "sticky" => 30,

    // Top/Right/Bottom/Left
    "inset" => 40,
    "top" => 40,
    "right" => 40,
    "bottom" => 40,
    "left" => 40,

    // Visibility
    "visible" => 50,
    "invisible" => 50,
    "collapse" => 50,

    // Z-Index
    "z" => 60,

    // Flex and Grid — using actual Tailwind class prefixes
    "flex-row" => 70,
    "flex-row-reverse" => 70,
    "flex-col" => 70,
    "flex-col-reverse" => 70,
    "flex-wrap" => 70,
    "flex-wrap-reverse" => 70,
    "flex-nowrap" => 70,
    "flex-1" => 70,
    "flex-auto" => 70,
    "flex-initial" => 70,
    "flex-none" => 70,
    "basis" => 70,
    "grow" => 70,
    "shrink" => 70,
    "order" => 70,
    // Alignment (actual Tailwind prefixes, not CSS property names)
    "justify" => 70,
    "justify-items" => 70,
    "justify-self" => 70,
    "items" => 70,
    "self" => 70,
    "place-content" => 70,
    "place-items" => 70,
    "place-self" => 70,
    "content" => 70,
    // Grid
    "grid-cols" => 70,
    "col-auto" => 70,
    "col-span" => 70,
    "col-start" => 70,
    "col-end" => 70,
    "grid-rows" => 70,
    "row-auto" => 70,
    "row-span" => 70,
    "row-start" => 70,
    "row-end" => 70,
    "gap" => 70,

    // Spacing
    "p" => 80,
    "px" => 80,
    "py" => 80,
    "pt" => 80,
    "pr" => 80,
    "pb" => 80,
    "pl" => 80,
    "m" => 80,
    "mx" => 80,
    "my" => 80,
    "mt" => 80,
    "mr" => 80,
    "mb" => 80,
    "ml" => 80,
    "space-x" => 80,
    "space-y" => 80,

    // Sizing
    "w" => 90,
    "min-w" => 90,
    "max-w" => 90,
    "h" => 90,
    "min-h" => 90,
    "max-h" => 90,
    "size" => 90,

    // Typography — using actual Tailwind class prefixes
    "font" => 100,
    "text" => 100,
    "leading" => 100,
    "tracking" => 100,
    "list" => 100,
    "decoration" => 100,
    "underline" => 100,
    "overline" => 100,
    "line-through" => 100,
    "no-underline" => 100,
    "antialiased" => 100,
    "subpixel-antialiased" => 100,
    "italic" => 100,
    "not-italic" => 100,
    "uppercase" => 100,
    "lowercase" => 100,
    "capitalize" => 100,
    "normal-case" => 100,
    "truncate" => 100,
    "whitespace" => 100,
    "break" => 100,
    "hyphens" => 100,
    "indent" => 100,
    "align" => 100,

    // Backgrounds
    "bg" => 110,
    "from" => 110,
    "via" => 110,
    "to" => 110,

    // Borders
    "border" => 120,
    "divide" => 120,
    "outline" => 120,
    "ring" => 120,
    "rounded" => 120,

    // Effects
    "shadow" => 130,
    "opacity" => 130,
    "mix-blend" => 130,
    "bg-blend" => 130,

    // Filters
    "filter" => 140,
    "blur" => 140,
    "brightness" => 140,
    "contrast" => 140,
    "drop-shadow" => 140,
    "grayscale" => 140,
    "hue-rotate" => 140,
    "invert" => 140,
    "saturate" => 140,
    "sepia" => 140,
    "backdrop" => 140,

    // Transitions and Animation
    "transition" => 160,
    "duration" => 160,
    "ease" => 160,
    "delay" => 160,
    "animate" => 160,

    // Transforms
    "transform" => 170,
    "origin" => 170,
    "scale" => 170,
    "rotate" => 170,
    "translate" => 170,
    "skew" => 170,

    // Interactivity
    "accent" => 180,
    "appearance" => 180,
    "cursor" => 180,
    "caret" => 180,
    "pointer-events" => 180,
    "resize" => 180,
    "scroll" => 180,
    "snap" => 180,
    "touch" => 180,
    "select" => 180,
    "will-change" => 180,

    // SVG
    "fill" => 190,
    "stroke" => 190,

    // Accessibility
    "sr-only" => 200,
    "not-sr-only" => 200,

    // Official
    "forced-color-adjust" => 210,
};

static ARBITRARY_VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[.*?\]$").unwrap()
});
static NUMERIC_SUFFIX_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"-\d+(\.\d+)?$").unwrap()
});
static ALPHA_SUFFIX_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"-[a-z]+$").unwrap()
});

pub fn parse_tailwind_class(class_name: &str) -> TailwindClass {
    let parts: Vec<&str> = class_name.split(':').collect();
    let actual_class = parts[parts.len() - 1];
    let modifiers = if parts.len() > 1 {
        Some(parts[..parts.len() - 1].join(":"))
    } else {
        None
    };

    // Strip arbitrary values like w-[100px]
    let base_class = ARBITRARY_VALUE_REGEX.replace(actual_class, "");
    // Strip numeric suffix like -4, -500
    let base_pattern = NUMERIC_SUFFIX_REGEX.replace(base_class.as_ref(), "");
    // Strip alphabetic suffix like -red, -lg
    let base_pattern = ALPHA_SUFFIX_REGEX.replace(base_pattern.as_ref(), "");

    let order = get_class_order(base_pattern.as_ref(), actual_class);

    TailwindClass {
        name: class_name.to_string(),
        order,
        modifier: modifiers,
    }
}

fn get_class_order(base_pattern: &str, full_class: &str) -> u32 {
    // Check exact match first
    if let Some(&order) = TAILWIND_ORDER_MAP.get(full_class) {
        return order;
    }

    // flex-{number} (e.g. flex-1, flex-2) is a flex-shorthand utility, not display:flex
    if full_class.starts_with("flex-") && base_pattern == "flex" {
        let suffix = &full_class[5..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return 70;
        }
    }

    // Check base pattern in the map
    if let Some(&order) = TAILWIND_ORDER_MAP.get(base_pattern) {
        return order;
    }

    // Prefix-based fallback scan (handles multi-segment prefixes like "grid-cols")
    for (pattern, &order) in TAILWIND_ORDER_MAP.entries() {
        if full_class.starts_with(&format!("{pattern}-")) || full_class == *pattern {
            return order;
        }
    }

    // Default to high number for unknown / custom classes
    999
}

fn get_responsive_order(modifier: &str) -> Option<usize> {
    let responsive_order = ["sm", "md", "lg", "xl", "2xl"];
    responsive_order.iter().position(|&x| x == modifier)
}

pub fn sort_tailwind_classes(classes: &[String]) -> Vec<String> {
    let mut parsed_classes: Vec<TailwindClass> = classes
        .iter()
        .map(|c| parse_tailwind_class(c))
        .collect();
    
    parsed_classes.sort_by(|a, b| {
        // First sort by order
        match a.order.cmp(&b.order) {
            Ordering::Equal => {
                // Then by modifier (responsive, pseudo-classes, etc.)
                let a_modifier = a.modifier.as_deref().unwrap_or("");
                let b_modifier = b.modifier.as_deref().unwrap_or("");
                
                match (a_modifier.is_empty(), b_modifier.is_empty()) {
                    (true, false) => Ordering::Less,   // Base classes first
                    (false, true) => Ordering::Greater,
                    (true, true) | (false, false) => {
                        // Sort responsive modifiers in order: sm, md, lg, xl, 2xl
                        match (get_responsive_order(a_modifier), get_responsive_order(b_modifier)) {
                            (Some(a_resp), Some(b_resp)) => a_resp.cmp(&b_resp),
                            _ => a_modifier.cmp(b_modifier),
                        }
                        .then_with(|| a.name.cmp(&b.name))
                    }
                }
            }
            other => other,
        }
    });
    
    parsed_classes.into_iter().map(|c| c.name).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tailwind_class() {
        let class = parse_tailwind_class("md:bg-blue-500");
        assert_eq!(class.name, "md:bg-blue-500");
        assert_eq!(class.modifier, Some("md".to_string()));
        assert_eq!(class.order, 110);
    }

    #[test]
    fn test_sort_simple_classes() {
        let classes = vec![
            "text-red-500".to_string(),
            "p-4".to_string(),
            "flex".to_string(),
        ];
        let sorted = sort_tailwind_classes(&classes);
        assert_eq!(sorted, vec!["flex", "p-4", "text-red-500"]);
    }

    #[test]
    fn test_sort_with_modifiers() {
        let classes = vec![
            "lg:text-red-500".to_string(),
            "md:text-red-500".to_string(),
            "text-red-500".to_string(),
        ];
        let sorted = sort_tailwind_classes(&classes);
        assert_eq!(sorted, vec!["text-red-500", "md:text-red-500", "lg:text-red-500"]);
    }
}