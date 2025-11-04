import { TweetRequest, TweetResponse } from '../types';
import type { XClient } from '../client';

/**
 * Tweets endpoint API
 */
export class Tweets {
  private client: XClient;

  /**
   * @internal
   */
  constructor(client: XClient) {
    this.client = client;
  }

  /**
   * Post a new tweet
   * @param request - The tweet request with text content
   * @returns Promise resolving to the created tweet
   * @throws XError if the request fails
   *
   * @example
   * ```typescript
   * const client = new XClient({ credentials });
   * const tweet = await client.tweets().post({ text: 'Hello, world!' });
   * console.log('Tweet posted:', tweet.data.id);
   * ```
   */
  async post(request: TweetRequest): Promise<TweetResponse> {
    const response = await this.client.getHttpClient().post<TweetResponse>('/2/tweets', request);
    return response.data;
  }
}
