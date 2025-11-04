use crate::client::XClient;
use crate::types::{TweetRequest, TweetResponse, XResult};
use reqwest::Method;

/// Tweets endpoint API
pub struct Tweets<'a> {
    client: &'a XClient,
}

impl<'a> Tweets<'a> {
    /// Create a new Tweets API instance
    pub(crate) fn new(client: &'a XClient) -> Self {
        Self { client }
    }

    /// Post a new tweet
    ///
    /// # Arguments
    /// * `request` - The tweet request with text content
    ///
    /// # Returns
    /// * `Ok(TweetResponse)` - The created tweet
    /// * `Err(XError)` - If the request fails
    ///
    /// # Example
    /// ```no_run
    /// use x_sdk::{XClient, Credentials, TweetRequest};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials::new(
    ///         "app_key",
    ///         "app_secret",
    ///         "access_token",
    ///         "access_secret"
    ///     );
    ///
    ///     let client = XClient::new(credentials).unwrap();
    ///     let tweet = TweetRequest::new("Hello, world!").unwrap();
    ///
    ///     match client.tweets().post(tweet).await {
    ///         Ok(response) => println!("Tweet posted: {}", response.data.id),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn post(&self, request: TweetRequest) -> XResult<TweetResponse> {
        let url = format!("{}/2/tweets", self.client.base_url());

        // Serialize the request body
        let body = serde_json::to_string(&request)?;

        // Build the HTTP request
        let http_request = self
            .client
            .http_client()
            .request(Method::POST, &url)
            .header("Content-Type", "application/json")
            .body(body)
            .build()?;

        // Execute the request with OAuth signing
        let response = self.client.execute(http_request).await?;

        // Parse the response
        let tweet_response: TweetResponse = response.json().await?;

        Ok(tweet_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Credentials;

    fn test_client() -> XClient {
        let credentials = Credentials::new("key", "secret", "token", "token_secret");
        XClient::new(credentials).unwrap()
    }

    #[test]
    fn test_tweets_endpoint_creation() {
        let client = test_client();
        let _tweets = client.tweets();
        // Just testing that we can create the endpoint
    }
}
