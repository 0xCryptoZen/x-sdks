import { XError, XErrorCode } from '../../src/types/error';

describe('XError', () => {
  it('should create error with message', () => {
    const error = new XError({
      code: XErrorCode.INVALID_REQUEST,
      message: 'Test error',
    });
    expect(error.message).toBe('Test error');
    expect(error.code).toBe(XErrorCode.INVALID_REQUEST);
    expect(error).toBeInstanceOf(Error);
    expect(error.name).toBe('XError');
  });

  it('should create authentication error', () => {
    const error = new XError({
      code: XErrorCode.AUTHENTICATION_FAILED,
      message: 'Auth failed',
    });
    expect(error.code).toBe(XErrorCode.AUTHENTICATION_FAILED);
  });

  it('should create rate limit error with retry info', () => {
    const error = new XError({
      code: XErrorCode.RATE_LIMIT_EXCEEDED,
      message: 'Rate limited',
      retryAfter: 60,
    });
    expect(error.code).toBe(XErrorCode.RATE_LIMIT_EXCEEDED);
    expect(error.getRetryDelay()).toBe(60);
  });

  it('should return null retry delay when not set', () => {
    const error = new XError({
      code: XErrorCode.NETWORK_ERROR,
      message: 'Test',
    });
    expect(error.getRetryDelay()).toBeNull();
  });

  it('should check if error is retryable', () => {
    const retryableError = new XError({
      code: XErrorCode.RATE_LIMIT_EXCEEDED,
      message: 'Rate limited',
    });
    expect(retryableError.isRetryable()).toBe(true);

    const nonRetryableError = new XError({
      code: XErrorCode.AUTHENTICATION_FAILED,
      message: 'Auth failed',
    });
    expect(nonRetryableError.isRetryable()).toBe(false);
  });

  it('should convert to JSON', () => {
    const error = new XError({
      code: XErrorCode.API_ERROR,
      message: 'API error',
      statusCode: 500,
    });
    const json = error.toJSON();
    expect(json.name).toBe('XError');
    expect(json.code).toBe(XErrorCode.API_ERROR);
    expect(json.message).toBe('API error');
  });
});
