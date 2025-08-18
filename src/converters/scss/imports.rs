use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn inline_scss_file(
    path: &Path,
    visited: &mut HashSet<PathBuf>,
    out: &mut String,
) -> std::io::Result<()> {
    let abs = fs::canonicalize(path)?;
    if !visited.insert(abs.clone()) {
        return Ok(());
    }
    let base_dir = abs.parent().unwrap_or_else(|| Path::new("."));
    let content = fs::read_to_string(&abs)?;

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(target) = parse_import_target(trimmed) {
            // Try to resolve any local-like target; if we cannot, print a warning but keep the line
            if let Some(candidate) = resolve_scss_like_path(base_dir, &target) {
                // Recursively inline dependency
                inline_scss_file(&candidate, visited, out)?;
                continue; // do not emit the @use/@import line
            } else {
                eprintln!(
                    "[scss] Warning: could not resolve import '{}' referenced from {}",
                    target,
                    abs.display()
                );
                // fall through to emit the original line
            }
        }
        out.push_str(line);
        out.push('\n');
    }

    // Note: simple selector nesting is flattened in a later pass (see nesting.rs)

    Ok(())
}

/// Extracts the import path from lines like:
/// @use "./foo";
/// @import './bar.scss';
/// @import "../baz"
/// Returns None for non-import lines.
fn parse_import_target(line: &str) -> Option<String> {
    let lower = line.to_ascii_lowercase();
    if !(lower.starts_with("@use ") || lower.starts_with("@import ")) {
        return None;
    }
    // Find first quote
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] != b'"' && bytes[i] != b'\'' {
        i += 1;
    }
    if i >= bytes.len() {
        return None;
    }
    let quote = bytes[i];
    i += 1; // skip quote
    let start = i;
    while i < bytes.len() && bytes[i] != quote {
        i += 1;
    }
    if i > start {
        let raw = &line[start..i];
        return Some(raw.trim().to_string());
    }
    None
}

/// Try to resolve SCSS-like import path to a file on disk.
/// Resolution strategy:
/// - Exact path as given
/// - Append ".scss"
/// - Prepend underscore partial variant for both cases
fn resolve_scss_like_path(base_dir: &Path, target: &str) -> Option<PathBuf> {
    let candidate = base_dir.join(target);
    let candidates = [
        candidate.clone(),
        candidate.with_extension("scss"),
        candidate.with_file_name(format!("_{}", candidate.file_name()?.to_string_lossy())),
        {
            let with_ext = candidate.with_extension("scss");
            with_ext.with_file_name(format!("_{}", with_ext.file_name()?.to_string_lossy()))
        },
    ];
    candidates.into_iter().find(|c| c.exists())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn inlines_relative_use_and_import() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        let main_scss = root.join("main.scss");
        let components_dir = root.join("components");
        fs::create_dir_all(&components_dir).unwrap();

        let button_scss = components_dir.join("_button.scss");
        let typography_scss = components_dir.join("typography.scss");

        fs::write(&button_scss, ".btn { color: red; }\n").unwrap();
        fs::write(&typography_scss, "h1 { font-weight: 700; }\n").unwrap();

        let mut main = fs::File::create(&main_scss).unwrap();
        writeln!(main, "@use './components/button';").unwrap();
        writeln!(main, "@import \"./components/typography.scss\";").unwrap();
        writeln!(main, "body {{ margin: 0; }}").unwrap();

        let mut visited: HashSet<PathBuf> = HashSet::new();
        let mut css = String::new();
        inline_scss_file(&main_scss, &mut visited, &mut css).unwrap();
        assert!(css.contains(".btn { color: red; }"));
        assert!(css.contains("h1 { font-weight: 700; }"));
        assert!(css.contains("body { margin: 0; }"));
        assert!(!css.contains("@use"));
        assert!(!css.contains("@import"));
    }

    #[test]
    fn prevents_infinite_recursion() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let a = root.join("a.scss");
        let b = root.join("b.scss");
        fs::write(&a, "@import './b';\n.a{color:blue;}\n").unwrap();
        fs::write(&b, "@use './a';\n.b{color:red;}\n").unwrap();
        let mut visited: HashSet<PathBuf> = HashSet::new();
        let mut css = String::new();
        inline_scss_file(&a, &mut visited, &mut css).unwrap();
        assert!(css.contains(&".a{color:blue;}".replace("{color:blue;}", "{color:blue;}")));
        assert!(css.contains(&".b{color:red;}".replace("{color:red;}", "{color:red;}")));
    }

    #[test]
    fn resolves_bare_import_names_in_same_directory() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let main = root.join("style.scss");
        let partial = root.join("_window.scss");
        fs::write(&partial, ".window { display: block; }\n").unwrap();
        fs::write(&main, "@import 'window';\nbody { margin: 0; }\n").unwrap();

        let mut visited: HashSet<PathBuf> = HashSet::new();
        let mut css = String::new();
        inline_scss_file(&main, &mut visited, &mut css).unwrap();
        assert!(css.contains(".window { display: block; }"));
        assert!(css.contains("body { margin: 0; }"));
        assert!(!css.contains("@import"));
    }
}
