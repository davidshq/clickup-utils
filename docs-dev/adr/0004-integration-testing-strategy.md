# ADR 0004: Integration Testing Strategy

## Status
Accepted

## Context

The ClickUp CLI interacts with a live third-party API and must be robust against real-world scenarios. Integration tests are needed to:
- Validate end-to-end functionality against the real ClickUp API
- Ensure authentication, CRUD, error handling, and rate limiting work as expected
- Prevent regressions in user-facing commands
- Provide confidence in releases

## Decision

- All integration tests use the `CLICKUP_API_TOKEN_TEST` environment variable for authentication.
- Test credentials and IDs are stored in a `.env.test` file (not checked into version control).
- Tests are isolated: they use unique names/tags for resources and clean up after themselves.
- Tests are marked with `#[ignore]` by default and only run when explicitly requested.
- Helper functions are provided for environment setup, resource discovery, and cleanup.
- Tests cover authentication, workspace/team listing, task lifecycle, commenting, error handling, rate limiting, list operations, and CLI basics.
- Tests are designed to be robust, with dynamic resource discovery and graceful handling of missing credentials.
- Use ergonomic testing crates (`assert_cmd`, `predicates`, `dotenvy`, `serial_test`) for reliability and maintainability.

## Consequences

### Benefits
- **Reliability**: Ensures CLI works with the real API
- **Regression Prevention**: Catches breaking changes before release
- **Confidence**: Developers can refactor with safety
- **Documentation**: Test plan and structure are documented
- **Maintainability**: Tests are isolated, robust, and easy to run

### Risks/Tradeoffs
- Integration tests require valid API credentials and may create/delete real data
- Tests are slower than unit tests and may be skipped in CI by default

## Outcomes
- ✅ All major CLI features are covered by integration tests
- ✅ Tests are passing and robust
- ✅ Test environment is isolated and safe
- ✅ Developers can run all integration tests with a single script or command

## Related Documents
- [Integration Test Plan](../INTEGRATION_TEST_PLAN.md)
- [Test Coverage Analysis](../TEST_COVERAGE_ANALYSIS.md)

---

*Date: July 14, 2025*
*Status: Complete and Adopted* 