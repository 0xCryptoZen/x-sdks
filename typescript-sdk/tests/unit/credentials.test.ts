import { Credentials } from '../../src/auth';

describe('Credentials', () => {
  it('should have required fields', () => {
    const validCredentials: Credentials = {
      appKey: 'test_key',
      appSecret: 'test_secret',
      accessToken: 'test_token',
      accessSecret: 'test_token_secret',
    };

    expect(validCredentials.appKey).toBe('test_key');
    expect(validCredentials.appSecret).toBe('test_secret');
    expect(validCredentials.accessToken).toBe('test_token');
    expect(validCredentials.accessSecret).toBe('test_token_secret');
  });

  it('should be a valid type for all required fields', () => {
    const credentials: Credentials = {
      appKey: 'app_key_123',
      appSecret: 'app_secret_456',
      accessToken: 'access_token_789',
      accessSecret: 'access_secret_012',
    };

    expect(typeof credentials.appKey).toBe('string');
    expect(typeof credentials.appSecret).toBe('string');
    expect(typeof credentials.accessToken).toBe('string');
    expect(typeof credentials.accessSecret).toBe('string');
  });
});
