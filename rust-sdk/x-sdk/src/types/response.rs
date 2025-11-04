use serde::Deserialize;

/// Response from posting a tweet
#[derive(Debug, Clone, Deserialize)]
pub struct TweetResponse {
    /// The tweet data
    pub data: TweetData,
}

/// Tweet data from the API
#[derive(Debug, Clone, Deserialize)]
pub struct TweetData {
    /// Unique identifier of the tweet
    pub id: String,

    /// The actual UTF-8 text of the tweet
    pub text: String,

    /// Edit history tweet IDs (if tweet was edited)
    #[serde(default)]
    pub edit_history_tweet_ids: Option<Vec<String>>,
}

impl TweetResponse {
    /// Get the tweet URL
    ///
    /// # Arguments
    /// * `username` - The username of the account that posted the tweet
    ///
    /// # Returns
    /// The full URL to the tweet
    pub fn tweet_url(&self, username: &str) -> String {
        format!("https://twitter.com/{}/status/{}", username, self.data.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_tweet_response() {
        let json = r#"
        {
            "data": {
                "id": "1234567890",
                "text": "Hello, world!"
            }
        }
        "#;

        let response: TweetResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.id, "1234567890");
        assert_eq!(response.data.text, "Hello, world!");
        assert!(response.data.edit_history_tweet_ids.is_none());
    }

    #[test]
    fn test_tweet_url() {
        let response = TweetResponse {
            data: TweetData {
                id: "1234567890".to_string(),
                text: "Test".to_string(),
                edit_history_tweet_ids: None,
            },
        };

        let url = response.tweet_url("testuser");
        assert_eq!(url, "https://twitter.com/testuser/status/1234567890");
    }
}
