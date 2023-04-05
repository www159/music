import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

enum RequestInvoke {
  abortQrocdeSession = "abort_qrcode_session",
  createQrcodeSession = "create_qrcode_session",
  getQrcode = "get_qrcode",
  listPlaylist = "list_playlist",
  testCookie = "test_cookie",
  testCookieLoad = "test_cookie_load",
}

export const abortQrocdeSession = async () => {
  await invoke<void>(RequestInvoke.abortQrocdeSession);
};

export const createQrcodeSession = async (valueSetter: (val: number) => void) => {
  const EVENT = "music-all://step";

  invoke<void>(RequestInvoke.createQrcodeSession);
  const unlisten = listen<number>(EVENT, (val) => {
    valueSetter(val.payload);
  });

  return unlisten;
};

export const getQrcode = async () => {
  return await invoke<Qrcode>(RequestInvoke.getQrcode);
};

export const listPlaylist = async (payload: PlayListData) => {
  return await invoke<Playlist[]>(RequestInvoke.listPlaylist, {
    payload
  });
};

// SECTION test invoke
export const testCookie = () => {
  invoke(RequestInvoke.testCookie);
};

export const testCookieLoad = () => {
  invoke(RequestInvoke.testCookieLoad);
};
// ~SECTION