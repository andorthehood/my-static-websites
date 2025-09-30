use std::collections::HashSet;
use std::path::{Path, PathBuf};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_real_style_scss_contains_expected_rules() {
        let path = Path::new("sites/polgarand.org/assets/style.scss");
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
