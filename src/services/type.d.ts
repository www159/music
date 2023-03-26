interface IBaseConfig {
    //TODO system based theme mode
    theme_mode?: "light" | "dark",
}

interface SongList {
  id: number;
  name: string;
  cover_img_url: string;
  creator_name: string;
}