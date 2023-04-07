interface IBaseConfig {
    //TODO system based theme mode
    theme_mode?: "light" | "dark",
}

// ANCHOR list playlist
interface PlayListData {
  order?: "hot" | "new",
  cat?: "全部",
  offset?: number,
  limit?: number
}

interface Playlist {
  id: number,
  name: string,
  coverImgUrl: string,
  author: string,
}

// ANCHOR get qrcode
interface Qrcode {
  rawData: string,
  unikey: string,
}

// ANCHOR get user account
interface UserAccount {
  userId: number,
  nickname: string,
  avatarImgUrl: string,
}

// ANCHOR get playlist detail
interface PlaylistDetailData {
  id: number,
  size?: number,
}
interface PlaylistDetail {
  id: number,
  name: string,
  coverImgUrl: string,
  description: string,
  createTime: number,
  trackUpdateTime: number,
  songs: Song[],
}

interface Song {
  id: number,
  name: string,
  singer: string[],
  album: string,
  albumId: number,
  coverImgUrl: string,
  duration: number,
}