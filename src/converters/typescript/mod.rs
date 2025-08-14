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
    let without_non_null = remove_postfix_non_null(&without_types);
    without_non_null
}

#[cfg(test)]
mod tests;
