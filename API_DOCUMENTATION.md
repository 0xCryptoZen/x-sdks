# X SDKs - Unified API Documentation

This document provides a unified API reference for both **Rust** and **TypeScript** X (Twitter) SDKs.

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [API Comparison](#api-comparison)
- [Authentication](#authentication)
- [Client Initialization](#client-initialization)
- [Posting Tweets](#posting-tweets)
- [Error Handling](#error-handling)
- [Advanced Configuration](#advanced-configuration)

---

## 🚀 Installation

### Rust

```toml
[dependencies]
x-sdk = { path = "./rust-sdk/x-sdk" }
tokio = { version = "1.0", features = ["full"] }
```

### TypeScript

```bash
npm install @x-sdks/typescript
```

---

## ⚡ Quick Start

### Rust

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

### TypeScript

```typescript
import { XClient, createTweetRequest } from '@x-sdks/typescript';

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

---

## 📊 API Comparison

### Interface Consistency

Both SDKs follow the same architectural pattern:

| Component | Rust | TypeScript |
|-----------|------|------------|
| **Client** | `XClient` | `XClient` |
| **Credentials** | `Credentials::new(...)` | `{ credentials: {...} }` |
| **Tweet Posting** | `client.tweets().post(...)` | `client.tweets().post(...)` |
| **Error Type** | `XError` enum | `XError` class |
| **Response** | `TweetResponse` struct | `TweetResponse` interface |

### Method Signatures

#### Posting a Tweet

**Rust:**
```rust
pub async fn post(&self, request: TweetRequest) -> XResult<TweetResponse>
```

**TypeScript:**
```typescript
async post(request: TweetRequest): Promise<TweetResponse>
```

Both return the same response structure:
```json
{
  "data": {
    "id": "1234567890",
    "text": "Hello, world!",
    "edit_history_tweet_ids": ["1234567890"]
  }
}
```

---

## 🔑 Authentication

Both SDKs use **OAuth 1.0a** authentication with identical credential requirements:

### Required Credentials

| Field | Rust | TypeScript | Description |
|-------|------|------------|-------------|
| App Key | `app_key: String` | `appKey: string` | Consumer Key |
| App Secret | `app_secret: String` | `appSecret: string` | Consumer Secret |
| Access Token | `access_token: String` | `accessToken: string` | User Token |
| Access Secret | `access_secret: String` | `accessSecret: string` | User Token Secret |

### Rust

```rust
use x_sdk::Credentials;

let credentials = Credentials::new(
    "your_app_key",
    "your_app_secret",
    "your_access_token",
    "your_access_secret"
);
```

### TypeScript

```typescript
import { Credentials } from '@x-sdks/typescript';

const credentials: Credentials = {
  appKey: 'your_app_key',
  appSecret: 'your_app_secret',
  accessToken: 'your_access_token',
  accessSecret: 'your_access_secret',
};
```

---

## 🏗️ Client Initialization

### Basic Initialization

**Rust:**
```rust
use x_sdk::{XClient, Credentials};

let credentials = Credentials::new("key", "secret", "token", "token_secret");
let client = XClient::new(credentials)?;
```

**TypeScript:**
```typescript
import { XClient } from '@x-sdks/typescript';

const client = new XClient({
  credentials: {
    appKey: 'key',
    appSecret: 'secret',
    accessToken: 'token',
    accessSecret: 'token_secret',
  },
});
```

### Advanced Configuration

**Rust:**
```rust
use x_sdk::XClient;
use std::time::Duration;

let client = XClient::builder(credentials)
    .timeout(Duration::from_secs(60))
    .base_url("https://api.twitter.com")
    .build()?;
```

**TypeScript:**
```typescript
import { XClient } from '@x-sdks/typescript';

const client = new XClient({
  credentials,
  timeout: 60000, // milliseconds
  baseUrl: 'https://api.twitter.com',
});
```

---

## 📝 Posting Tweets

### Basic Tweet

**Rust:**
```rust
use x_sdk::TweetRequest;

let tweet = TweetRequest::new("Hello, world!")?;
let response = client.tweets().post(tweet).await?;
println!("Tweet ID: {}", response.data.id);
```

**TypeScript:**
```typescript
import { createTweetRequest } from '@x-sdks/typescript';

const tweet = createTweetRequest('Hello, world!');
const response = await client.tweets().post(tweet);
console.log('Tweet ID:', response.data.id);
```

### Tweet Validation

Both SDKs validate tweets before posting:

- ✅ Text cannot be empty
- ✅ Text cannot exceed 280 characters
- ✅ Validation happens at request creation

**Rust:**
```rust
// This will return an error
let tweet = TweetRequest::new("")?; // Error: Tweet text cannot be empty
let tweet = TweetRequest::new("a".repeat(281))?; // Error: Exceeds 280 characters
```

**TypeScript:**
```typescript
// This will throw an error
createTweetRequest(''); // Error: Tweet text cannot be empty
createTweetRequest('a'.repeat(281)); // Error: Exceeds 280 characters
```

---

## ⚠️ Error Handling

Both SDKs provide comprehensive error handling with the same error categories:

### Error Categories

| Error Type | HTTP Status | Retryable | Description |
|------------|-------------|-----------|-------------|
| **Authentication Failed** | 401, 403 | ❌ | Invalid credentials |
| **Rate Limit Exceeded** | 429 | ✅ | Too many requests |
| **Invalid Request** | 400, 422 | ❌ | Bad request format |
| **API Error** | 5xx | ✅ | Server errors |
| **Network Error** | - | ✅ | Connection issues |

### Rust Error Handling

```rust
use x_sdk::{XError, TweetRequest};

match client.tweets().post(tweet).await {
    Ok(response) => {
        println!("Success: {}", response.data.id);
    }
    Err(XError::RateLimitExceeded { retry_after, message }) => {
        eprintln!("Rate limited: {}", message);
        if let Some(seconds) = retry_after {
            eprintln!("Retry after {} seconds", seconds);
        }
    }
    Err(XError::AuthenticationFailed(msg)) => {
        eprintln!("Auth failed: {}", msg);
    }
    Err(XError::InvalidRequest(msg)) => {
        eprintln!("Invalid request: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}

// Check if error is retryable
if error.is_retryable() {
    if let Some(delay) = error.retry_after() {
        // Wait and retry
    }
}
```

### TypeScript Error Handling

```typescript
import { XError, XErrorCode } from '@x-sdks/typescript';

try {
  const response = await client.tweets().post(tweet);
  console.log('Success:', response.data.id);
} catch (error) {
  if (error instanceof XError) {
    switch (error.code) {
      case XErrorCode.RATE_LIMIT_EXCEEDED:
        console.error('Rate limited:', error.message);
        const retryAfter = error.getRetryDelay();
        if (retryAfter) {
          console.error(`Retry after ${retryAfter} seconds`);
        }
        break;

      case XErrorCode.AUTHENTICATION_FAILED:
        console.error('Auth failed:', error.message);
        break;

      case XErrorCode.INVALID_REQUEST:
        console.error('Invalid request:', error.message);
        break;

      default:
        console.error('Error:', error.message);
    }

    // Check if error is retryable
    if (error.isRetryable()) {
      const delay = error.getRetryDelay();
      // Wait and retry
    }
  }
}
```

---

## 🔧 Advanced Configuration

### Custom HTTP Client

**Rust:**
```rust
use reqwest::Client as HttpClient;
use std::time::Duration;

let http_client = HttpClient::builder()
    .timeout(Duration::from_secs(60))
    .build()?;

let client = XClient::builder(credentials)
    .http_client(http_client)
    .build()?;
```

**TypeScript:**
```typescript
import axios from 'axios';

const httpClient = axios.create({
  timeout: 60000,
  // Custom configuration
});

const client = new XClient({
  credentials,
  httpClient,
});
```

### Environment Variables

Both SDKs support loading credentials from environment variables:

**Rust:**
```rust
use std::env;

let credentials = Credentials::new(
    env::var("X_APP_KEY")?,
    env::var("X_APP_SECRET")?,
    env::var("X_ACCESS_TOKEN")?,
    env::var("X_ACCESS_SECRET")?
);
```

**TypeScript:**
```typescript
const credentials = {
  appKey: process.env.X_APP_KEY!,
  appSecret: process.env.X_APP_SECRET!,
  accessToken: process.env.X_ACCESS_TOKEN!,
  accessSecret: process.env.X_ACCESS_SECRET!,
};
```

---

## 📚 Response Types

### TweetResponse

Both SDKs return identical response structures:

**Rust:**
```rust
pub struct TweetResponse {
    pub data: TweetData,
}

pub struct TweetData {
    pub id: String,
    pub text: String,
    pub edit_history_tweet_ids: Option<Vec<String>>,
}
```

**TypeScript:**
```typescript
interface TweetResponse {
  data: TweetData;
}

interface TweetData {
  id: string;
  text: string;
  edit_history_tweet_ids?: string[];
}
```

### Getting Tweet URL

**Rust:**
```rust
let url = response.tweet_url("username");
// Returns: "https://twitter.com/username/status/1234567890"
```

**TypeScript:**
```typescript
import { getTweetUrl } from '@x-sdks/typescript';

const url = getTweetUrl(response, 'username');
// Returns: "https://twitter.com/username/status/1234567890"
```

---

## 🧪 Testing

### Running Tests

**Rust:**
```bash
cd rust-sdk
cargo test
cargo test -- --nocapture  # With output
```

**TypeScript:**
```bash
cd typescript-sdk
npm test
npm run test:coverage  # With coverage
```

---

## 📦 Building

### Rust

```bash
cd rust-sdk
cargo build --release
```

### TypeScript

```bash
cd typescript-sdk
npm run build
```

---

## 🎯 Design Principles

Both SDKs follow the same design principles:

1. **Type Safety**: Strong typing in both languages
2. **Error Handling**: Comprehensive error types with retry information
3. **OAuth 1.0a**: Automatic request signing
4. **Async/Await**: Modern async patterns
5. **Extensibility**: Easy to add new endpoints
6. **Validation**: Input validation before API calls
7. **Consistency**: Identical interfaces across languages

---

## 📖 References

- [Twitter API v2 Documentation](https://developer.twitter.com/en/docs/twitter-api)
- [OAuth 1.0a Specification](https://oauth.net/core/1.0a/)
- [Rust SDK - twitter-v2-rs](https://github.com/jpopesculian/twitter-v2-rs)
- [TypeScript SDK - node-twitter-api-v2](https://github.com/PLhery/node-twitter-api-v2)

---

## 📄 License

MIT License - Both SDKs
