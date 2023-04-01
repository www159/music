use std::sync::Mutex;

pub mod netease;

// SECTION const
pub const APP_DIR: &str = "music-all";
const LOG_TARGET: &str = "app";
// ~SECTION

pub struct AppState {
    pub netease_app: Mutex<netease::App>
}