/**
 * Request to post a tweet
 */
export interface TweetRequest {
  /**
   * The text content of the tweet (max 280 characters)
   */
  text: string;
}

/**
 * Validate and create a tweet request
 * @param text - The tweet content
 * @returns Validated TweetRequest
 * @throws Error if text is invalid
 */
export function createTweetRequest(text: string): TweetRequest {
  if (!text || text.trim().length === 0) {
    throw new Error('Tweet text cannot be empty');
  }

  // Count characters (not bytes)
  const charCount = [...text].length;
  if (charCount > 280) {
    throw new Error(`Tweet text exceeds 280 characters (got ${charCount})`);
  }

  return { text };
}
