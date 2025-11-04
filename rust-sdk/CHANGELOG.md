# Changelog

All notable changes to the Rust SDK will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of Rust SDK
- OAuth 1.0a authentication with automatic request signing
- Tweet posting support (POST /2/tweets)
- Comprehensive error handling with `XError` type
- Type-safe request/response handling with serde
- Async/await support with tokio
- Builder pattern for `XClient`

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - 2025-11-04

### Added
- Initial release
- `XClient` with OAuth 1.0a authentication
- Tweet posting functionality via `TweetsAPI`
- Type-safe request/response handling
- Error handling with `XError` enum
- Support for custom base URL and timeout configuration
- 13 passing unit tests

[Unreleased]: https://github.com/0xCryptoZen/x-sdks/compare/rust-v0.1.0...HEAD
[0.1.0]: https://github.com/0xCryptoZen/x-sdks/releases/tag/rust-v0.1.0
