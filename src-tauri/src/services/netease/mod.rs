use crate::applications;
use crate::applications::netease::App;

use applications::AppState;

use applications::netease::ListRequest;
use applications::netease::ListResponse;
use applications::netease::api::list_playlist::Playlist;
use applications::netease::api::list_playlist::PlayListData;

use async_std::task;
use isahc::cookies::CookieJar;
#[tauri::command]
pub fn list_playlist(payload: PlayListData, app_state: tauri::State<'_, AppState>) -> Vec<Playlist> {
    let app = app_state.netease_app.lock().unwrap();
    let ListResponse::PlayList(playlists) = task::block_on(app.list(ListRequest::PlayList(payload))).unwrap();
    playlists
}

#[tauri::command]
pub fn test_cookie(app_state: tauri::State<'_, AppState>) {
    let mut app = app_state.netease_app.lock().unwrap();
    let cookie = CookieJar::default();
    app.set_cookie(cookie);
    app.save_cookie();
    log::debug!("save cookie!");
}