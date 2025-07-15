# ADR 0001: Library/Binary Separation

## Status
Accepted

## Context

The ClickUp CLI codebase was originally structured with binary-specific logic and reusable library code intermixed. This led to:
- Module duplication
- Scattered magic constants
- Difficulty maintaining configuration
- Poor separation of concerns
- Harder testing and extensibility

A clean separation between library and binary concerns was recommended in the architectural analysis to address these issues and lay a foundation for future improvements.

## Decision

We implemented a clear separation between the library and binary by:

- Creating a **constants module** (`src/constants.rs`) to centralize all magic values and configuration
- Updating **library exports** (`src/lib.rs`) to provide a clean, public API and re-export common constants
- Creating an **application layer** (`src/app.rs`) to encapsulate binary-specific logic, CLI structure, command routing, and lifecycle management
- Enhancing **configuration** (`src/config.rs`) to support environment-overridable values and type-safe access
- Simplifying the **main entry point** (`src/main.rs`) to delegate to the application layer
- Updating the **API module** (`src/api.rs`) to use configuration-driven values instead of hardcoded constants

## Consequences

### Benefits
- **Clean Architecture**: Clear separation between reusable library code and binary-specific logic
- **Centralized Configuration**: All magic values and configuration in one place, with environment overrides
- **Improved Maintainability**: No module duplication, clear dependencies, and better error handling
- **Enhanced Flexibility**: Easy to add new configuration options, override via environment, and test
- **Extensibility**: Foundation for future improvements (repository pattern, caching, DI, plugins)
- **Documentation**: Clear API documentation and usage examples

### Risks/Tradeoffs
- Requires ongoing discipline to maintain separation
- Some initial refactoring effort

## Outcomes
- ✅ Eliminated module duplication
- ✅ Centralized all constants
- ✅ Clean separation of binary logic
- ✅ Configuration-driven and environment-overridable
- ✅ Maintained all existing functionality
- ✅ Improved codebase maintainability, testability, and extensibility

## Related Documents
- [Library/Binary Separation Implementation](../ARCHITECTURAL_ANALYSIS_AND_RECOMMENDATIONS.md#1-completed-librarybinary-reorganization)
- [Architectural Analysis and Recommendations](../ARCHITECTURAL_ANALYSIS_AND_RECOMMENDATIONS.md)

---

*Date: July 14, 2025*
*Status: Complete and Tested* 