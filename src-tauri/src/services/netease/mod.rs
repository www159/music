//! netease app commands
//! resolve `rpc call 'invoke'` from frontend

use crate::applications;
use crate::applications::netease;
use crate::applications::netease::api::get_qrcode::Qrcode;
use crate::services;
use std::rc::Rc;
use std::sync::{Arc};
use parking_lot::{RwLock, Mutex};
use std::time;
use tauri::async_runtime::JoinHandle;
use services::AppState;
use applications::LOG_TARGET;

use tauri::async_runtime;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Service {
    unikey: Arc<RwLock<Option<String>>>,
    qrcode_session: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            unikey: Arc::new(RwLock::new(None)),
            qrcode_session: Arc::new(Mutex::new(None)),
        }
    }

    // TODO use refcell instead of deep clone
    pub fn get_unikey(&self) -> Option<String> {
        let atom = self.unikey.read();
        let ret = atom.clone();
        ret
    }

    pub fn set_unikey(&mut self, str: String) {
        let mut unikey = self.unikey.write();
        *unikey = Some(str);
    }

    pub fn get_qrcode_session(&self) -> Option<JoinHandle<()>> {
        let mut atom = self.qrcode_session.lock();
        let ret = atom.take();
        ret

    }

    pub fn set_qrcode_session(&mut self, task: JoinHandle<()>) {
        let mut atom = self.qrcode_session.lock();
        *atom = Some(task);
    }
}

use applications::netease::GetRequest;
use applications::netease::GetResponse;

#[tauri::command]
pub fn get_qrcode(app_state: tauri::State<AppState>) -> Qrcode {
    let app = app_state.netease_app.clone();
    let mut service = app_state.netease_service.clone();

    if let GetResponse::Qrcode(qrcode) = tauri::async_runtime::block_on(app.get(GetRequest::Qrcode)).unwrap() {
        // store unikey here
        // used by `create_qrcode_session`
        service.set_unikey(qrcode.unikey.clone());
        log::debug!(target: LOG_TARGET, "unikey is {}", service.get_unikey().unwrap());
        return qrcode;
    }
    log::error!(target: LOG_TARGET, "failed to get qrcode");
    return Qrcode::default();
}

use netease::api::get_qrlogin_status::QrloginStatus;

/// create qrcode login authentication session with Netease Cloud Music and frontend
/// # proccess
/// netease::App get qrlogin status(enum QrLoginStatus):
/// - Expire: emit("music-all://step", -1), terminate the session
/// - Scanning: no emit
/// - Confirming: emit("music-all://step", 1)
/// - Success: emit("music-all://step", 1), terminated the session.
#[tauri::command]
pub fn create_qrcode_session(app_state: tauri::State<AppState>) {
    let app = app_state.netease_app.clone();
    let service = app_state.netease_service.clone();
    let emitter = app_state.emit_service.clone();

    let task = tauri::async_runtime::spawn(async move {
        const EVENT: &str = "music-all://step";
        let checkpoint = std::time::Instant::now();

        loop {
            let unikey = service.get_unikey().unwrap();
            if let GetResponse::QrloginStatus(status) = app.get(GetRequest::QrloginStatus(unikey)).await.unwrap() {
                match status {
                    QrloginStatus::Expired => {
                        emitter.emit(EmitterField::MainWindow, EVENT, -1);
                        break;
                    },
                    QrloginStatus::Scanning => {
                        // do nothing
                    },
                    QrloginStatus::Confirming => {
                        emitter.emit(EmitterField::MainWindow, EVENT, 1);
                    }
                    QrloginStatus::Success => {
                        emitter.emit(EmitterField::MainWindow, EVENT, 1);
                        break;
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(700));

            let current = std::time::Instant::now();
            if current - checkpoint > Duration::from_secs(30) {
                log::debug!(target: LOG_TARGET, "session waiting too long time");
                break;
            }
        }    
    });

    let mut service = app_state.netease_service.clone();
    service.set_qrcode_session(task);
    log::debug!(target: LOG_TARGET, "qrcode session stored");

    tauri::async_runtime::spawn(async move {
        // std::thread::sleep(Duration::from_secs(10));
        // task.abort();
        // log::debug!(target: LOG_TARGET, "session loop abort");    
    });
    
    // app.session_loop(unikey, &*emitter);
}

#[tauri::command]
pub fn abort_qrcode_session(app_state: tauri::State<AppState>) {
    log::debug!(target: LOG_TARGET, "aborting qrcode session start");
    let serivce = app_state.netease_service.clone();
    let task = serivce.get_qrcode_session().unwrap();
    task.abort();
    log::debug!(target: LOG_TARGET, "qrcode session abort");
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
    let app = app_state.netease_app.clone();
    let ListResponse::PlayList(playlists) = tauri::async_runtime::block_on(app.list(ListRequest::PlayList(payload))).unwrap();
    playlists
}


use isahc::cookies::CookieJar;

use super::emit::EmitterField;
#[tauri::command]
pub fn test_cookie(app_state: tauri::State<AppState>) {
    let mut app = app_state.netease_app.clone();
    let cookie = CookieJar::default();
    app.set_cookie(cookie);
    app.save_cookie();
    log::debug!(target: applications::LOG_TARGET, "save cookie!");
}

#[tauri::command]
pub fn test_cookie_load(app_state: tauri::State<'_, AppState>) {
    // let mut app = app_state.netease_app.lock().unwrap();
    // app.load_cookie();
    
    let emitter = app_state.emit_service.clone();
    emitter.emit(EmitterField::MainWindow, "test-emit", "ok");
}