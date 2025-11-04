use serde::Serialize;

/// Request to post a tweet
#[derive(Debug, Clone, Serialize)]
pub struct TweetRequest {
    /// The text content of the tweet (max 280 characters)
    pub text: String,
}

impl TweetRequest {
    /// Create a new tweet request
    ///
    /// # Arguments
    /// * `text` - The tweet content (max 280 characters)
    ///
    /// # Returns
    /// * `Ok(TweetRequest)` if text is valid
    /// * `Err` if text is empty or exceeds 280 characters
    pub fn new(text: impl Into<String>) -> Result<Self, String> {
        let text = text.into();

        if text.is_empty() {
            return Err("Tweet text cannot be empty".to_string());
        }

        if text.chars().count() > 280 {
            return Err(format!(
                "Tweet text exceeds 280 characters (got {})",
                text.chars().count()
            ));
        }

        Ok(Self { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tweet() {
        let tweet = TweetRequest::new("Hello, world!").unwrap();
        assert_eq!(tweet.text, "Hello, world!");
    }

    #[test]
    fn test_empty_tweet() {
        let result = TweetRequest::new("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_tweet_too_long() {
        let long_text = "a".repeat(281);
        let result = TweetRequest::new(long_text);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds 280 characters"));
    }

    #[test]
    fn test_tweet_exactly_280_chars() {
        let text = "a".repeat(280);
        let tweet = TweetRequest::new(text).unwrap();
        assert_eq!(tweet.text.chars().count(), 280);
    }
}
