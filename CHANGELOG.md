# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-02-02

### Added
- **ClientBuilder & StreamingClientBuilder**: Introduced a builder pattern for easier client configuration.
- **Automatic URL Normalization**: The client now automatically appends `/v1` to the base URL if missing.
- **MCP Tool Support**: Added support for `mcp` (Model Context Protocol) tool types, as used in LM Studio.
- **Stateful Follow-up Support**: Improved support for `previous_response_id` in requests.
- **New Examples**:
    - `examples/lm_studio.rs`: Demonstrates connection to local LLM servers.
    - `examples/stateful_follow_up.rs`: Shows how to maintain conversation state.

### Changed
- **Schema Alignment**: Refactored `Item` and `Content` types to strictly follow the OpenAI Responses API schema.
- **Unified MessageContent**: Merged `InputContent` and `OutputContent` into `MessageContent` for better schema compliance while maintaining backward compatibility via type aliases.
- **Default Base URL**: Changed internal default from `https://api.openai.com/v1` to `https://api.openai.com` (with auto-normalization adding `/v1`).

### Fixed
- Improved serialization/deserialization logic for tool calls and content parts.
- Fixed refutability issues in `Tool` helper methods.
