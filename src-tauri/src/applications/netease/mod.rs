mod encryption;
pub mod api;

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
use isahc::cookies::CookieBuilder;

pub enum ListRequest {
    PlayList(list_playlist::PlayListData),
}
//~SECTION

//SECTION Response api
pub enum ListResponse {
    PlayList(Vec<list_playlist::Playlist>),
}
//~SECTION

//ANCHOR Netease app
pub struct App {
    client: isahc::HttpClient,
}

use std::time::Duration;
use std::fs;
use std::io;
use cookie_store::CookieStore;
use isahc::cookies::CookieJar;
use isahc::prelude::Configurable;
use tauri::api::path::cache_dir;

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
        Self {
            client
        }
    }

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
            
            let cookie_dir = match self.cookie_dir() {
                Ok(cookie_dir) => cookie_dir,
                Err(err) => {
                    log::error!(target: LOG_TARGET, "{}", err.to_string());
                    return;
                }
            };

            if !cookie_dir.exists() {
                if let Err(err) = fs::create_dir_all(&cookie_dir) {
                    log::error!(target: LOG_TARGET, "failed to create cookie dir: {}", err.to_string());
                    return;
                }
            }

            let mut file = match fs::File::create(cookie_dir.join(COOKIE_FILENAME)) {
                Ok(file) => file,
                Err(err) => {
                    log::error!(target: LOG_TARGET, "failed to create cookie file: {}", err.to_string());
                    return;
                }
            };

            let mut cookie_store = cookie_store::CookieStore::default();
            for base_uri in BASE_URI_LIST {
                let uri = &base_uri.parse().unwrap();
                let url = &base_uri.parse().unwrap();
                
                // parse `iashc::Cookie` to `cookie_store::Cookie`
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
                        url
                    ) {
                        Ok(cookie_locale) => cookie_locale,
                        Err(err) => {
                            log::error!(target: LOG_TARGET, "failed to parse iashc::Cookie to cookie_store::Cookie: {}", err.to_string());
                            return;
                        }
                    };

                    cookie_store.insert(cookie_locale, url).unwrap();
                }
                cookie_store.save_json(&mut file).unwrap();
            }
        }
        else {
            log::debug!(target: LOG_TARGET, "no cookie in http client");
        }
    }

    pub fn load_cookie(&self) -> Option<CookieJar> {
        let file = match fs::File::open(self
            .cookie_dir().unwrap()
            .join(COOKIE_FILENAME)) {
                Ok(file) => file,
                Err(err) => {
                    log::error!(target: LOG_TARGET, "failed to open {}: {}", COOKIE_FILENAME, err.to_string());
                    return None;
                }
            };
        
        match CookieStore::load_json(io::BufReader::new(file)) {
            Ok(cookie_store) => {
                let cookie_jar = CookieJar::default();
                for base_uri in BASE_URI_LIST {
                    for cookie_locale in cookie_store.matches(&base_uri.parse().unwrap()) {
                        let cookie = match CookieBuilder::new(cookie_locale.name(), cookie_locale.value())
                            .domain(COOKIE_DOMAIN)
                            .path(cookie_locale.path().unwrap_or("/"))
                            .build() {
                                Ok(cookie) => cookie,
                                Err(err) => {
                                    log::error!(target: LOG_TARGET, "failed to parse locale cookie: {}", err.to_string());
                                    return None;
                                }
                            };

                        if let Err(err) = cookie_jar.set(cookie, &base_uri.parse().unwrap()) {
                            log::error!(target: LOG_TARGET, "failed to set cookie: {}", err.to_string());
                        }
                    }
                }
                return Some(cookie_jar);
            }
            Err(err) => {
                log::error!(target: LOG_TARGET, "failed to read {}: {}", COOKIE_FILENAME, err.to_string());
                return None;
            }
        }
    }

    
    pub async fn list(&self, api: ListRequest) -> Option<ListResponse> {
        match api {
            ListRequest::PlayList(playlist_data) => {
                log::debug!(target: LOG_TARGET, "try to request with payload: {:#?}", playlist_data);
                let response = match list_playlist::request(playlist_data, &self.client).await {
                    Ok(response) => response,
                    Err(err) => {
                        log::error!(target: LOG_TARGET, "failed to request the list : {}", err.to_string());
                        return None;
                    }
                };
                Some(ListResponse::PlayList(response))
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