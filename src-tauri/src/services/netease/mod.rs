use crate::services::AppState;
use applications::netease::ListRequest;
use applications::netease::ListResponse;
use applications::netease::api::list_playlist::Playlist;
use applications::netease::api::list_playlist::PlayListData;

use async_std::task;

use crate::applications;
#[tauri::command]
pub fn list_playlist(payload: PlayListData, app_state: tauri::State<'_, AppState>) -> Vec<Playlist> {
    let app = app_state.netease_app.lock().unwrap();
    let ListResponse::PlayList(playlists) = task::block_on(app.list(ListRequest::PlayList(payload))).unwrap();
    playlists
}


use isahc::cookies::CookieJar;

use super::emit::EmitterField;
#[tauri::command]
pub fn test_cookie(app_state: tauri::State<'_, AppState>) {
    let mut app = app_state.netease_app.lock().unwrap();
    let cookie = CookieJar::default();
    app.set_cookie(cookie);
    app.save_cookie();
    log::debug!(target: applications::LOG_TARGET, "save cookie!");
}

#[tauri::command]
pub fn test_cookie_load(app_state: tauri::State<'_, AppState>) {
    // let mut app = app_state.netease_app.lock().unwrap();
    // app.load_cookie();
    
    let emitter = app_state.emit_service.lock().unwrap();
    emitter.emit(EmitterField::MainWindow, "test-emit", "ok");
}