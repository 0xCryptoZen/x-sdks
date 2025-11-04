# X SDK for TypeScript

A TypeScript SDK for the Twitter/X API v2 with OAuth 1.0a authentication support.

## Features

- ✅ OAuth 1.0a authentication with automatic request signing
- ✅ Post tweets (POST /2/tweets)
- ✅ Full TypeScript type safety
- ✅ Comprehensive error handling with retry information
- ✅ Promise-based async/await API
- ✅ Built on axios for reliable HTTP handling

## Installation

```bash
npm install @x-sdks/typescript
# or
yarn add @x-sdks/typescript
```

## Quick Start

```typescript
import { XClient, createTweetRequest } from '@x-sdks/typescript';

// Create client with credentials
const client = new XClient({
  credentials: {
    appKey: 'your_app_key',
    appSecret: 'your_app_secret',
    accessToken: 'your_access_token',
    accessSecret: 'your_access_secret',
  },
});

// Post a tweet
const tweet = createTweetRequest('Hello from TypeScript! 🚀');
const response = await client.tweets().post(tweet);
console.log('Tweet posted! ID:', response.data.id);
```

## Examples

### Basic Tweet Posting

```typescript
import { XClient, createTweetRequest } from '@x-sdks/typescript';

async function postTweet() {
  const client = new XClient({
    credentials: {
      appKey: process.env.X_APP_KEY!,
      appSecret: process.env.X_APP_SECRET!,
      accessToken: process.env.X_ACCESS_TOKEN!,
      accessSecret: process.env.X_ACCESS_SECRET!,
    },
  });

  const tweet = createTweetRequest('Hello, X!');
  const response = await client.tweets().post(tweet);

  console.log('Tweet ID:', response.data.id);
  console.log('Tweet text:', response.data.text);
}
```

### Error Handling

```typescript
import { XClient, createTweetRequest, XError, XErrorCode } from '@x-sdks/typescript';

async function postWithErrorHandling() {
  const client = new XClient({
    credentials: {
      appKey: process.env.X_APP_KEY!,
      appSecret: process.env.X_APP_SECRET!,
      accessToken: process.env.X_ACCESS_TOKEN!,
      accessSecret: process.env.X_ACCESS_SECRET!,
    },
  });

  try {
    const tweet = createTweetRequest('Hello!');
    const response = await client.tweets().post(tweet);
    console.log('Success:', response.data.id);
  } catch (error) {
    if (error instanceof XError) {
      switch (error.code) {
        case XErrorCode.RATE_LIMIT_EXCEEDED:
          console.error('Rate limited:', error.message);
          const retryAfter = error.getRetryDelay();
          if (retryAfter) {
            console.error(`Retry after ${retryAfter} seconds`);
          }
          break;

        case XErrorCode.AUTHENTICATION_FAILED:
          console.error('Authentication failed:', error.message);
          break;

        case XErrorCode.INVALID_REQUEST:
          console.error('Invalid request:', error.message);
          break;

        default:
          console.error('Error:', error.message);
      }
    } else {
      console.error('Unexpected error:', error);
    }
  }
}
```

### Custom Configuration

```typescript
import { XClient } from '@x-sdks/typescript';

const client = new XClient({
  credentials: {
    appKey: 'your_app_key',
    appSecret: 'your_app_secret',
    accessToken: 'your_access_token',
    accessSecret: 'your_access_secret',
  },
  timeout: 60000, // 60 seconds
  baseUrl: 'https://api.twitter.com', // Custom base URL (if needed)
});
```

## API Documentation

### `XClient`

Main client for interacting with the X API.

```typescript
const client = new XClient({
  credentials: {
    appKey: string;
    appSecret: string;
    accessToken: string;
    accessSecret: string;
  },
  timeout?: number;  // Default: 30000ms
  baseUrl?: string;  // Default: 'https://api.twitter.com'
});
```

### `createTweetRequest(text: string): TweetRequest`

Create and validate a tweet request.

```typescript
const tweet = createTweetRequest('Hello, world!');
```

Validates that:
- Text is not empty
- Text does not exceed 280 characters

### `TweetResponse`

Response from posting a tweet.

```typescript
interface TweetResponse {
  data: {
    id: string;
    text: string;
    edit_history_tweet_ids?: string[];
  };
}
```

### Error Handling

All operations throw `XError` on failure:

```typescript
class XError extends Error {
  code: XErrorCode;
  details: XErrorDetails;
  cause?: Error;

  isRetryable(): boolean;
  getRetryDelay(): number | null;
}
```

**Error Codes:**
- `AUTHENTICATION_FAILED` - Invalid credentials (401, 403)
- `RATE_LIMIT_EXCEEDED` - Rate limit hit (429)
- `INVALID_REQUEST` - Bad request (400, 422)
- `NETWORK_ERROR` - Network failure
- `API_ERROR` - Other API errors
- `UNKNOWN_ERROR` - Unexpected errors

Check if an error is retryable:

```typescript
if (error instanceof XError && error.isRetryable()) {
  const delay = error.getRetryDelay();
  // Wait and retry
}
```

## Running Examples

Set your credentials as environment variables:

```bash
export X_APP_KEY="your_app_key"
export X_APP_SECRET="your_app_secret"
export X_ACCESS_TOKEN="your_access_token"
export X_ACCESS_SECRET="your_access_secret"
```

Run the example:

```bash
npm install
npx ts-node examples/post-tweet.ts "Your tweet text here"
```

## Testing

Run the test suite:

```bash
npm test
```

Run tests with coverage:

```bash
npm run test:coverage
```

## Building

Build the project:

```bash
npm run build
```

This will compile TypeScript to JavaScript in the `dist/` directory.

## License

MIT

## References

- [Twitter API v2 Documentation](https://developer.twitter.com/en/docs/twitter-api)
- [OAuth 1.0a Specification](https://oauth.net/core/1.0a/)
