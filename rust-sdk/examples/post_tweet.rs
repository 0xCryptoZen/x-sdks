use std::env;
use x_sdk::{Credentials, TweetRequest, XClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment variables
    let credentials = Credentials::new(
        env::var("X_APP_KEY").expect("X_APP_KEY not set"),
        env::var("X_APP_SECRET").expect("X_APP_SECRET not set"),
        env::var("X_ACCESS_TOKEN").expect("X_ACCESS_TOKEN not set"),
        env::var("X_ACCESS_SECRET").expect("X_ACCESS_SECRET not set"),
    );

    // Create the X client
    let client = XClient::new(credentials)?;

    // Create a tweet
    let tweet_text = env::args()
        .nth(1)
        .unwrap_or_else(|| "Hello from x-sdk-rust! 🦀".to_string());

    let tweet = TweetRequest::new(tweet_text)?;

    // Post the tweet
    println!("Posting tweet...");
    match client.tweets().post(tweet).await {
        Ok(response) => {
            println!("✅ Tweet posted successfully!");
            println!("   ID: {}", response.data.id);
            println!("   Text: {}", response.data.text);
            println!("   URL: {}", response.tweet_url("your_username"));
        }
        Err(e) => {
            eprintln!("❌ Error posting tweet: {}", e);
            if e.is_retryable() {
                eprintln!("   This error is retryable");
                if let Some(retry_after) = e.retry_after() {
                    eprintln!("   Retry after {} seconds", retry_after);
                }
            }
            return Err(e.into());
        }
    }

    Ok(())
}
