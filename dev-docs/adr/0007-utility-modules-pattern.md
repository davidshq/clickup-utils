# ADR 0007: Utility Modules Pattern

## Status
Accepted

## Context

The ClickUp CLI codebase needed to eliminate code duplication and provide consistent functionality across all command modules. Common patterns like table creation, display formatting, error handling, and repository creation were being duplicated across multiple command files.

The architectural analysis identified the need for centralized utility modules to provide reusable functionality.

## Decision

- Create a centralized `utils.rs` module in the commands directory
- Implement specialized utility modules for common functionality:
  - **TableBuilder**: Builder pattern for consistent table creation
  - **DisplayUtils**: Standardized output formatting
  - **ErrorUtils**: Consistent error creation and handling
  - **RepositoryUtils**: Centralized repository creation
  - **TableHeaders**: Standardized table header constants
- Ensure all command modules use these utilities instead of implementing their own versions
- Maintain consistent interfaces and error handling across all utilities

## Consequences

### Benefits
- **Reduced Duplication**: Eliminates repetitive code across command modules
- **Consistency**: All commands use the same utilities for common operations
- **Maintainability**: Changes to common functionality only need to be made in one place
- **Testability**: Utilities can be tested independently
- **Extensibility**: Easy to add new utilities or extend existing ones

### Risks/Tradeoffs
- Requires discipline to use utilities instead of custom implementations
- Slight learning curve for new developers to understand available utilities

## Outcomes
- ✅ All command modules now use centralized utilities
- ✅ Consistent table creation across all commands
- ✅ Standardized error handling and display formatting
- ✅ Centralized repository creation logic
- ✅ Reduced code duplication and improved maintainability

## Related Documents
- [ADR 0006: CommandExecutor Pattern](../adr/0006-command-executor-pattern.md)
- [Architectural Analysis and Recommendations](../ARCHITECTURAL_ANALYSIS_AND_RECOMMENDATIONS.md)

---

*Date: July 14, 2025*
*Status: Complete and Adopted* 