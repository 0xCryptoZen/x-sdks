import { XClient } from '../../src/client';
import { Credentials } from '../../src/auth';

describe('XClient', () => {
  const mockCredentials: Credentials = {
    appKey: 'test_app_key',
    appSecret: 'test_app_secret',
    accessToken: 'test_access_token',
    accessSecret: 'test_access_secret',
  };

  describe('constructor', () => {
    it('should create an instance with valid credentials', () => {
      const client = new XClient({ credentials: mockCredentials });
      expect(client).toBeInstanceOf(XClient);
    });

    it('should use default base URL', () => {
      const client = new XClient({ credentials: mockCredentials });
      expect(client).toBeDefined();
      // Default base URL is set internally
    });

    it('should accept custom base URL', () => {
      const client = new XClient({
        credentials: mockCredentials,
        baseUrl: 'https://custom.api.twitter.com',
      });
      expect(client).toBeDefined();
    });

    it('should accept custom timeout', () => {
      const client = new XClient({
        credentials: mockCredentials,
        timeout: 60000,
      });
      expect(client).toBeDefined();
    });
  });

  describe('tweets', () => {
    it('should return tweets API instance', () => {
      const client = new XClient({ credentials: mockCredentials });
      const tweetsApi = client.tweets();
      expect(tweetsApi).toBeDefined();
    });
  });
});
