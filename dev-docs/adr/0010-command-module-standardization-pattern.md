# ADR 0010: Command Module Standardization Pattern

## Status
Accepted

## Context

The ClickUp CLI command modules had significant code duplication across multiple patterns:
- Execute function patterns (7/7 files)
- Table creation patterns (7/7 files)
- Empty results handling (6/6 files)
- Details display patterns (6/6 files)
- API client creation (6/7 files)
- Error handling patterns (3/3 files)
- Member display patterns (2/2 files)

This duplication made the codebase harder to maintain and created inconsistencies in user experience.

## Decision

Implement a comprehensive standardization pattern through utility modules:

- **CommandExecutor Trait**: Standardize command execution pattern across all modules
- **TableBuilder Pattern**: Builder pattern for consistent table creation and formatting
- **DisplayUtils Pattern**: Centralized output formatting for details and empty results
- **ErrorUtils Pattern**: Standardized error creation and handling
- **ApiUtils Pattern**: Centralized API client creation and management
- **TableHeaders**: Consistent table header constants

All command modules must use these standardized patterns instead of implementing their own versions.

## Consequences

### Benefits
- **Code Reduction**: ~200+ lines of duplicate code eliminated
- **Consistency**: All command modules follow the same patterns
- **Maintainability**: Changes to common patterns only need to be made in one place
- **Readability**: Command logic is cleaner and more focused
- **User Experience**: Consistent output formatting across all commands

### Risks/Tradeoffs
- Requires discipline to keep all command modules using the standardized patterns
- Slightly more complex initial setup for new command modules
- Need to maintain the utility modules alongside the command modules

## Outcomes
- ✅ All command modules use standardized patterns
- ✅ ~30-40% reduction in command file sizes
- ✅ Consistent user experience across all commands
- ✅ Easier to add new commands with consistent behavior

## Related Documents
- [Command Duplicate Code Analysis](../archived/COMMAND_DUPLICATE_CODE_ANALYSIS.md)

---

*Date: July 15, 2025*
*Status: Complete and Adopted* 