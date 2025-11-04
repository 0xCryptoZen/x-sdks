/**
 * Error codes for X SDK operations
 */
export enum XErrorCode {
  AUTHENTICATION_FAILED = 'AUTHENTICATION_FAILED',
  RATE_LIMIT_EXCEEDED = 'RATE_LIMIT_EXCEEDED',
  INVALID_REQUEST = 'INVALID_REQUEST',
  NETWORK_ERROR = 'NETWORK_ERROR',
  API_ERROR = 'API_ERROR',
  UNKNOWN_ERROR = 'UNKNOWN_ERROR',
}

/**
 * Details about an X SDK error
 */
export interface XErrorDetails {
  code: XErrorCode;
  message: string;
  statusCode?: number;
  apiError?: {
    title?: string;
    detail?: string;
    type?: string;
  };
  retryAfter?: number;
}

/**
 * Custom error class for X SDK operations
 */
export class XError extends Error {
  public readonly code: XErrorCode;
  public readonly details: XErrorDetails;
  public readonly cause?: Error;

  constructor(details: XErrorDetails, cause?: Error) {
    super(details.message);
    this.name = 'XError';
    this.code = details.code;
    this.details = details;
    this.cause = cause;

    // Maintains proper stack trace for where our error was thrown (only available on V8)
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, XError);
    }
  }

  /**
   * Check if the error is retryable
   */
  isRetryable(): boolean {
    return [
      XErrorCode.RATE_LIMIT_EXCEEDED,
      XErrorCode.NETWORK_ERROR,
      XErrorCode.API_ERROR,
    ].includes(this.code);
  }

  /**
   * Get retry delay in seconds if applicable
   */
  getRetryDelay(): number | null {
    return this.details.retryAfter ?? null;
  }

  /**
   * Convert to JSON for serialization
   */
  toJSON(): Record<string, unknown> {
    return {
      name: this.name,
      code: this.code,
      message: this.message,
      details: this.details,
      cause: this.cause?.message,
    };
  }
}
