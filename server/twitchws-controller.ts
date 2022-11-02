import { twitchWs } from './twitch-websocket';

const connectToChannel = (channelName: string) => {
  console.log(`debug username: ${channelName}`);
  twitchWs.send(`JOIN #${channelName}`);
};

export {
  connectToChannel
};