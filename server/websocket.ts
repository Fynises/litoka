import { WebSocketServer } from 'ws';
import config from '../config/config';
import { ClientConnectMessage, ServerWSObject } from './server-types';
import wsMap from './websocket-map';

const initWebSocketServer = () => {
  const wss = new WebSocketServer({ port: parseInt(config.wsPort) });

  wss.on('connection', (ws) => {
    ws.on('message', (data) => {
      const dataJson: ClientConnectMessage = JSON.parse(data.toString());
      console.log(`recieved new connection: ${JSON.stringify(dataJson)}`);
      const serverWsData: ServerWSObject = {
        wsObj: ws,
        clientInfo: dataJson
      };
      if (wsMap.get(dataJson.options.channel) === undefined) {
        wsMap.set(dataJson.options.channel, [serverWsData]);
      } else {
        wsMap.get(dataJson.options.channel).push(serverWsData);
      }
    });
  });
};

export default initWebSocketServer;