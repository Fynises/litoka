import WebSocket from 'ws';
import config from '../config/config';

const twitchWs: WebSocket = new WebSocket(config.twitchChatUrl);

twitchWs.on('message', (data) => {
  console.log(`recieved from twitch: ${data}`);
});

export {
  twitchWs
};