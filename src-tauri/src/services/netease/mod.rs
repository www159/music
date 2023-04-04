//! netease app commands
//! resolve `rpc call 'invoke'` from frontend

use crate::applications;
use crate::applications::netease;
use crate::applications::netease::api::get_qrcode::Qrcode;
use crate::services;
use std::sync::{Mutex, Arc};
use services::AppState;
use applications::LOG_TARGET;

use async_std::task;
use tauri::async_runtime;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Service {
    unikey: Option<String>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            unikey: None,
        }
    }

    // TODO use refcell instead of deep clone
    pub fn get_unikey(&self) -> Option<String> {
        self.unikey.clone()
    }

    pub fn set_unikey(&mut self, str: String) {
        self.unikey = Some(str);
    }


}

use applications::netease::GetRequest;
use applications::netease::GetResponse;

#[tauri::command]
pub fn get_qrcode(app_state: tauri::State<AppState>) -> Qrcode {
    let app = app_state.netease_app.lock().unwrap();
    let mut service = app_state.netease_service.lock().unwrap();

    if let GetResponse::Qrcode(qrcode) = task::block_on(app.get(GetRequest::Qrcode)).unwrap() {
        // store unikey here
        // used by `create_qrcode_session`
        service.set_unikey(qrcode.unikey.clone());
        return qrcode;
    }
    log::error!(target: LOG_TARGET, "failed to get qrcode");
    return Qrcode::default();
}

/// create qrcode login authentication session with Netease Cloud Music and frontend
/// # proccess
/// netease::App get qrlogin status(enum QrLoginStatus):
/// - Expire: emit("music-all://step", -1), terminate the session
/// - Scanning: no emit
/// - Confirming: emit("music-all://step", 1)
/// - Success: emit("music-all://step", 1), erminated the session.
#[tauri::command]
pub fn create_qrcode_session(app_state: tauri::State<AppState>) {
    use applications::netease::api::get_qrlogin_status::QrloginStatus;


    let app = app_state.netease_app.lock().unwrap();
    let unikey = app_state.netease_service.lock().unwrap().get_unikey().unwrap();
    let emitter = app_state.emit_service.lock().unwrap();

    app.session_loop(unikey, &*emitter);
}

use applications::netease::ListRequest;
use applications::netease::ListResponse;

use applications::netease::api::list_playlist::Playlist;
use applications::netease::api::list_playlist::PlayListData;

#[tauri::command]
pub fn list_playlist(payload: PlayListData, app_state: tauri::State<'_, AppState>) -> Vec<Playlist> {
    let app = app_state.netease_app.lock().unwrap();
    let ListResponse::PlayList(playlists) = tauri::async_runtime::block_on(app.list(ListRequest::PlayList(payload))).unwrap();
    playlists
}


use isahc::cookies::CookieJar;

use super::emit::EmitterField;
#[tauri::command]
pub fn test_cookie(app_state: tauri::State<AppState>) {
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