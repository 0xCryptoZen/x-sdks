import { XClient, createTweetRequest, XError, XErrorCode } from '../src';

async function main() {
  // Load credentials from environment variables
  const credentials = {
    appKey: process.env.X_APP_KEY || '',
    appSecret: process.env.X_APP_SECRET || '',
    accessToken: process.env.X_ACCESS_TOKEN || '',
    accessSecret: process.env.X_ACCESS_SECRET || '',
  };

  // Create the X client
  const client = new XClient({ credentials });

  // Create a tweet
  const tweetText = process.argv[2] || 'Hello from x-sdk-typescript! 🚀';

  try {
    const tweet = createTweetRequest(tweetText);

    console.log('Posting tweet...');
    const response = await client.tweets().post(tweet);

    console.log('✅ Tweet posted successfully!');
    console.log('   ID:', response.data.id);
    console.log('   Text:', response.data.text);
  } catch (error) {
    if (error instanceof XError) {
      console.error('❌ Error posting tweet:', error.message);
      console.error('   Code:', error.code);

      if (error.details.statusCode) {
        console.error('   Status:', error.details.statusCode);
      }

      if (error.isRetryable()) {
        console.error('   This error is retryable');
        const retryDelay = error.getRetryDelay();
        if (retryDelay) {
          console.error(`   Retry after ${retryDelay} seconds`);
        }
      }
    } else {
      console.error('❌ Unexpected error:', error);
    }
    process.exit(1);
  }
}

main();
