import WebSocket from 'ws';
import config from '../config/config';
import { parseIRC } from './chat-parser';

const twitchWs: WebSocket = new WebSocket(config.twitchChatUrl);

twitchWs.on('message', (data) => {
  console.log(`recieved from twitch: ${data}`);
  if (data.toString().includes('PRIVMSG')) {
    console.log(parseIRC(data.toString()));
  }
});

export {
  twitchWs
};