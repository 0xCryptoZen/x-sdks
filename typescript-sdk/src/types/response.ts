/**
 * Tweet data from the API
 */
export interface TweetData {
  /**
   * Unique identifier of the tweet
   */
  id: string;

  /**
   * The actual UTF-8 text of the tweet
   */
  text: string;

  /**
   * Edit history tweet IDs (if tweet was edited)
   */
  edit_history_tweet_ids?: string[];
}

/**
 * Response from posting a tweet
 */
export interface TweetResponse {
  /**
   * The tweet data
   */
  data: TweetData;
}

/**
 * Get the tweet URL
 * @param response - The tweet response
 * @param username - The username of the account that posted the tweet
 * @returns The full URL to the tweet
 */
export function getTweetUrl(response: TweetResponse, username: string): string {
  return `https://twitter.com/${username}/status/${response.data.id}`;
}
