pub mod list_playlist;

use isahc::{http, AsyncReadResponseExt};
use urlqstring::QueryParams;

use crate::applications::LOG_TARGET;

use super::encryption::{EncryptionMethod, self};


pub enum USERPLATFORM<'a> {
    PC,
    MOBILE,
    OTHER(Option<&'a str>),
}

pub async fn request(
    method: http::Method,
    path: &str,
    params: &QueryParams<'_>,
    encryption_method: EncryptionMethod,
    client: &isahc::HttpClient,
    user_agent: USERPLATFORM<'_>
) -> anyhow::Result<String> {
    //SECTION const
    const BASE_URL: &str = "https://music.163.com";

    const USER_AGENT_LIST: [&str; 14] = [
        // mobile [0:8]
        "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1",
        "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
        "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 6 Build/LYZ28E) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_2 like Mac OS X) AppleWebKit/603.2.4 (KHTML, like Gecko) Mobile/14F89;GameHelper",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 10_0 like Mac OS X) AppleWebKit/602.1.38 (KHTML, like Gecko) Version/10.0 Mobile/14A300 Safari/602.1",
        "Mozilla/5.0 (iPad; CPU OS 10_0 like Mac OS X) AppleWebKit/602.1.38 (KHTML, like Gecko) Version/10.0 Mobile/14A300 Safari/602.1",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:46.0) Gecko/20100101 Firefox/46.0",

        // pc [9:13]
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_5) AppleWebKit/603.2.4 (KHTML, like Gecko) Version/10.1.1 Safari/603.2.4",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:46.0) Gecko/20100101 Firefox/46.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.1058",
    ];

    //~SECTION
    // get crsf token from cookie
    let mut csrf = String::from("");

    if let Some(cookie_jar) = client.cookie_jar() {
        if let Some(cookie) = cookie_jar.get_by_name(
            &BASE_URL.parse()?,
            "__csrf"
        ) {
            csrf = cookie.value().to_string();
        }
    }

    
    match method {
        http::Method::POST => {
            // select `user_agent` from `USER_AGENT_LIST` by `user_agent`
            let user_agent_str = match user_agent {
                USERPLATFORM::PC => USER_AGENT_LIST[rand::random::<usize>() % 5 + 9],
                USERPLATFORM::MOBILE => USER_AGENT_LIST[rand::random::<usize>() % 9],
                USERPLATFORM::OTHER(str_option) => 
                    match str_option {
                        Some(str) => str,
                        None => USER_AGENT_LIST[rand::random::<usize>() % 14]
                    }
            };

            log::debug!(target: LOG_TARGET, "create request with user-agent: {user_agent_str}");

            // encrypt `params` with `encryption_method`
            let body = match encryption_method {
                EncryptionMethod::WEAPI => {
                    log::debug!(target: LOG_TARGET, "begin request with WEAPI encryption");
                    params.add_query_string("crsf_token", &csrf);
                    let json_str = params.json();
                    let (enc_params, enc_sec_key) = encryption::weapi_encryption(json_str.as_str());
        
                    QueryParams::from_vec(vec![
                        ("params", &enc_params),
                        ("encSecKey", &enc_sec_key),
                    ])
                    .stringify()
                }
            };

            // send request
            let uri = format!("{}{}?csrf_token={}", BASE_URL, path, csrf);
            let request = http::Request::post(&uri)
                .body(body).unwrap();

            // recieve response
            client
                .send_async(request)
                .await.map_err(|err| {anyhow::anyhow!("failed to request: {}", err.to_string())})?
                .text()
                .await.map_err(|err| { anyhow::anyhow!("failed to read response body: {}", err.to_string()) })
        },
        _ => Err(anyhow::anyhow!(""))
    }
}
