# X SDK for Rust

A Rust SDK for the Twitter/X API v2 with OAuth 1.0a authentication support.

## Features

- ✅ OAuth 1.0a authentication with automatic request signing
- ✅ Post tweets (POST /2/tweets)
- ✅ Type-safe request/response handling
- ✅ Comprehensive error handling with retry information
- ✅ Async/await support with tokio
- ✅ Built on reqwest for reliable HTTP handling

## Installation

### From crates.io (Recommended)

Add this to your `Cargo.toml`:

```toml
[dependencies]
x-twitter-sdk = "2.0.8"
tokio = { version = "1.0", features = ["full"] }
```

Or install via command line:

```bash
cargo add x-twitter-sdk
cargo add tokio --features full
```

### From GitHub (Development Version)

```toml
[dependencies]
x-twitter-sdk = { git = "https://github.com/0xCryptoZen/x-sdks", subdirectory = "rust-sdk/x-sdk" }
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use x_sdk::{XClient, Credentials, TweetRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials
    let credentials = Credentials::new(
        "your_app_key",
        "your_app_secret",
        "your_access_token",
        "your_access_secret"
    );

    // Create client
    let client = XClient::new(credentials)?;

    // Post a tweet
    let tweet = TweetRequest::new("Hello from Rust! 🦀")?;
    let response = client.tweets().post(tweet).await?;

    println!("Tweet posted! ID: {}", response.data.id);
    Ok(())
}
```

## Examples

### Basic Tweet Posting

```rust
use x_sdk::{XClient, Credentials, TweetRequest};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new(
        std::env::var("X_APP_KEY").unwrap(),
        std::env::var("X_APP_SECRET").unwrap(),
        std::env::var("X_ACCESS_TOKEN").unwrap(),
        std::env::var("X_ACCESS_SECRET").unwrap(),
    );

    let client = XClient::new(credentials).unwrap();
    let tweet = TweetRequest::new("Hello, X!").unwrap();

    match client.tweets().post(tweet).await {
        Ok(response) => {
            println!("Tweet ID: {}", response.data.id);
            println!("Tweet text: {}", response.data.text);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Error Handling

```rust
use x_sdk::{XClient, Credentials, TweetRequest, XError};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new("key", "secret", "token", "token_secret");
    let client = XClient::new(credentials).unwrap();
    let tweet = TweetRequest::new("Test").unwrap();

    match client.tweets().post(tweet).await {
        Ok(response) => println!("Success: {}", response.data.id),

        Err(XError::RateLimitExceeded { retry_after, message }) => {
            eprintln!("Rate limited: {}", message);
            if let Some(seconds) = retry_after {
                eprintln!("Retry after {} seconds", seconds);
            }
        }

        Err(XError::AuthenticationFailed(msg)) => {
            eprintln!("Authentication failed: {}", msg);
        }

        Err(XError::InvalidRequest(msg)) => {
            eprintln!("Invalid request: {}", msg);
        }

        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Custom Configuration

```rust
use x_sdk::{XClient, Credentials};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let credentials = Credentials::new("key", "secret", "token", "token_secret");

    let client = XClient::builder(credentials)
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap();
}
```

## Running Examples

Set your credentials as environment variables:

```bash
export X_APP_KEY="your_app_key"
export X_APP_SECRET="your_app_secret"
export X_ACCESS_TOKEN="your_access_token"
export X_ACCESS_SECRET="your_access_secret"
```

Run the example:

```bash
cd rust-sdk
cargo run --example post_tweet "Your tweet text here"
```

## API Documentation

### `Credentials`

Holds OAuth 1.0a credentials for Twitter/X API.

```rust
let credentials = Credentials::new(
    "app_key",
    "app_secret",
    "access_token",
    "access_secret"
);
```

### `XClient`

Main client for interacting with the X API.

```rust
// Simple creation
let client = XClient::new(credentials)?;

// With custom configuration
let client = XClient::builder(credentials)
    .timeout(Duration::from_secs(30))
    .base_url("https://api.twitter.com")
    .build()?;
```

### `TweetRequest`

Request to post a tweet.

```rust
let tweet = TweetRequest::new("Hello, world!")?;
```

Validates that:
- Text is not empty
- Text does not exceed 280 characters

### `TweetResponse`

Response from posting a tweet.

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

### Error Handling

All operations return `Result<T, XError>`:

```rust
pub enum XError {
    AuthenticationFailed(String),
    RateLimitExceeded { retry_after: Option<u64>, message: String },
    InvalidRequest(String),
    ApiError { code: u16, message: String },
    NetworkError(reqwest::Error),
    JsonError(serde_json::Error),
    OAuthError(String),
    Unknown(String),
}
```

Check if an error is retryable:

```rust
if error.is_retryable() {
    if let Some(seconds) = error.retry_after() {
        // Wait and retry
    }
}
```

## Testing

Run the test suite:

```bash
cd rust-sdk/x-sdk
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## License

MIT

## References

- [Twitter API v2 Documentation](https://developer.twitter.com/en/docs/twitter-api)
- [OAuth 1.0a Specification](https://oauth.net/core/1.0a/)
