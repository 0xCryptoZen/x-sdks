import axios, { AxiosInstance, AxiosError } from 'axios';
import { Credentials, OAuthClient } from './auth';
import { Tweets } from './endpoints';
import { XError, XErrorCode } from './types';

/**
 * Configuration options for XClient
 */
export interface XClientConfig {
  /**
   * OAuth 1.0a credentials
   */
  credentials: Credentials;

  /**
   * Base URL for the API (default: https://api.twitter.com)
   */
  baseUrl?: string;

  /**
   * Request timeout in milliseconds (default: 30000)
   */
  timeout?: number;

  /**
   * Custom axios instance (for advanced use cases)
   */
  httpClient?: AxiosInstance;
}

/**
 * Main client for interacting with the X (Twitter) API
 */
export class XClient {
  private httpClient: AxiosInstance;
  private oauthClient: OAuthClient;
  private baseUrl: string;

  /**
   * Default API base URL
   */
  static readonly DEFAULT_BASE_URL = 'https://api.twitter.com';

  /**
   * Default request timeout in milliseconds
   */
  static readonly DEFAULT_TIMEOUT = 30000;

  /**
   * Create a new X client
   * @param config - Client configuration
   */
  constructor(config: XClientConfig) {
    this.baseUrl = config.baseUrl || XClient.DEFAULT_BASE_URL;
    this.oauthClient = new OAuthClient(config.credentials);

    this.httpClient =
      config.httpClient ||
      axios.create({
        baseURL: this.baseUrl,
        timeout: config.timeout || XClient.DEFAULT_TIMEOUT,
        headers: {
          'Content-Type': 'application/json',
          'User-Agent': 'x-sdk-typescript/0.1.0',
        },
      });

    // Add request interceptor to sign requests
    this.httpClient.interceptors.request.use((config) => {
      const url = config.url ? `${this.baseUrl}${config.url}` : this.baseUrl;
      const method = config.method || 'GET';

      // Add OAuth authorization header
      config.headers.Authorization = this.oauthClient.getAuthHeader(method, url);

      return config;
    });

    // Add response interceptor to handle errors
    this.httpClient.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => {
        throw this.handleError(error);
      }
    );
  }

  /**
   * Get tweets endpoint API
   */
  tweets(): Tweets {
    return new Tweets(this);
  }

  /**
   * Get the HTTP client for making requests
   * @internal
   */
  getHttpClient(): AxiosInstance {
    return this.httpClient;
  }

  /**
   * Handle HTTP errors and convert to XError
   * @param error - Axios error
   * @returns XError
   */
  private handleError(error: AxiosError): XError {
    if (error.response) {
      // Server responded with error status
      const statusCode = error.response.status;
      const data = error.response.data as Record<string, unknown>;

      // Extract error message from response
      let message = 'API request failed';
      if (data && typeof data === 'object') {
        if ('detail' in data && typeof data.detail === 'string') {
          message = data.detail;
        } else if ('title' in data && typeof data.title === 'string') {
          message = data.title;
        } else if ('message' in data && typeof data.message === 'string') {
          message = data.message;
        }
      }

      // Handle specific status codes
      switch (statusCode) {
        case 401:
        case 403:
          return new XError({
            code: XErrorCode.AUTHENTICATION_FAILED,
            message: `Authentication failed: ${message}`,
            statusCode,
          });

        case 400:
        case 422:
          return new XError({
            code: XErrorCode.INVALID_REQUEST,
            message: `Invalid request: ${message}`,
            statusCode,
          });

        case 429: {
          // Rate limit exceeded
          const retryAfter = error.response.headers['x-rate-limit-reset']
            ? parseInt(error.response.headers['x-rate-limit-reset'], 10)
            : undefined;

          return new XError({
            code: XErrorCode.RATE_LIMIT_EXCEEDED,
            message: `Rate limit exceeded: ${message}`,
            statusCode,
            retryAfter,
          });
        }

        default:
          return new XError({
            code: XErrorCode.API_ERROR,
            message,
            statusCode,
          });
      }
    } else if (error.request) {
      // Request was made but no response received
      return new XError(
        {
          code: XErrorCode.NETWORK_ERROR,
          message: 'Network error: No response received from server',
        },
        error
      );
    } else {
      // Something else happened
      return new XError(
        {
          code: XErrorCode.UNKNOWN_ERROR,
          message: error.message || 'Unknown error occurred',
        },
        error
      );
    }
  }
}
