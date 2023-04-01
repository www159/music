use tauri::Manager;

use self::log::init_log;

pub mod netease;
pub mod log;

use crate::applications;

pub fn setup(app: & mut tauri::App) {
    // setup log service
    init_log().unwrap();

    // setup global state
    let netease_app = applications::netease::App::new();
    app.manage(applications::AppState {
        netease_app: std::sync::Mutex::new(netease_app)
    });
}