/**
 * production setting to true will authorise the app programmatically, don't use for testing
 */

const config = {
  port: process.env.PORT,
  wsPort: process.env.WS_PORT,
  wsUrl: process.env.WS_URL,
  production: process.env.PRODUCTION || false,
  twitchChatUrl: process.env.TWITCH_CHAT_URL,
  twitchClientId: process.env.TWITCH_CLIENT_ID,
  twitchClientSecret: process.env.TWITCH_CLIENT_SECRET,
  twitchAccessToken: process.env.TWITCH_ACCESS_TOKEN || '',
  twitchOAuth: process.env.TWITCH_OAUTH || '',
  twitchRefresh: ''
};

export default config;