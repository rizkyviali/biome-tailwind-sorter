use phf::phf_map;
use std::cmp::Ordering;

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
    
    // Flex and Grid
    "flex-row" => 70,
    "flex-row-reverse" => 70,
    "flex-col" => 70,
    "flex-col-reverse" => 70,
    "flex-wrap" => 70,
    "flex-wrap-reverse" => 70,
    "flex-nowrap" => 70,
    "place-content" => 70,
    "place-items" => 70,
    "align-content" => 70,
    "align-items" => 70,
    "align-self" => 70,
    "justify-content" => 70,
    "justify-items" => 70,
    "justify-self" => 70,
    "flex-auto" => 70,
    "flex-initial" => 70,
    "flex-none" => 70,
    "grow" => 70,
    "shrink" => 70,
    "order" => 70,
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
    
    // Typography
    "font-family" => 100,
    "font-size" => 100,
    "font-smoothing" => 100,
    "font-style" => 100,
    "font-weight" => 100,
    "font-variant-numeric" => 100,
    "letter-spacing" => 100,
    "line-height" => 100,
    "list-style-image" => 100,
    "list-style-position" => 100,
    "list-style-type" => 100,
    "text-align" => 100,
    "text-color" => 100,
    "text-decoration" => 100,
    "text-decoration-color" => 100,
    "text-decoration-style" => 100,
    "text-decoration-thickness" => 100,
    "text-underline-offset" => 100,
    "text-transform" => 100,
    "text-overflow" => 100,
    "vertical-align" => 100,
    "whitespace" => 100,
    "word-break" => 100,
    "hyphens" => 100,
    "content" => 100,
    
    // Backgrounds
    "bg" => 110,
    "from" => 110,
    "via" => 110,
    "to" => 110,
    "bg-attachment" => 110,
    "bg-clip" => 110,
    "bg-origin" => 110,
    "bg-position" => 110,
    "bg-repeat" => 110,
    "bg-size" => 110,
    "bg-image" => 110,
    
    // Borders
    "border" => 120,
    "border-collapse" => 120,
    "border-spacing" => 120,
    "table-layout" => 120,
    "border-style" => 120,
    "divide" => 120,
    "outline" => 120,
    "ring" => 120,
    
    // Effects
    "shadow" => 130,
    "opacity" => 130,
    "mix-blend-mode" => 130,
    "bg-blend-mode" => 130,
    
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
    "backdrop-filter" => 140,
    "backdrop-blur" => 140,
    "backdrop-brightness" => 140,
    "backdrop-contrast" => 140,
    "backdrop-grayscale" => 140,
    "backdrop-hue-rotate" => 140,
    "backdrop-invert" => 140,
    "backdrop-opacity" => 140,
    "backdrop-saturate" => 140,
    "backdrop-sepia" => 140,
    
    // Tables
    "caption-side" => 150,
    "empty-cells" => 150,
    
    // Transitions and Animation
    "transition" => 160,
    "duration" => 160,
    "ease" => 160,
    "delay" => 160,
    "animate" => 160,
    
    // Transforms
    "transform" => 170,
    "transform-origin" => 170,
    "scale" => 170,
    "rotate" => 170,
    "translate" => 170,
    "skew" => 170,
    
    // Interactivity
    "accent-color" => 180,
    "appearance" => 180,
    "cursor" => 180,
    "caret-color" => 180,
    "pointer-events" => 180,
    "resize" => 180,
    "scroll-behavior" => 180,
    "scroll-margin" => 180,
    "scroll-padding" => 180,
    "scroll-snap-align" => 180,
    "scroll-snap-stop" => 180,
    "scroll-snap-type" => 180,
    "touch-action" => 180,
    "user-select" => 180,
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

pub fn parse_tailwind_class(class_name: &str) -> TailwindClass {
    let parts: Vec<&str> = class_name.split(':').collect();
    let actual_class = parts[parts.len() - 1];
    let modifiers = if parts.len() > 1 {
        Some(parts[..parts.len() - 1].join(":"))
    } else {
        None
    };
    
    // Handle arbitrary values like w-[100px]
    let base_class = regex::Regex::new(r"\[.*?\]$")
        .unwrap()
        .replace(actual_class, "");
    
    // Handle numeric suffixes like p-4, text-lg, etc.
    let base_pattern = regex::Regex::new(r"-\d+(\.\d+)?$")
        .unwrap()
        .replace(&base_class, "");
    let base_pattern = regex::Regex::new(r"-[a-z]+$")
        .unwrap()
        .replace(&base_pattern, "");
    
    let order = get_class_order(&base_pattern, actual_class);
    
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
    
    // Check base pattern
    if let Some(&order) = TAILWIND_ORDER_MAP.get(base_pattern) {
        return order;
    }
    
    // Check for common patterns
    for (pattern, &order) in TAILWIND_ORDER_MAP.entries() {
        if full_class.starts_with(&format!("{}-", pattern)) || full_class == *pattern {
            return order;
        }
    }
    
    // Handle special cases
    if full_class.starts_with("text-") {
        if regex::Regex::new(r"^text-(xs|sm|base|lg|xl|\d+xl)$")
            .unwrap()
            .is_match(full_class)
        {
            return 100; // font-size
        }
        if regex::Regex::new(r"^text-(left|center|right|justify|start|end)$")
            .unwrap()
            .is_match(full_class)
        {
            return 100; // text-align
        }
        return 100; // text-color by default
    }
    
    if full_class.starts_with("bg-") {
        return 110;
    }
    
    if full_class.starts_with("border-") {
        return 120;
    }
    
    if full_class.starts_with("rounded") {
        return 120;
    }
    
    if full_class.starts_with("shadow") {
        return 130;
    }
    
    if full_class.starts_with("font-") {
        return 100;
    }
    
    if full_class.starts_with("transition") {
        return 160;
    }
    
    if full_class.starts_with("duration") {
        return 160;
    }
    
    // Default to high number for unknown classes
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

pub fn are_classes_sorted(classes: &[String]) -> bool {
    let sorted = sort_tailwind_classes(classes);
    classes == sorted
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