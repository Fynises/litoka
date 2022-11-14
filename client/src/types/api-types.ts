export interface WebSocketURLApiReturn {
  ws_url: string
}

export interface ClipData {
  clip_url: string,
  streamer: string,
  profile_pic: string,
  clip_duration: number
}

export interface ShoutOutURLParams {
  channel: string,
  allow_mods: boolean,
  filter_type: string,
  filter_params: string
}

export interface ShoutOutOptions {
  options: ShoutOutURLParams,
  hash: string
}