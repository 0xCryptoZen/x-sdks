use crate::types::XError;
use oauth1_request::signature_method::hmac_sha1::HmacSha1;
use oauth1_request::{Credentials as OAuthCredentials, Token as OAuthToken};
use reqwest::Request;

/// OAuth 1.0a credentials for Twitter/X API
#[derive(Debug, Clone)]
pub struct Credentials {
    /// App Key (Consumer Key)
    pub app_key: String,
    /// App Secret (Consumer Secret)
    pub app_secret: String,
    /// Access Token
    pub access_token: String,
    /// Access Token Secret
    pub access_secret: String,
}

impl Credentials {
    /// Create new credentials
    pub fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
        access_secret: impl Into<String>,
    ) -> Self {
        Self {
            app_key: app_key.into(),
            app_secret: app_secret.into(),
            access_token: access_token.into(),
            access_secret: access_secret.into(),
        }
    }

    /// Validate that all credentials are non-empty
    pub fn validate(&self) -> Result<(), XError> {
        if self.app_key.is_empty()
            || self.app_secret.is_empty()
            || self.access_token.is_empty()
            || self.access_secret.is_empty()
        {
            return Err(XError::AuthenticationFailed(
                "All credentials must be non-empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// OAuth 1.0a signature generator
#[derive(Clone)]
pub struct OAuth {
    credentials: Credentials,
}

impl OAuth {
    /// Create a new OAuth instance
    pub fn new(credentials: Credentials) -> Result<Self, XError> {
        credentials.validate()?;
        Ok(Self { credentials })
    }

    /// Sign a request with OAuth 1.0a
    pub fn sign_request(&self, request: &mut Request) -> Result<(), XError> {
        let method = request.method().as_str();
        let url = request.url().as_str();

        // Convert to oauth1_request types
        let client = OAuthCredentials::new(&self.credentials.app_key, &self.credentials.app_secret);
        let token_creds = OAuthCredentials::new(
            &self.credentials.access_token,
            &self.credentials.access_secret,
        );
        let token = OAuthToken::new(client, token_creds);

        // Generate OAuth authorization header with empty request parameters
        let auth_header = oauth1_request::authorize(method, url, &(), &token, HmacSha1::new());

        // Add the authorization header to the request
        request.headers_mut().insert(
            "Authorization",
            auth_header.parse().map_err(|e| {
                XError::OAuthError(format!("Failed to parse authorization header: {}", e))
            })?,
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_new() {
        let creds = Credentials::new("key", "secret", "token", "token_secret");
        assert_eq!(creds.app_key, "key");
        assert_eq!(creds.app_secret, "secret");
        assert_eq!(creds.access_token, "token");
        assert_eq!(creds.access_secret, "token_secret");
    }

    #[test]
    fn test_credentials_validate_success() {
        let creds = Credentials::new("key", "secret", "token", "token_secret");
        assert!(creds.validate().is_ok());
    }

    #[test]
    fn test_credentials_validate_empty() {
        let creds = Credentials::new("", "secret", "token", "token_secret");
        assert!(creds.validate().is_err());
    }
}
