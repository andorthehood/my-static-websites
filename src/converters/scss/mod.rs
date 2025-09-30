use std::collections::HashSet;
use std::path::{Path, PathBuf};
mod imports;
mod nesting;
mod prefixes;

/// Very primitive SCSS to CSS converter that only supports inlining of `@use` and `@import`.
/// - Only local relative imports ("./" or "../") are supported.
/// - Supports optional quotes and optional trailing semicolon.
/// - Ignores media queries or import options; just inlines raw content.
/// - Prevents infinite recursion by tracking visited absolute paths.
/// - Does NOT process variables, mixins, etc.
/// - Adds minimal support for flattening simple nested selectors like `.foo { .bar { ... } }` → `.foo .bar { ... }`.
/// - Adds vendor prefixes for better browser compatibility (flexbox, user-select, appearance, etc.).
pub fn scss_to_css_with_inline_imports(entry_path: &Path) -> std::io::Result<String> {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut inlined = String::new();
    imports::inline_scss_file(entry_path, &mut visited, &mut inlined)?;
    let flattened = nesting::flatten_basic_nesting(&inlined);

    // Add vendor prefixes for better browser compatibility
    let prefix_config = prefixes::PrefixConfig::default();
    let prefixed = prefixes::add_vendor_prefixes(&flattened, &prefix_config);

    Ok(prefixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_real_style_scss_contains_expected_rules() {
        let path = Path::new("sites/lepkef.ing/assets/style.scss");
        let css = scss_to_css_with_inline_imports(path).expect("convert scss");
        println!("FLATTENED_LEN={}", css.len());
        if let Some(pos) = css.find("article") {
            println!(
                "AROUND_ARTICLE>>>{}<<<",
                &css[pos.saturating_sub(80)..(pos + 200).min(css.len())]
            );
        }
        if let Some(pos) = css.find("p{") {
            println!(
                "AROUND_P>>>{}<<<",
                &css[pos.saturating_sub(80)..(pos + 120).min(css.len())]
            );
        }
        assert!(css.contains("article img.loaded{background: initial;"));
        assert!(css.contains("p{margin: 0;"));
    }

    #[test]
    fn test_scss_to_css_adds_vendor_prefixes() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let test_scss = temp_dir.path().join("test.scss");

        // Create SCSS content with flexbox and other prefixable properties
        let scss_content = r#"
.container {
    display: flex;
    flex-direction: column;
    user-select: none;
    appearance: none;
}

.effects {
    backdrop-filter: blur(5px);
}
"#;
        fs::write(&test_scss, scss_content).unwrap();

        let result = scss_to_css_with_inline_imports(&test_scss).expect("convert scss");

        // Verify vendor prefixes were added
        assert!(result.contains("display: -webkit-flex;"));
        assert!(result.contains("display: -ms-flexbox;"));
        assert!(result.contains("display:flex;") || result.contains("display: flex;"));

        assert!(result.contains("-webkit-user-select: none;"));
        assert!(result.contains("-moz-user-select: none;"));
        assert!(result.contains("-ms-user-select: none;"));
        assert!(result.contains("user-select:none;") || result.contains("user-select: none;"));

        assert!(result.contains("-webkit-appearance: none;"));
        assert!(result.contains("-moz-appearance: none;"));
        assert!(result.contains("appearance:none;") || result.contains("appearance: none;"));

        assert!(result.contains("-webkit-backdrop-filter: blur(5px);"));
        assert!(
            result.contains("backdrop-filter:blur(5px);")
                || result.contains("backdrop-filter: blur(5px);")
        );
    }
}
