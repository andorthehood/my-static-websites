# TODO: Module-Level Privacy Control

**Priority**: 🟡
**Estimated Effort**: 2-3 hours
**Created**: 2024-12-19
**Status**: Open

## Problem Description

The current module structure exposes too many implementation details through public APIs. Many functions and types that should be internal to modules are currently public, creating a leaky abstraction. This makes the codebase harder to maintain and refactor.

Current issues:
- Many internal functions are marked as `pub` unnecessarily
- Implementation details are exposed in module interfaces
- Hard to change internal implementations without breaking external code
- No clear distinction between public API and internal implementation

## Proposed Solution

Use Rust's visibility modifiers to create proper module boundaries:

- Use `pub(crate)` for functions that need to be accessible within the crate but not externally
- Use `pub(super)` for functions that should only be accessible to parent modules
- Keep implementation details private with no visibility modifier
- Only expose the minimal necessary public API

## Implementation Plan

### Step 1: Audit current visibility
- Review all `pub` functions and identify which should be internal
- Check which functions are actually used across module boundaries
- Document the intended public API for each module

### Step 2: Update visibility modifiers
- Change unnecessary `pub` functions to `pub(crate)` or private
- Update module re-exports to only expose necessary interfaces
- Ensure all internal functions are properly scoped

### Step 3: Update imports and dependencies
- Fix any broken imports after visibility changes
- Update module documentation to reflect new public APIs
- Run tests to ensure no functionality is broken

## Success Criteria

- [ ] All modules have clear, minimal public APIs
- [ ] Implementation details are properly encapsulated
- [ ] No unnecessary public functions exposed
- [ ] All tests pass after visibility changes
- [ ] Module documentation reflects actual public interfaces

## Affected Components

- `src/template_processors/` - Update visibility of internal functions
- `src/converters/` - Update visibility of internal functions
- `src/minifier/` - Update visibility of internal functions
- `src/parsers/` - Update visibility of internal functions
- `src/server/` - Update visibility of internal functions
- All module `mod.rs` files - Update re-exports

## Risks & Considerations

- **Breaking Changes**: Changing visibility may break internal dependencies
- **Testing**: Need to ensure all tests still work with new visibility
- **Documentation**: Public API documentation needs to be updated
- **Dependencies**: Need to carefully track which functions are used where

## Related Items

- **Depends on**: TODO 001 (Traits for Abstract Interfaces)
- **Blocks**: TODO 003 (Builder Pattern for Complex Operations)
- **Related**: TODO 008 (Module Re-exports for Clean Public API)

## References

- [Rust Book - Visibility and Privacy](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
- [Rust Reference - Visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html)

## Notes

This change will make the codebase more maintainable and provide clearer boundaries between modules. Should be done after establishing trait interfaces to ensure proper abstraction layers.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
