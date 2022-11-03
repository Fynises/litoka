import { ShoutOutCommand } from './types/server-types';
import wsMap from './websocket-map';
import config from '../config/config';

const sendClip = async (command: ShoutOutCommand) => {
  wsMap.get(command.fromChannel).forEach(async (element) => {
    if (command.isStreamer || (command.isMod && element.clientInfo.options.allowMods)) {
      const streamerId: string = (await getTargetStreamerId(command.targetChannel)).data[0].id;
      const randomClip: string = await getRandomClip(streamerId);
      if (randomClip !== null) {
        element.wsObj.send(randomClip);
      }
    }
  });
};

const getTargetStreamerId = async (name: string) => {
  try {
    const response = await fetch(`https://api.twitch.tv/helix/users?login=${name}`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${config.twitchOAuth}`,
        'Client-Id': config.twitchClientId
      }
    });
    return await response.json() as TwitchGetUser;
  } catch (err) {
    console.log(err);
  }
};

const getRandomClip = async (streamerId: string) => {
  const clips: TwitchClipsData[] = (await getRandomClipsFromApi(streamerId)).data;
  if (clips.length !== 0) {
    console.log(clips);
    return clips[Math.floor(Math.random() * clips.length)].embed_url;
  } else {
    return null;
  }
};

const getRandomClipsFromApi = async (streamerId: string) => {
  try {
    const response = await fetch(`https://api.twitch.tv/helix/clips?broadcaster_id=${streamerId}&first=100`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${config.twitchOAuth}`,
        'Client-Id': config.twitchClientId
      }
    });
    return await response.json() as TwitchGetClips;
  } catch (err) {
    console.log(err);
  }
};

export {
  sendClip
};