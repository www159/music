use crate::applications;

use applications::AppState;

use applications::netease::ListRequest;
use applications::netease::ListResponse;
use applications::netease::api::list_playlist::Playlist;
use applications::netease::api::list_playlist::PlayListData;
use async_std::task;
#[tauri::command]
pub fn list_playlist(payload: PlayListData, app_state: tauri::State<'_, AppState>) -> Vec<Playlist> {
    let app = app_state.netease_app.lock().unwrap();
    let ListResponse::PlayList(playlists) = task::block_on(app.list(ListRequest::PlayList(payload))).unwrap();
    playlists
}