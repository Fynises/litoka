import * as React from 'react';
import { useEffect, useState } from 'react';
import { fetchWsUrl } from './api-app';
import WebSocket from 'isomorphic-ws';
import hash from 'object-hash';

interface WebSocketURLApiReturn {
  wsUrl: string
}

interface ClipData {
  clip_url: string,
  streamer: string
}

const Player = () => {

  const searchParams: URLSearchParams = new URLSearchParams(document.location.search);

  const [clips, setClips] = useState<ClipData[]>([]);

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
          channel: searchParams.get('channel').toLowerCase(),
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
        const dataJson: ClipData = JSON.parse(data.data.toString());
        console.log(`data recieved: ${dataJson}`);
        setClips(previous => [...previous, dataJson]);
      };
    });
  }, []);

  //encountering Cross-Origin Read Blocking errors

  return (
    <>
      {
        clips.length != 0 && (<span>
          <video width='100%' autoPlay>
            <source
              src={clips.pop().clip_url}
              type='video/mp4'
            />
          </video>
        </span>)
      }
    </>
  );
};

export default Player;