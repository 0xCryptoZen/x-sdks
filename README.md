# X (Twitter) SDKs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Tests](https://img.shields.io/badge/rust%20tests-13%2F13%20passing-brightgreen)](./rust-sdk)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://0xCryptoZen.github.io/x-sdks)

Dual-language SDKs for Twitter/X API v2 with OAuth 1.0a authentication. Built with unified interfaces for **Rust** and **TypeScript**.

## 🌟 Features

- ✅ **OAuth 1.0a Authentication** - Automatic request signing
- ✅ **Tweet Posting** - POST /2/tweets endpoint
- ✅ **Type Safety** - Strong typing in both languages
- ✅ **Error Handling** - Comprehensive error types with retry information
- ✅ **Unified API** - Identical interfaces across languages
- ✅ **Production Ready** - Comprehensive tests and documentation

## 📦 SDKs

### Rust SDK

Type-safe, async SDK built with reqwest and tokio.

```rust
use x_sdk::{XClient, Credentials, TweetRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::new(
        "app_key",
        "app_secret",
        "access_token",
        "access_secret"
    );

    let client = XClient::new(credentials)?;
    let tweet = TweetRequest::new("Hello from Rust! 🦀")?;
    let response = client.tweets().post(tweet).await?;

    println!("Tweet posted! ID: {}", response.data.id);
    Ok(())
}
```

**[📚 Rust SDK Documentation →](./rust-sdk/README.md)**

### TypeScript SDK

Fully typed SDK built with axios and modern TypeScript.

```typescript
import { XClient, createTweetRequest } from '@zen_tools/x-sdk';

async function main() {
  const client = new XClient({
    credentials: {
      appKey: 'app_key',
      appSecret: 'app_secret',
      accessToken: 'access_token',
      accessSecret: 'access_secret',
    },
  });

  const tweet = createTweetRequest('Hello from TypeScript! 🚀');
  const response = await client.tweets().post(tweet);

  console.log('Tweet posted! ID:', response.data.id);
}
```

**[📚 TypeScript SDK Documentation →](./typescript-sdk/README.md)**

## 🚀 Quick Start

### Installation

**Rust:**
```toml
[dependencies]
x_sdk = { git = "https://github.com/0xCryptoZen/x-sdks", subdirectory = "rust-sdk/x-sdk" }
tokio = { version = "1.0", features = ["full"] }
```

**TypeScript:**
```bash
npm install @zen_tools/x-sdk
# or
yarn add @zen_tools/x-sdk
```

### Authentication

Both SDKs require Twitter/X API credentials. Get yours from the [Twitter Developer Portal](https://developer.twitter.com/).

**Required credentials:**
- App Key (Consumer Key)
- App Secret (Consumer Secret)
- Access Token
- Access Token Secret

**Environment Variables:**
```bash
export X_APP_KEY="your_app_key"
export X_APP_SECRET="your_app_secret"
export X_ACCESS_TOKEN="your_access_token"
export X_ACCESS_SECRET="your_access_secret"
```

### Basic Usage

**Rust:**
```rust
use x_sdk::{XClient, Credentials, TweetRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::new(
        env::var("X_APP_KEY")?,
        env::var("X_APP_SECRET")?,
        env::var("X_ACCESS_TOKEN")?,
        env::var("X_ACCESS_SECRET")?
    );

    let client = XClient::new(credentials)?;
    let tweet = TweetRequest::new("My first tweet!")?;
    let response = client.tweets().post(tweet).await?;

    println!("✅ Tweet posted: {}", response.data.id);
    Ok(())
}
```

**TypeScript:**
```typescript
import { XClient, createTweetRequest } from '@zen_tools/x-sdk';

const client = new XClient({
  credentials: {
    appKey: process.env.X_APP_KEY!,
    appSecret: process.env.X_APP_SECRET!,
    accessToken: process.env.X_ACCESS_TOKEN!,
    accessSecret: process.env.X_ACCESS_SECRET!,
  },
});

const tweet = createTweetRequest('My first tweet!');
const response = await client.tweets().post(tweet);
console.log('✅ Tweet posted:', response.data.id);
```

## 🎯 API Consistency

Both SDKs follow identical patterns:

| Operation | Rust | TypeScript |
|-----------|------|------------|
| **Initialize Client** | `XClient::new(credentials)?` | `new XClient({ credentials })` |
| **Post Tweet** | `client.tweets().post(tweet).await?` | `await client.tweets().post(tweet)` |
| **Create Tweet** | `TweetRequest::new(text)?` | `createTweetRequest(text)` |
| **Error Handling** | `Result<T, XError>` | `try/catch XError` |

## 📖 Documentation

- **[API Reference](./API_DOCUMENTATION.md)** - Unified API documentation
- **[Rust SDK Guide](./rust-sdk/README.md)** - Rust-specific documentation
- **[TypeScript SDK Guide](./typescript-sdk/README.md)** - TypeScript-specific documentation
- **[GitHub Pages](https://0xCryptoZen.github.io/x-sdks)** - Full documentation site

## 🧪 Testing

### Rust
```bash
cd rust-sdk
cargo test
```

**Test Results:** 13/13 passing ✅

### TypeScript
```bash
cd typescript-sdk
npm test
```

## 🛠️ Advanced Configuration

### Rust

```rust
use x_sdk::XClient;
use std::time::Duration;

let client = XClient::builder(credentials)
    .timeout(Duration::from_secs(60))
    .base_url("https://api.twitter.com")
    .build()?;
```

### TypeScript

```typescript
const client = new XClient({
  credentials,
  timeout: 60000,
  baseUrl: 'https://api.twitter.com',
});
```

## ⚠️ Error Handling

Both SDKs provide comprehensive error handling:

**Error Categories:**
- `AuthenticationFailed` - Invalid credentials (401, 403)
- `RateLimitExceeded` - Too many requests (429)
- `InvalidRequest` - Bad request (400, 422)
- `ApiError` - Server errors (5xx)
- `NetworkError` - Connection issues

**Rust Example:**
```rust
match client.tweets().post(tweet).await {
    Ok(response) => println!("Success: {}", response.data.id),
    Err(XError::RateLimitExceeded { retry_after, .. }) => {
        if let Some(seconds) = retry_after {
            println!("Rate limited. Retry after {} seconds", seconds);
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

**TypeScript Example:**
```typescript
try {
  const response = await client.tweets().post(tweet);
  console.log('Success:', response.data.id);
} catch (error) {
  if (error instanceof XError) {
    if (error.code === XErrorCode.RATE_LIMIT_EXCEEDED) {
      const retryAfter = error.getRetryDelay();
      console.log(`Rate limited. Retry after ${retryAfter} seconds`);
    }
  }
}
```

## 🏗️ Project Structure

```
x-sdks/
├── README.md                    # This file
├── API_DOCUMENTATION.md         # Unified API reference
├── rust-sdk/                    # Rust implementation
│   ├── Cargo.toml
│   ├── x-sdk/
│   │   ├── src/
│   │   ├── tests/
│   │   └── Cargo.toml
│   ├── examples/
│   └── README.md
├── typescript-sdk/              # TypeScript implementation
│   ├── package.json
│   ├── src/
│   ├── tests/
│   ├── examples/
│   └── README.md
├── docs/                        # Documentation site
└── .github/
    └── workflows/
        └── docs.yml             # GitHub Pages deployment
```

## 🤝 Contributing

Contributions are welcome! Please follow the workflow defined in [CLAUDE.md](./CLAUDE.md):

1. Create an issue describing the change
2. Create a feature branch: `feature/#<issue>-description`
3. Implement changes with tests
4. Commit with issue reference: `feat: description (#issue)`
5. Create a pull request

## 📦 Publishing

For maintainers - choose your preferred method:

### 🚀 Quick Release (Simplest)

```bash
# One command publishes both SDKs
./scripts/tag-release.sh v2.0.3
```

### ⚡ Auto Release (With Git Hooks)

```bash
# One-time setup
./scripts/setup-hooks.sh

# Then just tag and push
git tag v2.0.3
git push origin v2.0.3
```

### 📚 Documentation

- **[QUICK_RELEASE.md](./QUICK_RELEASE.md)** - Quick reference for all release methods
- **[RELEASE.md](./RELEASE.md)** - Complete unified release guide
- **[PUBLISHING.md](./PUBLISHING.md)** - TypeScript SDK only
- **[PUBLISHING_RUST.md](./PUBLISHING_RUST.md)** - Rust SDK only

## 📄 License

MIT License - see [LICENSE](./LICENSE) for details.

## 🔗 Links

- **Repository:** https://github.com/0xCryptoZen/x-sdks
- **Documentation:** https://0xCryptoZen.github.io/x-sdks
- **Twitter API v2:** https://developer.twitter.com/en/docs/twitter-api
- **OAuth 1.0a Spec:** https://oauth.net/core/1.0a/

## 🙏 Acknowledgments

- Inspired by [twitter-v2-rs](https://github.com/jpopesculian/twitter-v2-rs) (Rust)
- Inspired by [node-twitter-api-v2](https://github.com/PLhery/node-twitter-api-v2) (TypeScript)

## 📊 Status

- ✅ Rust SDK: Complete with 13 passing tests
- ✅ TypeScript SDK: Complete with full type coverage
- ✅ Documentation: Comprehensive API reference
- ✅ Examples: Working examples for both SDKs

---

**Built with ❤️ using [Claude Code](https://claude.com/claude-code)**
