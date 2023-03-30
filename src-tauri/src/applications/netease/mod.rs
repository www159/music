mod encryption;
pub mod api;

//SECTION const

const TIMEOUT_SEC: u64 = 100;

//~SECTION

//SECTION Rquest api
use api::list_playlist;
use isahc::prelude::Configurable;

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
        .build()
        .expect("failed to create netease app");
        Self {
            client
        }
    }

    
    pub async fn list(&self, api: ListRequest) -> anyhow::Result<ListResponse> {
        match api {
            ListRequest::PlayList(playlist_data) => {
                let response = list_playlist::request(playlist_data, &self.client).await?;
                Ok(ListResponse::PlayList(response))
            }
        }
    }
}
