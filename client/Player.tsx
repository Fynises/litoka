import * as React from 'react';
import { useEffect } from 'react';
import { fetchWsUrl } from './api-app';
import WebSocket from 'isomorphic-ws';
import hash from 'object-hash';

interface WebSocketURLApiReturn {
  wsUrl: string
}

const Player = () => {

  const searchParams: URLSearchParams = new URLSearchParams(document.location.search);

  /**
   * planned url query params:
   * channel
   * allowMods
   * filterType
   * filterParams
   */

  useEffect(() => {
    const abortController = new AbortController();
    const signal = abortController.signal;
    fetchWsUrl(signal).then((data: WebSocketURLApiReturn) => {
      console.log(`fetched websocket url from server: ${data.wsUrl}`);
      const ws = new WebSocket(data.wsUrl);
      ws.onopen = () => {
        console.log('connecting to server websocket');
        const optionsObj = {
          channel: searchParams.get('channel'),
          allowMods: searchParams.get('allowMods'),
          filterType: searchParams.get('filterType'),
          filterParams: searchParams.get('filterParams')
        };
        ws.send(JSON.stringify({
          options: optionsObj,
          hash: hash(optionsObj)
        }));
      };
      //placeholder for reference later
      ws.onmessage = (data) => {
        console.log(`data recieved: ${data.data}`);
      };
    });
  });

  return (
    <></>
  );
};

export default Player;