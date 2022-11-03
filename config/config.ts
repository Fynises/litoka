const config = {
  port: process.env.PORT,
  wsPort: process.env.WS_PORT,
  wsUrl: process.env.WS_URL,
  twitchChatUrl: process.env.TWITCH_CHAT_URL,
  twitchClientId: process.env.TWITCH_CLIENT_ID,
  twitchAccessToken: process.env.TWITCH_ACCESS_TOKEN,
  twitchOAuth: process.env.TWITCH_OAUTH
};

export default config;