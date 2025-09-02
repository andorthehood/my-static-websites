use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::traits::AssetConverter;
use crate::error::Result;
mod imports;
mod nesting;

/// Very primitive SCSS to CSS converter that only supports inlining of `@use` and `@import`.
/// - Only local relative imports ("./" or "../") are supported.
/// - Supports optional quotes and optional trailing semicolon.
/// - Ignores media queries or import options; just inlines raw content.
/// - Prevents infinite recursion by tracking visited absolute paths.
/// - Does NOT process variables, mixins, etc.
/// - Adds minimal support for flattening simple nested selectors like `.foo { .bar { ... } }` → `.foo .bar { ... }`.
pub fn scss_to_css_with_inline_imports(entry_path: &Path) -> std::io::Result<String> {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut inlined = String::new();
    imports::inline_scss_file(entry_path, &mut visited, &mut inlined)?;
    let flattened = nesting::flatten_basic_nesting(&inlined);
    Ok(flattened)
}

/// SCSS to CSS converter implementation
pub struct ScssConverter;

impl ScssConverter {
    /// Create a new SCSS converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for ScssConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetConverter for ScssConverter {
    fn convert(&self, input: &str, source_path: Option<&Path>) -> Result<String> {
        if let Some(path) = source_path {
            // Use the existing file-based conversion when path is available
            scss_to_css_with_inline_imports(path).map_err(|e| e.into())
        } else {
            // For string-only input, just flatten nesting (no imports possible)
            Ok(nesting::flatten_basic_nesting(input))
        }
    }

    fn supported_extensions(&self) -> Vec<&str> {
        vec!["scss"]
    }

    fn output_extension(&self) -> &str {
        "css"
    }
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
}

#[cfg(test)]
mod trait_tests {
    use super::*;
    use crate::traits::AssetConverter;

    #[test]
    fn test_scss_converter_trait() {
        let converter = ScssConverter::new();
        assert_eq!(converter.supported_extensions(), vec!["scss"]);
        assert_eq!(converter.output_extension(), "css");

        // Test basic nesting without file path
        let input = ".foo { .bar { color: red; } }";
        let result = converter.convert(input, None).expect("Conversion failed");
        assert!(result.contains(".foo .bar"));
    }

    #[test]
    fn test_scss_converter_with_path() {
        let converter = ScssConverter::new();
        let path = Path::new("sites/lepkef.ing/assets/style.scss");
        
        // Only run this test if the file exists
        if path.exists() {
            let result = converter.convert("", Some(path));
            assert!(result.is_ok());
        }
    }
}
