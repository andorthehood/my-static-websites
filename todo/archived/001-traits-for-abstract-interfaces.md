# TODO: Traits for Abstract Interfaces

**Priority**: 🟡
**Estimated Effort**: 4-6 hours
**Created**: 2024-12-19
**Status**: Open

## Problem Description

The current codebase lacks clear interface abstractions between modules. Template processing, asset conversion, and minification are tightly coupled with concrete implementations, making the code harder to test, maintain, and extend. There's no way to swap implementations or mock dependencies for testing.

Current issues:
- Direct function calls between modules create tight coupling
- No abstraction layer for different processor types
- Difficult to unit test components in isolation
- Hard to add new processor types without modifying existing code

## Proposed Solution

Create trait-based interfaces to define clean contracts between modules:

- `TemplateProcessor` trait for template processing operations
- `AssetConverter` trait for asset conversion (TypeScript, SCSS)
- `Minifier` trait for minification operations
- `FileProcessor` trait for file processing operations

This will enable dependency injection, easier testing with mocks, and cleaner separation of concerns.

## Implementation Plan

### Step 1: Create traits module
- Create `src/traits.rs` with base trait definitions
- Define `TemplateProcessor`, `AssetConverter`, `Minifier`, and `FileProcessor` traits
- Include proper error handling with `Result<T, Error>` return types

### Step 2: Implement traits for existing processors
- Implement `TemplateProcessor` for liquid and markdown processors
- Implement `AssetConverter` for TypeScript and SCSS converters
- Implement `Minifier` for HTML, CSS, and JS minifiers
- Maintain backward compatibility during transition

### Step 3: Update module interfaces
- Modify `template_processors/processor.rs` to use trait-based approach
- Update `converters/mod.rs` to expose trait implementations
- Update `minifier/mod.rs` to use trait interfaces

## Success Criteria

- [ ] All major processing components implement appropriate traits
- [ ] Unit tests can use mock implementations of traits
- [ ] No breaking changes to existing functionality
- [ ] Code is more modular and testable
- [ ] Documentation updated to reflect new interfaces

## Affected Components

- `src/traits.rs` - New file with trait definitions
- `src/template_processors/` - Update to implement traits
- `src/converters/` - Update to implement traits  
- `src/minifier/` - Update to implement traits
- `src/processor.rs` - Update to use trait-based approach
- `src/generate.rs` - May need updates to use new interfaces

## Risks & Considerations

- **Breaking Changes**: Risk of breaking existing functionality during refactoring
- **Performance**: Trait objects may have slight performance overhead
- **Complexity**: May increase code complexity initially
- **Dependencies**: Need to ensure all modules can work with new trait system

## Related Items

- **Depends on**: None (can be implemented independently)
- **Blocks**: TODO 002 (Module-Level Privacy Control)
- **Related**: TODO 010 (Testing with Mocks)

## References

- [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book - Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

## Notes

This is a foundational change that will enable many other improvements. Should be implemented early in the refactoring process to provide a solid foundation for other interface improvements.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
