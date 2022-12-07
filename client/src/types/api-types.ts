export interface WebSocketURLApiReturn {
  ws_url: string
}

export interface ClipData {
  clip_url: string,
  streamer: string,
  profile_pic: string,
  clipper?: string,
  game?: string,
  clip_duration: number
}

export interface ShoutOutURLParams {
  channel: string,
  allow_mods?: boolean,
  allow_vip?: boolean,
  allow_subs?: boolean,
  filter_type?: string,
  filter_params?: string,
  disable_overrides?: boolean
}

export interface ShoutOutOptions {
  options: ShoutOutURLParams,
  hash: string
}