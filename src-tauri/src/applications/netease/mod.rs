//! resolve request to Netease Cloud Music api

pub mod api;
mod encryption;

//SECTION const

const TIMEOUT_SEC: u64 = 100;
const APP_DIR: &str = "netease";
const COOKIE_FILENAME: &str = "cookie.json";
const COOKIE_DOMAIN: &str = "music.163.com";
const BASE_URI_LIST: [&str; 12] = [
    "https://music.163.com/",
    "https://music.163.com/eapi/clientlog",
    "https://music.163.com/eapi/feedback",
    "https://music.163.com/api/clientlog",
    "https://music.163.com/api/feedback",
    "https://music.163.com/neapi/clientlog",
    "https://music.163.com/neapi/feedback",
    "https://music.163.com/weapi/clientlog",
    "https://music.163.com/weapi/feedback",
    "https://music.163.com/wapi/clientlog",
    "https://music.163.com/wapi/feedback",
    "https://music.163.com/openapi/clientlog",
];

//~SECTION

//SECTION Rquest api
use api::list_playlist;

use api::get_user_account;

use isahc::cookies::CookieBuilder;

pub enum ListRequest {
    PlayList(list_playlist::PlayListData),
}

pub enum GetRequest {
    UserAccount,
    Qrcode,
    QrloginStatus(String),
}
//~SECTION

//SECTION Response api
pub enum ListResponse {
    PlayList(Vec<list_playlist::Playlist>),
}

pub enum GetResponse {
    UserAccount(UserAccount),
    Qrcode(get_qrcode::Qrcode),
    QrloginStatus(get_qrlogin_status::QrloginStatus),
}
//~SECTION

/// app to resolve request with netease cloud music api
pub struct App {
    client: isahc::HttpClient,
}

use cookie_store::CookieStore;
use isahc::cookies::CookieJar;
use isahc::prelude::Configurable;
use std::fs;
use std::io;
use std::time::Duration;
use tauri::api::path::cache_dir;

use crate::applications::netease::api::get_qrcode;
use crate::services;
use crate::services::netease;

use self::api::get_qrlogin_status;
use self::api::get_user_account::UserAccount;

use super::LOG_TARGET;

// TODO resolve all Err in app;
impl App {
    // ANCHOR new()
    // <1> try to get cookie from cache dir
    // <2> if get cookie, create http client with cookie, goto<3>, else goto<6>
    // <3> try to get login status with the cookie
    // <4> if the cookie is valid, save new cookie to cache dir goto<7>
    // <5> if the cookie expired, clean cookie file, goto<6>
    // <6> request with guest mode
    // <7> request with nomal mode
    //
    // TODO implement nomal mode
    pub fn new() -> Self {
        let client = isahc::HttpClient::builder()
            .timeout(Duration::from_secs(TIMEOUT_SEC))
            .cookies()
            .build()
            .expect("failed to create netease connection");
        Self { client }
    }

    // SECTION cookie store&load
    /// set client cookie with [`isahc::cookie::CookieJar`]
    pub fn set_cookie(&mut self, cookie_jar: isahc::cookies::CookieJar) {
        log::debug!(target: LOG_TARGET, "try to set cookie: {:#?}", cookie_jar);

        self.client = isahc::HttpClient::builder()
            .timeout(Duration::from_secs(TIMEOUT_SEC))
            .cookies()
            .cookie_jar(cookie_jar)
            .build()
            .expect(&format!("failed to create netease connection with cookie"));
    }

    fn cookie_dir(&self) -> anyhow::Result<std::path::PathBuf> {
        let path = cache_dir()
            .ok_or(anyhow::anyhow!("failed to get cookie from cache dir"))?
            .join(super::APP_DIR)
            .join(APP_DIR);

        Ok(path)
    }

    pub fn save_cookie(&self) {
        const MAX_AGE: &str = "31536000";

        if let Some(cookie_jar) = self.client.cookie_jar() {
            // get cookie dir
            let cookie_dir = match self.cookie_dir() {
                Ok(cookie_dir) => cookie_dir,
                Err(err) => {
                    log::error!(target: LOG_TARGET, "{}", err.to_string());
                    return;
                }
            };

            // check cookie dir
            if !cookie_dir.exists() {
                if let Err(err) = fs::create_dir_all(&cookie_dir) {
                    log::error!(
                        target: LOG_TARGET,
                        "failed to create cookie dir: {}",
                        err.to_string()
                    );
                    return;
                }
            }

            // create or open cookie file
            let mut file = match fs::File::create(cookie_dir.join(COOKIE_FILENAME)) {
                Ok(file) => file,
                Err(err) => {
                    log::error!(
                        target: LOG_TARGET,
                        "failed to create cookie file: {}",
                        err.to_string()
                    );
                    return;
                }
            };

            // parse `isahc::Cookie` to `cookie_store::Cookie` to json
            let mut cookie_store = cookie_store::CookieStore::default();
            for base_uri in BASE_URI_LIST {
                let uri = &base_uri.parse().unwrap();
                let url = &base_uri.parse().unwrap();

                for cookie in cookie_jar.get_for_uri(uri) {
                    let cookie_locale = match cookie_store::Cookie::parse(
                        format!(
                            "{}={}; Path={}; Domain={}; MAX-Age={}",
                            cookie.name(),
                            cookie.value(),
                            uri.path(),
                            COOKIE_DOMAIN,
                            MAX_AGE
                        ),
                        url,
                    ) {
                        Ok(cookie_locale) => cookie_locale,
                        Err(err) => {
                            log::error!(
                                target: LOG_TARGET,
                                "failed to parse iashc::Cookie to cookie_store::Cookie: {}",
                                err.to_string()
                            );
                            return;
                        }
                    };

                    cookie_store.insert(cookie_locale, url).unwrap();
                }
                cookie_store.save_json(&mut file).unwrap();
            }
        } else {
            log::debug!(target: LOG_TARGET, "no cookie in http client");
        }
    }

    pub fn load_cookie(&mut self) {
        // open cookie file
        let file = match fs::File::open(self.cookie_dir().unwrap().join(COOKIE_FILENAME)) {
            Ok(file) => file,
            Err(err) => {
                log::error!(
                    target: LOG_TARGET,
                    "failed to open {}: {}",
                    COOKIE_FILENAME,
                    err.to_string()
                );
                return;
            }
        };

        // parse json to `cookie_store::Cookie` to `isahc::cookie`
        match CookieStore::load_json(io::BufReader::new(file)) {
            Ok(cookie_store) => {
                let cookie_jar = CookieJar::default();
                for base_uri in BASE_URI_LIST {
                    for cookie_locale in cookie_store.matches(&base_uri.parse().unwrap()) {
                        let cookie =
                            match CookieBuilder::new(cookie_locale.name(), cookie_locale.value())
                                .domain(COOKIE_DOMAIN)
                                .path(cookie_locale.path().unwrap_or("/"))
                                .build()
                            {
                                Ok(cookie) => cookie,
                                Err(err) => {
                                    log::error!(
                                        target: LOG_TARGET,
                                        "failed to parse locale cookie: {}",
                                        err.to_string()
                                    );
                                    return;
                                }
                            };

                        if let Err(err) = cookie_jar.set(cookie, &base_uri.parse().unwrap()) {
                            log::error!(
                                target: LOG_TARGET,
                                "failed to set cookie: {}",
                                err.to_string()
                            );
                        }
                    }
                }
                log::debug!(target: LOG_TARGET, "load cookie: {:#?}", cookie_jar);
                self.set_cookie(cookie_jar);
            }
            Err(err) => {
                log::error!(
                    target: LOG_TARGET,
                    "failed to read {}: {}",
                    COOKIE_FILENAME,
                    err.to_string()
                );
                return;
            }
        }
    }

    pub async fn session_loop(&self, unikey: String, emitter: &services::emit::Service) {
        const EVENT: &str = "music-all://step";
        tauri::async_runtime::spawn(async move {
            log::debug!(target: LOG_TARGET, "get unikey: {}", unikey);
            log::debug!(target: LOG_TARGET, "get emitter: {:#?}", emitter);
        });
    } 
    // ~SECTION cookie store&load

    pub async fn get(&self, api: GetRequest) -> Option<GetResponse> {
        match api {
            GetRequest::UserAccount => {
                log::debug!(target: LOG_TARGET, "try to request GET with payload None");
                match get_user_account::request(&self.client).await {
                    Ok(response) => Some(GetResponse::UserAccount(response)),
                    Err(err) => {
                        log::error!(
                            target: LOG_TARGET,
                            "failed to request Get login status: {}",
                            err.to_string()
                        );
                        None
                    }
                }
            }
            GetRequest::Qrcode => {
                log::debug!(target: LOG_TARGET, "try to request Get with payload None");
                match get_qrcode::request(&self.client).await {
                    Ok(response) => Some(GetResponse::Qrcode(response)),
                    Err(err) => {
                        log::error!(
                            target: LOG_TARGET,
                            "failed to request Get qrcode: {}",
                            err.to_string()
                        );
                        None
                    }
                }
            }

            GetRequest::QrloginStatus(unikey) => {
                log::debug!(
                    target: LOG_TARGET,
                    "try to request Get with payload: {:#?}",
                    unikey
                );
                match get_qrlogin_status::request(&unikey, &self.client).await {
                    Ok(reponse) => Some(GetResponse::QrloginStatus(reponse)),
                    Err(err) => {
                        log::error!(
                            target: LOG_TARGET,
                            "failed to request Get qrcode login status: {}",
                            err.to_string()
                        );
                        None
                    }
                }
            }
        }
    }

    pub async fn list(&self, api: ListRequest) -> Option<ListResponse> {
        match api {
            ListRequest::PlayList(playlist_data) => {
                log::debug!(
                    target: LOG_TARGET,
                    "try to request LIST with payload: {:#?}",
                    playlist_data
                );
                match list_playlist::request(playlist_data, &self.client).await {
                    Ok(response) => Some(ListResponse::PlayList(response)),
                    Err(err) => {
                        log::error!(
                            target: LOG_TARGET,
                            "failed to request List playlists: {}",
                            err.to_string()
                        );
                        None
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cookie_dir_works() {
        let app = App::new();
        let path = cache_dir().unwrap();
        let path = path.join(super::super::APP_DIR).join(APP_DIR);

        assert_eq!(path.to_str(), app.cookie_dir().unwrap().to_str());
    }
}
