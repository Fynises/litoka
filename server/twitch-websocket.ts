import WebSocket from 'ws';
import config from '../config/config';
//import { sendClip } from './api-controller';
import { parseIRC } from './chat-parser';
import { TwitchMessage } from '../types/server-types';

const twitchWs: WebSocket = new WebSocket(config.twitchChatUrl);

twitchWs.on('message', (data) => {
  console.log(`recieved from twitch: ${data}`);
  if (data.toString().includes('PRIVMSG')) {
    const parsedIrc: TwitchMessage = parseIRC(data.toString());
    console.log(parsedIrc);
  }
});

export {
  twitchWs
};