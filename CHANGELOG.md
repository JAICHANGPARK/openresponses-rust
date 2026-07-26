# Changelog

All notable changes to this project will be documented in this file.

## [2026.7.26] - 2026-07-26

### Added
- **OpenResponses 2026-04-24 Spec Support**:
  - **Conversation Compaction**: Added `Item::Compaction`, `CompactResponseBody`, `CompactResource`, and `Client::compact_response` / `compact_response_raw` for `/v1/responses/compact`.
  - **Assistant Message Phase**: Added `MessagePhase` (`commentary`, `final_answer`) field to assistant messages to preserve intermediate reasoning vs final answer state for models like `gpt-5.3-codex`.
  - **Video Content Input**: Added support for `MessageContent::InputVideo` (`input_video`).
  - **WebSocket Type Definitions**: Added `WebSocketResponseCreateEvent`, `WebSocketErrorEvent`, and `WebSocketErrorPayload`.
- **Package Updates**:
  - Upgraded dependencies (`reqwest`, `tokio`, `bytes`, `mockito`, etc.).

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
