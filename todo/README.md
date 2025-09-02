# Technical Debt & Future Improvements

This directory contains documentation for planned improvements and technical debt. Each TODO item has its own file for better tracking and focused discussions.

## File Naming Convention

- **Format**: `NNN-short-description.md` (3-digit number + kebab-case description)
- **Numbers**: Sequential, starting from 001
- **Descriptions**: Brief, descriptive, using kebab-case

## Priority Levels

- 🔴 **High** - Should be addressed before major releases
- 🟡 **Medium** - Should be addressed in next development cycle
- 🟢 **Low** - Nice to have, address when convenient

## Adding New TODOs

1. **Choose next sequential number** (check existing files)
2. **Use the template**: Copy `_template.md` to `NNN-description.md`
3. **Fill in details**: Complete all sections in the template
4. **Update this README**: Add entry to the appropriate priority section
5. **Assign priority**: Use 🔴🟡🟢 indicators

## Completing TODOs

1. **Mark as completed**: Update status to "Completed" in the TODO file
2. **Move to archived folder**: Move the completed TODO file to `archived/` folder
4. **Update related TODOs**: Check if completion affects other items and update dependencies

## Template

Use `_template.md` as the starting point for new TODO items. It includes all the standard sections and formatting.

## Archive Process

When a TODO is completed:
1. **Update the TODO file**: Change status from "Open" to "Completed"
2. **Add completion date**: Note when the task was finished
3. **Move to archived folder**: Move the file to `todo/archived/` for historical reference
5. **Review dependencies**: Update any other TODOs that depended on this completed item

## Current TODO Items

### 🟡 Medium Priority

- **001-traits-for-abstract-interfaces.md** - Create trait-based interfaces for better abstraction
- **002-module-level-privacy-control.md** - Use visibility modifiers to control module boundaries
- **003-builder-pattern-for-complex-operations.md** - Implement builder pattern for site generation
- **004-error-type-abstraction.md** - Improve error handling with better context and types
- **006-dependency-injection-pattern.md** - Implement dependency injection for better testing
- **009-interface-segregation.md** - Break down large interfaces into focused ones
- **010-testing-with-mocks.md** - Implement comprehensive testing with mocks

### 🟢 Low Priority

- **005-configuration-structs.md** - Replace scattered constants with structured configuration
- **007-feature-flags-for-optional-components.md** - Add feature flags for optional functionality
- **008-module-re-exports-for-clean-public-api.md** - Create clean public API through re-exports

## Implementation Order

The TODOs are designed to be implemented in order, with each building on the previous ones:

1. **001** → **002** → **003** → **004** → **005** → **006** → **007** → **008** → **009** → **010**

This order ensures that foundational changes (traits, privacy) are implemented first, followed by architectural improvements (builder pattern, error handling), and finally testing improvements.

## Interface Improvements Focus

These TODO items focus specifically on creating clean interfaces and better architecture:

- **Traits and Abstractions**: Items 001, 009 provide clean interfaces
- **Module Organization**: Items 002, 008 improve module boundaries
- **Configuration and DI**: Items 003, 005, 006 improve configurability
- **Error Handling**: Item 004 improves error context and debugging
- **Optional Features**: Item 007 enables feature flags
- **Testing**: Item 010 enables comprehensive testing with mocks

The goal is to transform the codebase from a tightly-coupled application into a well-architected, testable, and maintainable system while preserving the zero-dependency philosophy.
