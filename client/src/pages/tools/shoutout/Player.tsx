import * as React from 'react';
import { useEffect, useState } from 'react';
import { fetchWsUrl } from './api-app';
import WebSocket from 'isomorphic-ws';
import hash from 'object-hash';
import ReactPlayer from 'react-player';
import { ClipData, ShoutOutOptions, ShoutOutURLParams, WebSocketURLApiReturn } from '../../../types/api-types';

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
      console.log(`fetched websocket url from server: ${data.ws_url}`);
      try {
        const ws = new WebSocket(data.ws_url);
        ws.onopen = () => {
          console.log('connecting to server websocket');

          const optionsObj: ShoutOutURLParams = {
            channel: searchParams.get('channel').toLowerCase(),
            allow_mods: searchParams.get('allowMods') === 'true',
            filter_type: searchParams.get('filterType'),
            filter_params: searchParams.get('filterParams')
          };

          const shoutoutOptions: ShoutOutOptions = {
            options: optionsObj,
            hash: hash(optionsObj)
          };

          ws.send(JSON.stringify(shoutoutOptions));
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
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-around',
              alignItems: 'center',
              fontFamily: 'sans-serif',
              color: 'white',
              fontSize: '52px',
              paddingBottom: 3,
              textShadow: '-4px -4px 0 #000, 0 -4px 0 #000, 4px -4px 0 #000, 4px 0 0 #000, 4px 4px 0 #000, 0 4px 0 #000, -4px 4px 0 #000, -4px 0 0 #000'
            }}
          >
            {`Check out ${clips[0].streamer}!`}
            <img
              src={clips[0].profile_pic}
              width={96}
              height={96}
              alt='profile_pic'
            />
          </div>
          <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
            <ReactPlayer
              width='800px'
              height='450px'
              url={clips[0].clip_url}
              playsinline={true}
              playing={true}
              muted={false}
              onEnded={() => shiftNext(clips[0].clip_url)}
            />
          </div>
        </span>)
      }
    </>
  );
};

export default Player;