import { invoke } from "@tauri-apps/api";

enum RequestInvoke {
  listPlaylist = "list_playlist",
}

export const listPlaylist = async (payload: PlayListData) => {
  return invoke<Playlist[]>(RequestInvoke.listPlaylist, {
    payload
  });
};