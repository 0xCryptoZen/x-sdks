/**
 * X SDK for TypeScript
 *
 * A TypeScript SDK for the Twitter/X API v2 with OAuth 1.0a authentication support.
 *
 * @example
 * ```typescript
 * import { XClient, createTweetRequest } from '@x-sdks/typescript';
 *
 * const client = new XClient({
 *   credentials: {
 *     appKey: 'your_app_key',
 *     appSecret: 'your_app_secret',
 *     accessToken: 'your_access_token',
 *     accessSecret: 'your_access_secret',
 *   },
 * });
 *
 * const tweet = createTweetRequest('Hello from TypeScript!');
 * const response = await client.tweets().post(tweet);
 * console.log('Tweet posted:', response.data.id);
 * ```
 */

export { XClient, type XClientConfig } from './client';
export { Credentials, OAuthClient } from './auth';
export {
  XError,
  XErrorCode,
  type XErrorDetails,
  TweetRequest,
  TweetResponse,
  TweetData,
  createTweetRequest,
  getTweetUrl,
} from './types';
export { Tweets } from './endpoints';
