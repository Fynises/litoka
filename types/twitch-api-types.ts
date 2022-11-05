interface TwitchUserData {
  id: string,
  login: string,
  type: string,
  broadcaster_type: string,
  description: string,
  profile_image_url: string,
  view_count: number,
  created_at: string
}

interface TwitchGetUser {
  data: TwitchUserData[]
}

interface TwitchClipsData {
  id: string,
  url: string,
  embed_url: string,
  broadcaster_id: string,
  broadcaster_name: string,
  creator_id: string,
  creator_name: string,
  video_id: string,
  game_id: string,
  language: string,
  title: string,
  view_count: number,
  created_at: string,
  thumbnail_url: string,
  duration: number,
  vod_offset: number
}

interface PaginationObj {
  cursor: string
}

interface TwitchGetClips {
  data: TwitchClipsData[],
  pagination: PaginationObj
}