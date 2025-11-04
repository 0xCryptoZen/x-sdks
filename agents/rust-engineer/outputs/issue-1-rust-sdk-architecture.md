# Rust SDK Architecture Guidance - Issue #1

**Agent:** rust-engineer
**Date:** 2025-11-04
**Issue:** #1 - X (Twitter) SDK for Rust

---

## 🎯 Architecture Overview

The Rust SDK should follow a layered architecture pattern:

```
┌─────────────────────────────────┐
│   Public API Layer              │  (XClient, builder pattern)
├─────────────────────────────────┤
│   Authentication Layer          │  (OAuth 1.0a signing)
├─────────────────────────────────┤
│   HTTP Transport Layer          │  (reqwest)
├─────────────────────────────────┤
│   Type System Layer             │  (Request/Response types)
└─────────────────────────────────┘
```

---

## 📦 Project Structure

Recommend using workspace structure for future extensibility:

```
rust-sdk/
├── Cargo.toml (workspace)
├── x-sdk/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs          # Public API exports
│   │   ├── client.rs       # XClient implementation
│   │   ├── auth.rs         # OAuth 1.0a implementation
│   │   ├── types/
│   │   │   ├── mod.rs
│   │   │   ├── request.rs  # Request types
│   │   │   ├── response.rs # Response types
│   │   │   └── error.rs    # Error types
│   │   └── endpoints/
│   │       ├── mod.rs
│   │       └── tweets.rs   # Tweet endpoints
│   └── tests/
│       ├── integration_tests.rs
│       └── fixtures/
└── examples/
    └── post_tweet.rs
```

---

## 🔑 Core Components

### 1. Client Design Pattern

**Recommendation:** Use the Builder pattern for configuration

**Rationale:**
- Flexible credential setup
- Optional HTTP client customization
- Clear API for users
- Type-safe construction

**Key Traits:**
- `Clone` for client reuse
- `Send + Sync` for tokio compatibility
- Interior mutability NOT needed (immutable design)

---

### 2. Authentication Strategy

**OAuth 1.0a Implementation Approach:**

**Option A: Use existing crate (RECOMMENDED)**
- Pros: Battle-tested, handles edge cases, RFC compliant
- Cons: External dependency
- Recommendation: `oauth1-request` crate

**Option B: Custom implementation**
- Pros: Zero extra dependencies, full control
- Cons: Complex signature algorithm, high risk of bugs
- **NOT RECOMMENDED** for security-critical code

**Components Needed:**
1. Nonce generation (cryptographically secure)
2. Timestamp generation (Unix epoch)
3. Parameter encoding (percent encoding per RFC 3986)
4. Signature base string construction
5. HMAC-SHA1 signing
6. Authorization header formatting

---

### 3. Type System Design

**Request Types:**
- Use `#[derive(Serialize)]` from serde
- Use builder pattern for optional fields
- Validate constraints (e.g., tweet length) at construction time
- Use `&str` in builders, `String` in final types

**Response Types:**
- Use `#[derive(Deserialize)]` from serde
- Handle optional fields with `Option<T>`
- Use `#[serde(rename_all = "snake_case")]` for API conventions
- Flatten nested structures where beneficial

**Error Types:**
- Custom enum implementing `std::error::Error`
- Wrap underlying errors (reqwest, serde)
- Include HTTP status codes
- Include API error messages
- Use `thiserror` crate for ergonomic error handling

---

### 4. HTTP Layer Design

**Client Configuration:**
- Use `reqwest::Client` with custom configuration
- Set reasonable timeouts (30s default)
- Enable gzip compression
- Configure User-Agent header
- Connection pooling (automatic with reqwest)

**Request Pipeline:**
1. Construct typed request
2. Serialize to JSON
3. Generate OAuth signature
4. Add Authorization header
5. Send HTTP request
6. Handle response status
7. Deserialize response body
8. Map to Result type

---

## 🛡️ Error Handling Strategy

**Error Categories:**

1. **Authentication Errors** (401, 403)
   - Invalid credentials
   - Expired tokens
   - Signature mismatch

2. **Client Errors** (400, 422)
   - Invalid request format
   - Validation failures
   - Duplicate content

3. **Rate Limiting** (429)
   - Include rate limit headers
   - Expose retry-after duration

4. **Server Errors** (500, 502, 503)
   - Transient failures
   - Service degradation

5. **Network Errors**
   - Connection timeouts
   - DNS failures
   - SSL errors

**Error Propagation:**
- Use `Result<T, XError>` everywhere
- Implement `From<E>` for common error types
- Provide context with error messages
- Do NOT panic in library code

---

## 🔒 Security Considerations

1. **Credential Handling:**
   - Never log credentials
   - Use `secrecy` crate for sensitive data
   - Clear credentials from memory when possible
   - No default credentials

2. **Request Signing:**
   - Use cryptographically secure random for nonces
   - Validate timestamp to prevent replay attacks
   - Sort parameters consistently
   - Use constant-time comparison where applicable

3. **TLS:**
   - Enforce HTTPS only
   - Validate certificates
   - Use modern TLS versions (1.2+)

---

## ⚡ Performance Considerations

1. **Connection Reuse:**
   - Single `reqwest::Client` per `XClient`
   - Connection pooling automatic

2. **Memory:**
   - Avoid unnecessary clones
   - Use `&str` in APIs where possible
   - Stream large responses (future: media upload)

3. **Async Design:**
   - All I/O operations async
   - Compatible with tokio runtime
   - No blocking operations in async context

---

## 🧪 Testing Strategy

1. **Unit Tests:**
   - Test OAuth signature generation
   - Test type serialization/deserialization
   - Test error handling

2. **Integration Tests:**
   - Mock HTTP server using `wiremock`
   - Test full request/response cycle
   - Test error scenarios

3. **Example Code:**
   - Real-world usage examples
   - Demonstrate error handling
   - Show best practices

---

## 📚 Dependencies Recommendation

**Required:**
```toml
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
oauth1-request = "0.5"  # For OAuth 1.0a
```

**Development:**
```toml
[dev-dependencies]
wiremock = "0.5"
tokio-test = "0.4"
```

**Optional Enhancement:**
```toml
thiserror = "1.0"  # Ergonomic error handling
secrecy = "0.8"    # Secure credential storage
```

---

## 🚀 Implementation Phases

### Phase 1: Foundation (Priority: High)
- Project structure setup
- Core type definitions
- Error type hierarchy
- Basic client struct

### Phase 2: Authentication (Priority: High)
- OAuth 1.0a implementation
- Signature generation
- Authorization header construction
- Credential validation

### Phase 3: Tweet Posting (Priority: High)
- POST /2/tweets endpoint
- Request building
- Response parsing
- Error handling

### Phase 4: Testing (Priority: High)
- Unit tests
- Integration tests with mocks
- Example code
- Documentation

### Phase 5: Future Extensions (Priority: Low)
- Media upload
- Rate limit handling
- Additional endpoints
- Retry mechanisms

---

## 💡 Best Practices

1. **API Design:**
   - Ergonomic for common use cases
   - Hard to misuse
   - Clear error messages
   - Comprehensive documentation

2. **Versioning:**
   - Use SemVer strictly
   - Document breaking changes
   - Provide migration guides

3. **Documentation:**
   - Rustdoc for all public items
   - Code examples in docs
   - README with quick start
   - Architecture decision records

4. **Code Quality:**
   - Run `clippy` with all warnings
   - Format with `rustfmt`
   - No `unsafe` code unless absolutely necessary
   - Minimum of warnings

---

## ⚠️ Common Pitfalls to Avoid

1. **OAuth Signature Bugs:**
   - Incorrect parameter encoding
   - Wrong sorting order
   - Including OAuth parameters in signature
   - Timestamp/nonce issues

2. **Error Handling:**
   - Swallowing errors silently
   - Not providing context
   - Using `unwrap()` in library code
   - Generic error messages

3. **Type Design:**
   - Over-complicated builders
   - Missing validation
   - Poor error messages for validation failures

4. **Testing:**
   - Not mocking external services
   - Hard-coding credentials in tests
   - Not testing error paths

---

## 🎓 Reference Materials

- [Twitter API v2 Documentation](https://developer.twitter.com/en/docs/twitter-api)
- [OAuth 1.0a RFC 5849](https://tools.ietf.org/html/rfc5849)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [reqwest Documentation](https://docs.rs/reqwest/)
- [serde Documentation](https://serde.rs/)

---

**Next Steps:**
1. Set up Cargo workspace
2. Define type hierarchy
3. Implement OAuth layer
4. Implement tweet endpoint
5. Add comprehensive tests

**Remember:** This is guidance only. You will implement the actual code based on these architectural recommendations.
