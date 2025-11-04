//! # X SDK for Rust
//!
//! A Rust SDK for the Twitter/X API v2 with OAuth 1.0a authentication support.
//!
//! ## Features
//!
//! - OAuth 1.0a authentication
//! - Post tweets (POST /2/tweets)
//! - Type-safe request/response handling
//! - Comprehensive error handling
//! - Async/await support with tokio
//!
//! ## Quick Start
//!
//! ```no_run
//! use x_sdk::{XClient, Credentials, TweetRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create credentials
//!     let credentials = Credentials::new(
//!         "your_app_key",
//!         "your_app_secret",
//!         "your_access_token",
//!         "your_access_secret"
//!     );
//!
//!     // Create client
//!     let client = XClient::new(credentials)?;
//!
//!     // Post a tweet
//!     let tweet = TweetRequest::new("Hello from Rust! 🦀")?;
//!     let response = client.tweets().post(tweet).await?;
//!
//!     println!("Tweet posted! ID: {}", response.data.id);
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All operations return `Result<T, XError>` where `XError` provides detailed
//! error information including HTTP status codes, API error messages, and retry
//! information for rate limiting.
//!
//! ```no_run
//! use x_sdk::{XClient, Credentials, TweetRequest, XError};
//!
//! #[tokio::main]
//! async fn main() {
//!     let credentials = Credentials::new("key", "secret", "token", "token_secret");
//!     let client = XClient::new(credentials).unwrap();
//!     let tweet = TweetRequest::new("Test tweet").unwrap();
//!
//!     match client.tweets().post(tweet).await {
//!         Ok(response) => println!("Success: {}", response.data.id),
//!         Err(XError::RateLimitExceeded { retry_after, message }) => {
//!             eprintln!("Rate limited: {}", message);
//!             if let Some(seconds) = retry_after {
//!                 eprintln!("Retry after {} seconds", seconds);
//!             }
//!         }
//!         Err(XError::AuthenticationFailed(msg)) => {
//!             eprintln!("Auth failed: {}", msg);
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

mod auth;
mod client;
mod endpoints;
mod types;

// Re-export public API
pub use auth::Credentials;
pub use client::{XClient, XClientBuilder};
pub use types::{TweetData, TweetRequest, TweetResponse, XError, XResult};
