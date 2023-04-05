//! netease app commands
//! resolve `rpc call 'invoke'` from frontend

use crate::applications;
use crate::applications::netease;
use crate::applications::netease::api::get_qrcode::Qrcode;
use crate::services;
use std::sync::{Mutex, Arc};
use std::time;
use async_std::future::timeout;
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

    let (tx, mut rx) = tauri::async_runtime::channel(1);

    
    tauri::async_runtime::block_on(tx.send((
        app_state.netease_app.clone(), 
        app_state.netease_service.clone(), 
        app_state.emit_service.clone()))).unwrap();
    // let unikey = app_state.netease_service.lock().unwrap().get_unikey().unwrap();
    // let emitter = app_state.emit_service.lock().unwrap();

    let task = tauri::async_runtime::spawn(async move {
        if let Some(message) = rx.recv().await {
            let (app, service, emitter) = message;
            let app = app.lock().unwrap();
            let service = service.lock().unwrap();
            let emitter = emitter.lock().unwrap();

            log::debug!(target: LOG_TARGET, "receive params");

            let begin = time::Instant::now();
            loop {
                let current = time::Instant::now();
                if current - begin > Duration::from_secs(5) {
                    log::debug!(target: LOG_TARGET, "loop 5 secs");
                    break;
                }
                log::debug!(target: LOG_TARGET, "next loop!");
            }
        }
    });

    std::thread::sleep(Duration::from_secs(2));
    log::debug!(target: LOG_TARGET, "force kill task");
    task.abort();

    // app.session_loop(unikey, &*emitter);
}

// #[tauri::command]
// async fn command_name<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
//   Ok(())
// }

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