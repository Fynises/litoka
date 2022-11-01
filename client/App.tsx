import * as React from 'react';
import { useState, useEffect } from 'react';
import { fetchWsUrl } from './api-app';
import WebSocket from 'isomorphic-ws';
import hash from 'object-hash';

interface WebSocketURLApiReturn {
  wsUrl: string
}

const App = () => {

  const [message, setMessage] = useState('');
  const [hasError, setHasError] = useState(false);
  const [webSocketUrl, setWebSocketUrl] = useState('');

  const getWsUrl = () => {
    const abortController = new AbortController;
    const signal = abortController.signal;
    fetchWsUrl(signal).then((data: WebSocketURLApiReturn) => {
      console.log(`recieved websocket url from server: ${data.wsUrl}`);
      setWebSocketUrl(data.wsUrl);
    });
    return function cleanup() {
      abortController.abort();
    };
  };

  const searchParams: URLSearchParams = new URLSearchParams(document.location.search);

  /**
   * planned url query params:
   * channel
   * allowMods
   * filterType
   * filterParams
   */

  useEffect(() => {
    if (!searchParams.has('channel')) {
      setMessage('there is an error with the url parameters, refer to: https://github.com/Fynises/twitch-open-so for further documentation');
      setHasError(true);
    }

    if (hasError) {
      getWsUrl();

      const ws = new WebSocket(webSocketUrl);
      ws.onopen = () => {
        console.log('connecting to server websocket...');
        const optionsObj = {
          channel: searchParams.get('channel'),
          allowMods: searchParams.get('allowMods'),
          filterType: searchParams.get('filterType'),
          filterParams: searchParams.get('filterParams')
        };
        ws.send(JSON.stringify({
          hash: hash(optionsObj),
          options: optionsObj
        }));
      };
      //placeholder for now
      ws.onmessage = (data) => {
        console.log(`data recieved: ${data}`);
      };
    }

  });

  return (
    <>
      <p>{message}</p>
    </>
  );
};

export default App;