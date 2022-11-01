import {WebSocketServer} from 'ws';
import config from '../config/config';

const initWebSocketServer = () => {
  const wss = new WebSocketServer({ port: parseInt(config.wsPort) });

  wss.on('connection', (ws) => {
    ws.on('message', (data) => {
      console.log(JSON.parse(data.toString()));
    });
  });
};

export default initWebSocketServer;