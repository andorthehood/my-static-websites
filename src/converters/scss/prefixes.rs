/// Configuration for vendor prefix generation
#[derive(Debug, Clone)]
pub struct PrefixConfig {
    /// Whether to add flexbox prefixes (-webkit-, -ms-)
    pub flexbox: bool,
    /// Whether to add user interaction prefixes (-webkit-, -moz-)
    pub user_interaction: bool,
    /// Whether to add effect prefixes like backdrop-filter (-webkit-)
    pub effects: bool,
}

impl Default for PrefixConfig {
    fn default() -> Self {
        Self {
            flexbox: true,
            user_interaction: true,
            effects: true,
        }
    }
}

/// A CSS declaration that might need vendor prefixes
#[derive(Debug, Clone)]
struct Declaration {
    property: String,
    value: String,
}

/// Add vendor prefixes to CSS content based on configuration
pub fn add_vendor_prefixes(css: &str, config: &PrefixConfig) -> String {
    // Use regex-like approach to find and replace declarations within CSS rules
    let mut result = css.to_string();

    // Find all CSS rules and process their declarations
    let mut pos = 0;
    while let Some(open_brace) = result[pos..].find('{') {
        let absolute_open = pos + open_brace;
        if let Some(close_brace) = result[absolute_open..].find('}') {
            let absolute_close = absolute_open + close_brace;

            // Extract the rule content between braces
            let rule_content = &result[absolute_open + 1..absolute_close];
            let prefixed_content = add_prefixes_to_declarations(rule_content, config);

            // Replace the content if it changed
            if prefixed_content != rule_content {
                result.replace_range(absolute_open + 1..absolute_close, &prefixed_content);
                pos = absolute_open + prefixed_content.len() + 1;
            } else {
                pos = absolute_close + 1;
            }
        } else {
            break;
        }
    }

    result
}

/// Add vendor prefixes to declarations within a CSS rule
fn add_prefixes_to_declarations(declarations: &str, config: &PrefixConfig) -> String {
    let mut result = String::new();
    let mut current_decl = String::new();

    for ch in declarations.chars() {
        if ch == ';' {
            current_decl.push(ch);

            // Process this complete declaration
            if let Some(declaration) = parse_declaration_from_text(&current_decl) {
                // Add prefixes before the original declaration
                let prefixes = get_required_prefixes(&declaration, config);
                for prefix in prefixes {
                    // Check if this prefix already exists in the full declarations (more flexible check)
                    let prefix_property = prefix.split(':').next().unwrap_or("").trim();
                    let prefix_value = prefix
                        .split(':')
                        .nth(1)
                        .and_then(|v| v.split(';').next())
                        .unwrap_or("")
                        .trim();
                    let prefix_pattern = format!("{}:{}", prefix_property, prefix_value);

                    if !declarations.contains(&prefix_pattern) && !declarations.contains(&prefix) {
                        result.push_str(&prefix);
                    }
                }
            }

            // Add the original declaration
            result.push_str(&current_decl);
            current_decl.clear();
        } else {
            current_decl.push(ch);
        }
    }

    // Handle any remaining declaration without semicolon
    if !current_decl.trim().is_empty() {
        if let Some(declaration) = parse_declaration_from_text(&current_decl) {
            let prefixes = get_required_prefixes(&declaration, config);
            for prefix in prefixes {
                let prefix_property = prefix.split(':').next().unwrap_or("").trim();
                let prefix_value = prefix
                    .split(':')
                    .nth(1)
                    .and_then(|v| v.split(';').next())
                    .unwrap_or("")
                    .trim();
                let prefix_pattern = format!("{}:{}", prefix_property, prefix_value);

                if !declarations.contains(&prefix_pattern) && !declarations.contains(&prefix) {
                    result.push_str(&prefix);
                }
            }
        }
        result.push_str(&current_decl);
    }

    result
}

/// Parse a CSS declaration from text (without line-based parsing)
fn parse_declaration_from_text(text: &str) -> Option<Declaration> {
    let trimmed = text.trim();

    // Skip empty or non-declaration content
    if trimmed.is_empty() || !trimmed.contains(':') {
        return None;
    }

    // Split on first colon
    let colon_pos = trimmed.find(':')?;
    let property = trimmed[..colon_pos].trim().to_string();
    let value_part = trimmed[colon_pos + 1..].trim();

    // Remove trailing semicolon if present
    let value = if value_part.ends_with(';') {
        value_part[..value_part.len() - 1].trim().to_string()
    } else {
        value_part.to_string()
    };

    Some(Declaration { property, value })
}

/// Get required vendor prefixes for a declaration
fn get_required_prefixes(decl: &Declaration, config: &PrefixConfig) -> Vec<String> {
    let mut prefixes = Vec::new();

    // Flexbox properties
    if config.flexbox {
        match decl.property.as_str() {
            "display" if decl.value == "flex" => {
                prefixes.push(format!("display: -webkit-flex;"));
                prefixes.push(format!("display: -ms-flexbox;"));
            }
            "flex-direction" => {
                prefixes.push(format!("-webkit-flex-direction: {};", decl.value));
                prefixes.push(format!("-ms-flex-direction: {};", decl.value));
            }
            "justify-content" => {
                prefixes.push(format!("-webkit-justify-content: {};", decl.value));
                prefixes.push(format!(
                    "-ms-flex-pack: {};",
                    map_justify_content_to_ms(&decl.value)
                ));
            }
            "align-items" => {
                prefixes.push(format!("-webkit-align-items: {};", decl.value));
                prefixes.push(format!(
                    "-ms-flex-align: {};",
                    map_align_items_to_ms(&decl.value)
                ));
            }
            "flex" => {
                prefixes.push(format!("-webkit-flex: {};", decl.value));
                prefixes.push(format!("-ms-flex: {};", decl.value));
            }
            _ => {}
        }
    }

    // User interaction properties
    if config.user_interaction {
        match decl.property.as_str() {
            "user-select" => {
                prefixes.push(format!("-webkit-user-select: {};", decl.value));
                prefixes.push(format!("-moz-user-select: {};", decl.value));
                prefixes.push(format!("-ms-user-select: {};", decl.value));
            }
            "appearance" => {
                prefixes.push(format!("-webkit-appearance: {};", decl.value));
                prefixes.push(format!("-moz-appearance: {};", decl.value));
            }
            _ => {}
        }
    }

    // Effects properties
    if config.effects {
        match decl.property.as_str() {
            "backdrop-filter" => {
                prefixes.push(format!("-webkit-backdrop-filter: {};", decl.value));
            }
            _ => {}
        }
    }

    prefixes
}

/// Map CSS3 justify-content values to IE10+ -ms-flex-pack equivalents
fn map_justify_content_to_ms(value: &str) -> &str {
    match value {
        "flex-start" => "start",
        "flex-end" => "end",
        "space-between" => "justify",
        "space-around" => "distribute",
        "center" => "center",
        _ => value, // fallback to original
    }
}

/// Map CSS3 align-items values to IE10+ -ms-flex-align equivalents
fn map_align_items_to_ms(value: &str) -> &str {
    match value {
        "flex-start" => "start",
        "flex-end" => "end",
        "center" => "center",
        "stretch" => "stretch",
        "baseline" => "baseline",
        _ => value, // fallback to original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_declaration_simple() {
        let decl = parse_declaration_from_text("  color: red;  ").unwrap();
        assert_eq!(decl.property, "color");
        assert_eq!(decl.value, "red");
    }

    #[test]
    fn test_parse_declaration_no_semicolon() {
        let decl = parse_declaration_from_text("margin: 0").unwrap();
        assert_eq!(decl.property, "margin");
        assert_eq!(decl.value, "0");
    }

    #[test]
    fn test_parse_declaration_skips_non_declarations() {
        assert!(parse_declaration_from_text("}.foo{").is_none());
        assert!(parse_declaration_from_text("@media screen").is_none());
        assert!(parse_declaration_from_text("/* comment */").is_none());
        assert!(parse_declaration_from_text("").is_none());
    }

    #[test]
    fn test_flexbox_display_prefixes() {
        let config = PrefixConfig::default();
        let css = ".test{display:flex;}";
        let result = add_vendor_prefixes(css, &config);

        assert!(result.contains("display:flex;"));
        assert!(result.contains("display: -webkit-flex;"));
        assert!(result.contains("display: -ms-flexbox;"));
    }

    #[test]
    fn test_user_select_prefixes() {
        let config = PrefixConfig::default();
        let css = ".test{user-select:none;}";
        let result = add_vendor_prefixes(css, &config);

        assert!(result.contains("user-select:none;"));
        assert!(result.contains("-webkit-user-select: none;"));
        assert!(result.contains("-moz-user-select: none;"));
        assert!(result.contains("-ms-user-select: none;"));
    }

    #[test]
    fn test_appearance_prefixes() {
        let config = PrefixConfig::default();
        let css = ".test{appearance:none;}";
        let result = add_vendor_prefixes(css, &config);

        assert!(result.contains("appearance:none;"));
        assert!(result.contains("-webkit-appearance: none;"));
        assert!(result.contains("-moz-appearance: none;"));
    }

    #[test]
    fn test_backdrop_filter_prefixes() {
        let config = PrefixConfig::default();
        let css = ".test{backdrop-filter:blur(5px);}";
        let result = add_vendor_prefixes(css, &config);

        assert!(result.contains("backdrop-filter:blur(5px);"));
        assert!(result.contains("-webkit-backdrop-filter: blur(5px);"));
    }

    #[test]
    fn test_configuration_toggles() {
        let mut config = PrefixConfig::default();
        config.flexbox = false;

        let css = ".test{display:flex;}";
        let result = add_vendor_prefixes(css, &config);

        assert!(result.contains("display:flex;"));
        assert!(!result.contains("-webkit-flex"));
        assert!(!result.contains("-ms-flexbox"));
    }

    #[test]
    fn test_no_duplicate_prefixes() {
        let config = PrefixConfig::default();
        let css = ".test{display:-webkit-flex;display:flex;}";
        let result = add_vendor_prefixes(css, &config);

        // Should not add another -webkit-flex since it already exists
        let webkit_count = result.matches("display: -webkit-flex;").count();
        assert_eq!(webkit_count, 0); // Won't be added because -webkit-flex already exists

        // But should still have the original
        assert!(result.contains("display:-webkit-flex;"));
        assert!(result.contains("display:flex;"));
    }

    #[test]
    fn test_justify_content_mapping() {
        assert_eq!(map_justify_content_to_ms("flex-start"), "start");
        assert_eq!(map_justify_content_to_ms("flex-end"), "end");
        assert_eq!(map_justify_content_to_ms("space-between"), "justify");
        assert_eq!(map_justify_content_to_ms("space-around"), "distribute");
        assert_eq!(map_justify_content_to_ms("center"), "center");
    }

    #[test]
    fn test_align_items_mapping() {
        assert_eq!(map_align_items_to_ms("flex-start"), "start");
        assert_eq!(map_align_items_to_ms("flex-end"), "end");
        assert_eq!(map_align_items_to_ms("center"), "center");
        assert_eq!(map_align_items_to_ms("stretch"), "stretch");
        assert_eq!(map_align_items_to_ms("baseline"), "baseline");
    }
}
