import { invoke } from "@tauri-apps/api";

enum RequestInvoke {
  getResourceRecomend = "get_resources_recommend",
}

export const getSongListRecommend = async() => {
  return invoke<SongList>(RequestInvoke.getResourceRecomend);
};