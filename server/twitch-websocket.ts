import WebSocket from 'ws';
import config from '../config/config';
import { sendClip } from './api-controller';
import { parseIRC } from './chat-parser';
import { ShoutOutCommand } from '../types/server-types';

const twitchWs: WebSocket = new WebSocket(config.twitchChatUrl);

twitchWs.on('message', (data) => {
  console.log(`recieved from twitch: ${data}`);
  if (data.toString().includes('PRIVMSG')) {
    const parsedIrC: ShoutOutCommand = parseIRC(data.toString());
    if (parsedIrC !== null) {
      console.log(parsedIrC);
      sendClip(parsedIrC);
    }
  }
});

export {
  twitchWs
};