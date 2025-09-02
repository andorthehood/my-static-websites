mod is_identifier_char;

mod as_casts;
mod call_generics;
mod interface_blocks;
mod postfix_non_null;
mod query_selector_generics;
mod type_annotations;
mod utils;

use as_casts::remove_as_casts;
use call_generics::remove_generics_before_calls;
use interface_blocks::remove_interface_blocks;
use postfix_non_null::remove_postfix_non_null;
use query_selector_generics::remove_query_selector_generics;
use type_annotations::remove_type_annotations;
use crate::traits::AssetConverter;
use crate::error::Result;
use std::path::Path;

/// Minimal TypeScript-to-JavaScript stripper tailored for constructs used in router.ts.
/// This does not fully parse TS; it heuristically removes:
/// - `interface ... { ... }` blocks
/// - Generic annotations after `querySelector`/`querySelectorAll`, e.g. `<HTMLElement>`
/// - Generic arguments after identifiers when directly followed by a call, e.g. `Promise<void>(...)`
/// - Parameter and return type annotations in functions and arrow functions
/// - Variable type annotations in `const`/`let`/`var` declarations
/// - `as Type` casts (e.g., `(style as HTMLLinkElement)` -> `(style)`)
/// - Postfix non-null assertions like `value!` or `call()`
///
/// It intentionally does NOT implement enums or other TS features.
pub fn strip_typescript_types(input: &str) -> String {
    let without_interfaces = remove_interface_blocks(input);
    let without_generics = remove_query_selector_generics(&without_interfaces);
    let without_call_generics = remove_generics_before_calls(&without_generics);
    let without_casts = remove_as_casts(&without_call_generics);
    let without_types = remove_type_annotations(&without_casts);
    
    remove_postfix_non_null(&without_types)
}

/// TypeScript to JavaScript converter implementation
pub struct TypeScriptConverter;

impl TypeScriptConverter {
    /// Create a new TypeScript converter
    pub fn new() -> Self {
        Self
    }
}

impl Default for TypeScriptConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetConverter for TypeScriptConverter {
    fn convert(&self, input: &str, _source_path: Option<&Path>) -> Result<String> {
        // TypeScript conversion doesn't need the source path
        Ok(strip_typescript_types(input))
    }

    fn supported_extensions(&self) -> Vec<&str> {
        vec!["ts", "tsx"]
    }

    fn output_extension(&self) -> &str {
        "js"
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod trait_tests {
    use super::*;
    use crate::traits::AssetConverter;

    #[test]
    fn test_typescript_converter_trait() {
        let converter = TypeScriptConverter::new();
        assert_eq!(converter.supported_extensions(), vec!["ts", "tsx"]);
        assert_eq!(converter.output_extension(), "js");

        let input = "interface User { name: string; } const user: User = { name: 'test' };";
        let result = converter.convert(input, None).expect("Conversion failed");
        
        // Should remove interface and type annotation
        assert!(!result.contains("interface User"));
        assert!(!result.contains(": User"));
        assert!(result.contains("const user = { name: 'test' };"));
    }

    #[test]
    fn test_typescript_converter_with_generics() {
        let converter = TypeScriptConverter::new();
        let input = "document.querySelector<HTMLElement>('.test')";
        let result = converter.convert(input, None).expect("Conversion failed");
        
        // Should remove generic type parameter
        assert_eq!(result, "document.querySelector('.test')");
    }
}
