import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

enum RequestInvoke {
  createQrcodeSession = "create_qrcode_session",
  getQrcode = "get_qrcode",
  listPlaylist = "list_playlist",
  testCookie = "test_cookie",
  testCookieLoad = "test_cookie_load",
}

export const createQrcodeSession = async (valueSetter: (val: number) => void) => {
  const EVENT = "music-all://step";

  invoke<void>(RequestInvoke.createQrcodeSession);
  const unlisten = listen<number>(EVENT, (val) => {
    valueSetter(val.payload);
  });

  return unlisten;
};

export const getQrcode = async () => {
  return invoke<Qrcode>(RequestInvoke.getQrcode);
};

export const listPlaylist = async (payload: PlayListData) => {
  return invoke<Playlist[]>(RequestInvoke.listPlaylist, {
    payload
  });
};

// SECTION test invoke
export const testCookie = async () => {
  invoke(RequestInvoke.testCookie);
};

export const testCookieLoad = async () => {
  invoke(RequestInvoke.testCookieLoad);
};
// ~SECTION