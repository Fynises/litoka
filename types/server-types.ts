import WebSocket from 'ws';

export interface ClientConnectOptions {
  channel: string,
  allowMods: string,
  filterType: string,
  filterParams: string
}

export interface ClientConnectMessage {
  options: ClientConnectOptions
  hash: string
}

export interface ServerWSObject {
  wsObj: WebSocket,
  clientInfo: ClientConnectMessage
}

export interface ShoutOutCommand {
  fromChannel: string,
  requesterName: string,
  requesterId: string,
  isStreamer: boolean,
  isMod: boolean,
  overrideRandom: boolean,
  targetChannel: string
}

export interface TwitchMessage {
  chatterName: string,
  chatterId: string,
  channel: string,
  isBroadcaster: boolean,
  isMod: boolean,
  isSubscriber: boolean,
  message: string
}

export interface ClipData {
  clip_url: string,
  streamer: string,
  profile_pic: string,
  duration: number
}

export interface Streamer {
  id: string,
  profilePic:string
}