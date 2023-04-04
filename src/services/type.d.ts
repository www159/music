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

// ANCHOR get qrcode status