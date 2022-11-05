import { WebSocketServer } from 'ws';
import config from '../config/config';
import { ClientConnectMessage } from '../types/server-types';
import wsMap from './websocket-map';
import { connectToChannel } from './twitchws-controller';

const initWebSocketServer = () => {
  const wss = new WebSocketServer({ port: parseInt(config.wsPort) });

  wss.on('connection', (ws) => {
    ws.on('message', (data) => {
      const dataJson: ClientConnectMessage = JSON.parse(data.toString());
      console.log(`recieved new connection: ${JSON.stringify(dataJson)}`);

      if (wsMap.get(dataJson.options.channel) === undefined) {
        wsMap.set(dataJson.options.channel, new Map().set(ws, dataJson.options));
        connectToChannel(dataJson.options.channel);
      } else {
        wsMap.get(dataJson.options.channel).set(ws, dataJson.options);
        connectToChannel(dataJson.options.channel);
      }
    });

    ws.on('close', () => {
      console.log('connection closed');
      wsMap.forEach((v,k) => {
        v.delete(ws);
      });
    });

  });
};

export default initWebSocketServer;