import { ClipData, ShoutOutCommand, Streamer } from '../types/server-types';
import wsMap from './websocket-map';
import config from '../config/config';
import { getRawClip } from './processor';

const sendClip = async (command: ShoutOutCommand) => {
  wsMap.get(command.fromChannel).forEach(async (v, k) => {
    if (command.isStreamer || (command.isMod && v.allowMods !== 'false')) {
      try {
        const streamerData = (await getTargetStreamerId(command.targetChannel)).data[0];
        const streamer: Streamer = {
          id: streamerData.id,
          profilePic: streamerData.profile_image_url
        };

        /**
         * this code for filtering by dates is to be changed in the future
         * this is a testing solution for now
         */

        if (v.filterType === 'days' && (v.filterParams !== null || !Number.isNaN(v.filterParams))) {
          console.log('filter by days called');
          const currentDate: Date = new Date();
          const startDate: Date = new Date;
          startDate.setDate(startDate.getDate() - parseInt(v.filterParams));
          const filteredRandomClip: ClipData = await getRandomClipFiltered(streamer, currentDate, startDate);
          if (filteredRandomClip !== null) {
            k.send(JSON.stringify(filteredRandomClip));
            console.log(`sent: ${filteredRandomClip.clip_url} to client`);
          }
          return;
        }

        const randomClip: ClipData = await getRandomClip(streamer);
        if (randomClip !== null) {
          k.send(JSON.stringify(randomClip));
          console.log(`sent: ${randomClip.clip_url} to client`);
        }
      } catch (err) {
        console.log(err);
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

const getRandomClip = async (streamer: Streamer) => {
  const clips: TwitchClipsData[] = (await getRandomClipsFromApi(streamer.id)).data;
  if (clips.length !== 0) {
    console.log(`retrieved ${clips.length} clips from twitch api`);
    const randomClip: TwitchClipsData = clips[Math.floor(Math.random() * clips.length)];
    const clip: ClipData = {
      clip_url: getRawClip(randomClip.thumbnail_url),
      streamer: randomClip.broadcaster_name,
      profile_pic: streamer.profilePic,
      duration: randomClip.duration
    };
    return clip;
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

const getRandomClipFiltered = async (streamer: Streamer, endDate: Date, startDate: Date) => {
  const clips: TwitchClipsData[] = (await getClipsFilterDays(streamer.id, endDate, startDate)).data;
  if (clips.length !== 0) {
    console.log(`retrieved ${clips.length} clips from twitch api`);
    const randomClip: TwitchClipsData = clips[Math.floor(Math.random() * clips.length)];
    const clip: ClipData = {
      clip_url: getRawClip(randomClip.thumbnail_url),
      streamer: randomClip.broadcaster_name,
      profile_pic: streamer.profilePic,
      duration: randomClip.duration
    };
    return clip;
  } else {
    return await getRandomClip(streamer);
  }
};

const getClipsFilterDays = async (streamerId: string, endDate: Date, startDate: Date) => {
  try {
    const params: URLSearchParams = new URLSearchParams();
    params.append('broadcaster_id', streamerId);
    params.append('ended_at', endDate.toISOString());
    params.append('started_at', startDate.toISOString());
    params.append('first', '100');
    console.log(params.toString());
    const response = await fetch(`https://api.twitch.tv/helix/clips?${params.toString()}`, {
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