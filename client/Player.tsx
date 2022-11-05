import * as React from 'react';
import { useEffect, useState } from 'react';
import { fetchWsUrl } from './api-app';
import WebSocket from 'isomorphic-ws';
import hash from 'object-hash';
import ReactPlayer from 'react-player';

interface WebSocketURLApiReturn {
  wsUrl: string
}

interface ClipData {
  clip_url: string,
  streamer: string,
  clipDuration: number,
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
      try {
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
          console.log(`url recieved: ${dataJson.clip_url}`);
          setClips(previous => [...previous, dataJson]);
        };
      } catch (err) {
        console.log(err);
      }
    });
  }, []);

  const shiftNext = (url: string) => {
    setClips(clips.filter(item => item.clip_url !== url));
  };

  return (
    <>
      {
        clips.length != 0 && (<span>
          <ReactPlayer
            width='100%'
            url={clips[0].clip_url}
            playsinline={true}
            playing={true}
            muted={false}
            onEnded={() => shiftNext(clips[0].clip_url)}
          />
        </span>)
      }
    </>
  );
};

export default Player;