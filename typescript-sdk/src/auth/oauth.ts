import OAuth from 'oauth-1.0a';
import crypto from 'crypto';
import { XError, XErrorCode } from '../types';

/**
 * OAuth 1.0a credentials for Twitter/X API
 */
export interface Credentials {
  /**
   * App Key (Consumer Key)
   */
  appKey: string;

  /**
   * App Secret (Consumer Secret)
   */
  appSecret: string;

  /**
   * Access Token
   */
  accessToken: string;

  /**
   * Access Token Secret
   */
  accessSecret: string;
}

/**
 * Validate OAuth credentials
 * @param credentials - The credentials to validate
 * @throws XError if credentials are invalid
 */
export function validateCredentials(credentials: Credentials): void {
  const { appKey, appSecret, accessToken, accessSecret } = credentials;

  if (!appKey || !appSecret || !accessToken || !accessSecret) {
    throw new XError({
      code: XErrorCode.AUTHENTICATION_FAILED,
      message: 'All credentials must be non-empty',
    });
  }
}

/**
 * OAuth 1.0a client for signing requests
 */
export class OAuthClient {
  private oauth: OAuth;
  private credentials: Credentials;

  constructor(credentials: Credentials) {
    validateCredentials(credentials);
    this.credentials = credentials;

    this.oauth = new OAuth({
      consumer: {
        key: credentials.appKey,
        secret: credentials.appSecret,
      },
      signature_method: 'HMAC-SHA1',
      hash_function(baseString: string, key: string): string {
        return crypto.createHmac('sha1', key).update(baseString).digest('base64');
      },
    });
  }

  /**
   * Generate OAuth authorization header for a request
   * @param method - HTTP method (GET, POST, etc.)
   * @param url - Request URL
   * @returns Authorization header value
   */
  getAuthHeader(method: string, url: string): string {
    const token = {
      key: this.credentials.accessToken,
      secret: this.credentials.accessSecret,
    };

    const authData = this.oauth.authorize(
      {
        url,
        method: method.toUpperCase(),
      },
      token
    );

    return this.oauth.toHeader(authData).Authorization;
  }
}
