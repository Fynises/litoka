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
      console.log(dataJson.options.allowMods);
      if (wsMap.get(dataJson.options.channel) === undefined) {
        wsMap.set(dataJson.options.channel, new Map().set(ws, dataJson.options));
        connectToChannel(dataJson.options.channel);
      } else {
        wsMap.get(dataJson.options.channel).set(ws, dataJson.options);
        connectToChannel(dataJson.options.channel);
      }
    });

    ws.on('close', () => {
      console.log('client connection closed');
      wsMap.forEach((v, k) => {
        v.delete(ws);
      });
    });

    // eslint-disable-next-line @typescript-eslint/no-empty-function
    ws.on('pong', () => {});

  });

  setInterval(() => {
    console.log('ping');
    wss.clients.forEach((client) => {
      client.ping();
    });
  }, 20000);

};
////textShadow: '-4px -4px 0 #000, 0 -4px 0 #000, 4px -4px 0 #000, 4px 0 0 #000, 4px 4px 0 #000, 0 4px 0 #000, -4px 4px 0 #000, -4px 0 0 #000'
export default initWebSocketServer;