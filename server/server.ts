import app from './express';
import config from '../config/config';
import initWebSocketServer from './websocket';
import { twitchWs } from './twitch-websocket';

initWebSocketServer();

twitchWs.on('open', () => {
  console.log('connected to twitch api');
  twitchWs.send('CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands');
  twitchWs.send(`PASS oauth:${config.twitchAccessToken}`);
  twitchWs.send('NICK opensobot');
});

app.listen(config.port, () => {
  console.info('Server started on point %s.', config.port);
});