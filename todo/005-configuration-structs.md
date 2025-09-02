# TODO: Configuration Structs

**Priority**: 🟢
**Estimated Effort**: 2-3 hours
**Created**: 2024-12-19
**Status**: Open

## Problem Description

The current configuration in `src/config.rs` uses scattered constants and hardcoded values. This makes it difficult to customize the behavior of the static site generator and hard to add new configuration options.

Current issues:
- Configuration is scattered across multiple constants
- Hard to add new configuration options
- No way to customize behavior without code changes
- Configuration values are hardcoded in multiple places
- No validation for configuration values

## Proposed Solution

Replace scattered constants with structured configuration:

- Create `SiteConfig` struct with all configuration options
- Implement `Default` trait for sensible defaults
- Add configuration validation
- Support configuration from files or environment variables
- Make configuration easily extensible

## Implementation Plan

### Step 1: Design configuration structure
- Create `SiteConfig` struct with all current configuration options
- Implement `Default` trait with current hardcoded values
- Add validation methods for configuration values
- Design configuration loading from files

### Step 2: Implement configuration loading
- Add support for loading configuration from files
- Add environment variable support for configuration
- Implement configuration validation
- Add configuration merging (file + environment + defaults)

### Step 3: Update codebase to use new configuration
- Replace hardcoded constants with configuration struct
- Update all modules to use the new configuration system
- Add configuration validation at startup
- Ensure backward compatibility

## Success Criteria

- [ ] All configuration is centralized in `SiteConfig` struct
- [ ] Configuration can be loaded from files and environment variables
- [ ] Configuration validation is implemented
- [ ] All modules use the new configuration system
- [ ] Backward compatibility is maintained

## Affected Components

- `src/config.rs` - Complete rewrite with structured configuration
- `src/generate.rs` - Update to use new configuration
- `src/watch.rs` - Update to use new configuration
- `src/server/` - Update to use new configuration
- All modules - Update to use new configuration system

## Risks & Considerations

- **Breaking Changes**: Configuration changes may break existing setups
- **Complexity**: More complex configuration system
- **Performance**: Configuration loading may add startup overhead
- **Testing**: Need to test all configuration combinations

## Related Items

- **Depends on**: TODO 004 (Error Type Abstraction)
- **Blocks**: TODO 006 (Dependency Injection Pattern)
- **Related**: TODO 003 (Builder Pattern for Complex Operations)

## References

- [Rust Book - Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [serde crate documentation](https://docs.rs/serde/latest/serde/)
- [config crate documentation](https://docs.rs/config/latest/config/)

## Notes

This change will make the static site generator much more configurable and easier to customize for different use cases. Should be implemented after error handling improvements.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
