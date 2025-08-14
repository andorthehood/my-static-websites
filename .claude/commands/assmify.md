Please refactor an existing utility function to use x86 assembly. Requirements:
> 1. Choose an existing function that is currently implemented in pure Rust
> 2. The function must be actively used somewhere in the codebase (not just in tests)
> 3. Replace the existing implementation with an assembly-optimized version that maintains the same interface
> 4. Use the established pattern (separate .s file for assembly, Rust glue code with conditional compilation)
> 5. Verify the refactored function works by running tests