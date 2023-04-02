import { invoke } from "@tauri-apps/api";

enum RequestInvoke {
  listPlaylist = "list_playlist",
  testCookie = "test_cookie",
  testCookieLoad = "test_cookie_load",
}

export const listPlaylist = async (payload: PlayListData) => {
  return invoke<Playlist[]>(RequestInvoke.listPlaylist, {
    payload
  });
};

export const testCookie = async () => {
  invoke(RequestInvoke.testCookie);
};

export const testCookieLoad = async () => {
  invoke(RequestInvoke.testCookieLoad);
};