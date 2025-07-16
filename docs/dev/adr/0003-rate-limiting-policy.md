# ADR 0003: Rate Limiting and Retry Policy

## Status
Accepted

## Context

The ClickUp API enforces strict rate limits (requests per minute) and returns HTTP 429 errors when exceeded. The CLI must:
- Avoid hitting API rate limits
- Handle rate limit errors gracefully
- Support different limits for free/paid accounts
- Provide a good user experience (automatic retry, progress feedback)

## Decision

- Implement a `RateLimiter` struct that tracks requests in a sliding 60-second window.
- Enforce configurable requests-per-minute (RPM) limits, with environment overrides.
- On approaching the limit, automatically delay requests to avoid hitting the quota.
- On HTTP 429 (rate limit) errors, automatically retry after the appropriate delay, up to a configurable maximum number of retries.
- Provide progress feedback for long waits.
- Allow users to configure rate limiting parameters (RPM, buffer seconds, max retries) via environment variables or config.
- All API requests go through the rate limiter.

## Consequences

### Benefits
- **Reliability**: Prevents API bans and failed requests due to rate limits
- **User Experience**: Automatic retry and progress feedback
- **Configurability**: Supports different account types and user needs
- **Maintainability**: Centralized rate limiting logic
- **Testability**: Rate limiter is unit tested

### Risks/Tradeoffs
- Slightly increased latency when approaching limits
- Complexity in handling edge cases (e.g., clock skew, burst traffic)

## Outcomes
- ✅ CLI never exceeds ClickUp API rate limits
- ✅ All rate limit errors are handled gracefully
- ✅ Users can tune rate limiting for their account
- ✅ Rate limiting logic is tested and documented

## Related Documents
- [Architectural Analysis and Recommendations](../ARCHITECTURAL_ANALYSIS_AND_RECOMMENDATIONS.md)
- [src/rate_limiter.rs](../../src/rate_limiter.rs)

---

*Date: July 14, 2025*
*Status: Complete and Adopted* 