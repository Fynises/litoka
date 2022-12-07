import * as React from 'react';
import { useEffect, useState } from 'react';
import { fetchWsUrl } from './api-app';
import WebSocket from 'isomorphic-ws';
import hash from 'object-hash';
import ReactPlayer from 'react-player';
import { ClipData, ShoutOutOptions, ShoutOutURLParams } from '../../../types/api-types';

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


  // TODO: move these functions to a helper file
  const tryGetString = (query: string): string => {
    const param = searchParams.get(query);
    if (param !== null) {
      return param as string;
    } else {
      return 'err';
    }
  };

  const tryGetStringNullable = (query: string): string | undefined => {
    const param = searchParams.get(query);
    if (param !== null) {
      return param as string;
    } else {
      return undefined;
    }
  };

  const tryGetBool = (query: string): boolean => {
    const param = searchParams.get(query);
    if (param !== null) {
      return param === 'true';
    } else {
      return false;
    }
  };

  useEffect(() => {
    const abortController = new AbortController();
    const signal = abortController.signal;
    fetchWsUrl(signal).then((data) => {
      if (data !== undefined) {
        console.log(`fetched websocket url from server: ${data.ws_url}`);
        try {
          const ws = new WebSocket(data.ws_url);
          ws.onopen = () => {
            console.log('connecting to server websocket');

            const optionsObj: ShoutOutURLParams = {
              channel: tryGetString('channel'),
              allow_mods: tryGetBool('allowMods'),
              allow_vip: tryGetBool('allowVip'),
              allow_subs: tryGetBool('allowSubs'),
              filter_type: tryGetStringNullable('filterType'),
              filter_params: tryGetStringNullable('filterParams'),
              disable_overrides: tryGetBool('disableOverrides')
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
      } else {
        console.log('error fetching websocket url from server');
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
          <div
            style={{
              fontFamily: 'sans-serif',
              color: 'white',
              fontSize: '20px',
              marginTop: -10
            }}
          >
            <div style={{ paddingLeft: 12, paddingBottom: 1 }}>{`playing: ${clips[0].game}`}</div>
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
          <div
            style={{
              fontFamily: 'sans-serif',
              color: 'white',
              fontSize: '20px',
              display: 'flex',
              marginTop: -24,
              justifyContent: 'right'
            }}
          >
            <div style={{ paddingRight: 12 }}>{`clipper: ${clips[0].clipper}`}</div>
          </div>
        </span>)
      }
    </>
  );
};

export default Player;