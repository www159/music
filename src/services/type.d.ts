interface IBaseConfig {
    //TODO system based theme mode
    theme_mode?: "light" | "dark",
}

interface PlayListData {
  order?: "hot" | "new",
  cat?: "全部",
  offset?: number,
  limit?: number
}

interface Playlist {
  id: number,
  name: string,
  cover_img_url: string,
  author: string,
}