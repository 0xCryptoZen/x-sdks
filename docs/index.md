# Welcome to X SDKs Documentation

## Overview

X SDKs provides dual-language implementations for Twitter/X API v2, offering identical interfaces in both **Rust** and **TypeScript**.

### Key Features

- 🔐 **OAuth 1.0a Authentication** - Automatic request signing
- 📝 **Tweet Posting** - Full support for POST /2/tweets
- 🛡️ **Type Safety** - Strong typing in both languages
- ⚠️ **Error Handling** - Comprehensive error types with retry logic
- 🔄 **Unified API** - Consistent interfaces across languages
- ✅ **Production Ready** - Tested and documented

## Quick Navigation

- [📖 API Reference](./API_DOCUMENTATION.html) - Complete API documentation
- [🦀 Rust SDK](./rust-sdk/README.html) - Rust SDK documentation
- [🚀 TypeScript SDK](./typescript-sdk/README.html) - TypeScript SDK documentation
- [💻 GitHub Repository](https://github.com/0xCryptoZen/x-sdks) - Source code

## Getting Started

### Installation

**Rust:**
```toml
[dependencies]
x-sdk = { git = "https://github.com/0xCryptoZen/x-sdks", subdirectory = "rust-sdk/x-sdk" }
tokio = { version = "1.0", features = ["full"] }
```

**TypeScript:**
```bash
npm install @x-sdks/typescript
```

### Your First Tweet

**Rust:**
```rust
use x_sdk::{XClient, Credentials, TweetRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::new(
        "app_key", "app_secret",
        "access_token", "access_secret"
    );

    let client = XClient::new(credentials)?;
    let tweet = TweetRequest::new("Hello, X! 🦀")?;
    let response = client.tweets().post(tweet).await?;

    println!("Tweet ID: {}", response.data.id);
    Ok(())
}
```

**TypeScript:**
```typescript
import { XClient, createTweetRequest } from '@x-sdks/typescript';

const client = new XClient({
  credentials: {
    appKey: 'app_key',
    appSecret: 'app_secret',
    accessToken: 'access_token',
    accessSecret: 'access_secret',
  },
});

const tweet = createTweetRequest('Hello, X! 🚀');
const response = await client.tweets().post(tweet);
console.log('Tweet ID:', response.data.id);
```

## API Consistency

Both SDKs follow identical patterns:

| Operation | Rust | TypeScript |
|-----------|------|------------|
| Initialize | `XClient::new(credentials)?` | `new XClient({ credentials })` |
| Post Tweet | `client.tweets().post(tweet).await?` | `await client.tweets().post(tweet)` |
| Handle Errors | `Result<T, XError>` | `try/catch XError` |

## Documentation Structure

### Core Documentation
- **[Home](./index.html)** - This page
- **[API Reference](./API_DOCUMENTATION.html)** - Unified API documentation
- **[Contributing](https://github.com/0xCryptoZen/x-sdks/blob/main/CLAUDE.md)** - Contribution guidelines

### SDK-Specific Guides
- **[Rust SDK Guide](./rust-sdk/README.html)** - Installation, usage, examples
- **[TypeScript SDK Guide](./typescript-sdk/README.html)** - Installation, usage, examples

## Examples

### Error Handling

**Rust:**
```rust
use x_sdk::{XError, TweetRequest};

match client.tweets().post(tweet).await {
    Ok(response) => println!("Success: {}", response.data.id),
    Err(XError::RateLimitExceeded { retry_after, .. }) => {
        eprintln!("Rate limited. Retry after: {:?}", retry_after);
    }
    Err(XError::AuthenticationFailed(msg)) => {
        eprintln!("Auth failed: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

**TypeScript:**
```typescript
import { XError, XErrorCode } from '@x-sdks/typescript';

try {
  const response = await client.tweets().post(tweet);
  console.log('Success:', response.data.id);
} catch (error) {
  if (error instanceof XError) {
    if (error.code === XErrorCode.RATE_LIMIT_EXCEEDED) {
      console.error('Rate limited. Retry after:', error.getRetryDelay());
    } else if (error.code === XErrorCode.AUTHENTICATION_FAILED) {
      console.error('Auth failed:', error.message);
    }
  }
}
```

### Advanced Configuration

**Rust:**
```rust
use std::time::Duration;

let client = XClient::builder(credentials)
    .timeout(Duration::from_secs(60))
    .base_url("https://api.twitter.com")
    .build()?;
```

**TypeScript:**
```typescript
const client = new XClient({
  credentials,
  timeout: 60000,
  baseUrl: 'https://api.twitter.com',
});
```

## Features

### Current Features
- ✅ OAuth 1.0a authentication
- ✅ POST /2/tweets (tweet posting)
- ✅ Tweet validation (280 characters)
- ✅ Comprehensive error handling
- ✅ Retry information for rate limits
- ✅ Type-safe request/response

### Planned Features
- 📋 Media upload (POST /1.1/media/upload)
- 📋 Tweet with media
- 📋 Get tweet by ID (GET /2/tweets/:id)
- 📋 Delete tweet (DELETE /2/tweets/:id)
- 📋 Rate limit handling with automatic retry
- 📋 Streaming API support

## Testing

### Rust
```bash
cd rust-sdk
cargo test
```
**Status:** 13/13 tests passing ✅

### TypeScript
```bash
cd typescript-sdk
npm test
```

## Support

- **Issues:** [GitHub Issues](https://github.com/0xCryptoZen/x-sdks/issues)
- **Discussions:** [GitHub Discussions](https://github.com/0xCryptoZen/x-sdks/discussions)
- **Twitter API:** [Developer Portal](https://developer.twitter.com/)

## License

MIT License - see [LICENSE](https://github.com/0xCryptoZen/x-sdks/blob/main/LICENSE) for details.

## Acknowledgments

- Inspired by [twitter-v2-rs](https://github.com/jpopesculian/twitter-v2-rs)
- Inspired by [node-twitter-api-v2](https://github.com/PLhery/node-twitter-api-v2)
- Built with [Claude Code](https://claude.com/claude-code)

---

**Ready to get started?** Choose your language:
- [🦀 Rust SDK →](./rust-sdk/README.html)
- [🚀 TypeScript SDK →](./typescript-sdk/README.html)
