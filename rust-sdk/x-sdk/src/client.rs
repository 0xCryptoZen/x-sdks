use crate::auth::{Credentials, OAuth};
use crate::endpoints::Tweets;
use crate::types::{XError, XResult};
use reqwest::{Client as HttpClient, Request};
use std::time::Duration;

/// X (Twitter) API client
#[derive(Clone)]
pub struct XClient {
    http_client: HttpClient,
    oauth: OAuth,
    base_url: String,
}

impl XClient {
    /// Default API base URL
    pub const DEFAULT_BASE_URL: &'static str = "https://api.twitter.com";

    /// Default request timeout in seconds
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Create a new X client with credentials
    ///
    /// # Arguments
    /// * `credentials` - OAuth 1.0a credentials
    ///
    /// # Example
    /// ```no_run
    /// use x_twitter_sdk::{XClient, Credentials};
    ///
    /// let credentials = Credentials::new(
    ///     "app_key",
    ///     "app_secret",
    ///     "access_token",
    ///     "access_secret"
    /// );
    ///
    /// let client = XClient::new(credentials).unwrap();
    /// ```
    pub fn new(credentials: Credentials) -> XResult<Self> {
        Self::builder(credentials).build()
    }

    /// Create a new client builder
    ///
    /// # Arguments
    /// * `credentials` - OAuth 1.0a credentials
    pub fn builder(credentials: Credentials) -> XClientBuilder {
        XClientBuilder::new(credentials)
    }

    /// Get tweets endpoint API
    pub fn tweets(&self) -> Tweets<'_> {
        Tweets::new(self)
    }

    /// Execute a request with OAuth signing
    pub(crate) async fn execute(&self, mut request: Request) -> XResult<reqwest::Response> {
        // Add User-Agent header
        request.headers_mut().insert(
            "User-Agent",
            "x-sdk-rust/0.1.0"
                .parse()
                .map_err(|e| XError::Unknown(format!("Invalid User-Agent: {}", e)))?,
        );

        // Sign the request with OAuth
        self.oauth.sign_request(&mut request)?;

        // Execute the request
        let response = self.http_client.execute(request).await?;

        // Check for HTTP errors
        let status = response.status();
        if !status.is_success() {
            return Err(self.handle_error_response(response).await);
        }

        Ok(response)
    }

    /// Handle error responses from the API
    async fn handle_error_response(&self, response: reqwest::Response) -> XError {
        let status = response.status();
        let status_code = status.as_u16();

        // Try to get retry-after header for rate limiting
        let retry_after = response
            .headers()
            .get("x-rate-limit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        // Try to get the error message from response body
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        match status_code {
            401 | 403 => XError::AuthenticationFailed(error_message),
            400 | 422 => XError::InvalidRequest(error_message),
            429 => XError::RateLimitExceeded {
                retry_after,
                message: error_message,
            },
            _ => XError::ApiError {
                code: status_code,
                message: error_message,
            },
        }
    }

    /// Get the base URL for API requests
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get a reference to the HTTP client
    pub(crate) fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

/// Builder for XClient
pub struct XClientBuilder {
    credentials: Credentials,
    base_url: Option<String>,
    timeout: Option<Duration>,
    http_client: Option<HttpClient>,
}

impl XClientBuilder {
    /// Create a new builder
    fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            base_url: None,
            timeout: None,
            http_client: None,
        }
    }

    /// Set custom base URL (for testing or custom endpoints)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set custom HTTP client
    pub fn http_client(mut self, client: HttpClient) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Build the XClient
    pub fn build(self) -> XResult<XClient> {
        let oauth = OAuth::new(self.credentials)?;

        let http_client = self.http_client.unwrap_or_else(|| {
            HttpClient::builder()
                .timeout(
                    self.timeout
                        .unwrap_or_else(|| Duration::from_secs(XClient::DEFAULT_TIMEOUT_SECS)),
                )
                .build()
                .expect("Failed to build HTTP client")
        });

        Ok(XClient {
            http_client,
            oauth,
            base_url: self
                .base_url
                .unwrap_or_else(|| XClient::DEFAULT_BASE_URL.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_credentials() -> Credentials {
        Credentials::new("test_key", "test_secret", "test_token", "test_token_secret")
    }

    #[test]
    fn test_client_new() {
        let credentials = test_credentials();
        let client = XClient::new(credentials);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_builder() {
        let credentials = test_credentials();
        let client = XClient::builder(credentials)
            .base_url("https://test.example.com")
            .timeout(Duration::from_secs(10))
            .build();

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url(), "https://test.example.com");
    }

    #[test]
    fn test_client_with_invalid_credentials() {
        let credentials = Credentials::new("", "", "", "");
        let client = XClient::new(credentials);
        assert!(client.is_err());
    }
}
