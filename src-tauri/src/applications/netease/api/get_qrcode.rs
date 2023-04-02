use std::any;

use serde::Deserialize;
use urlqstring::QueryParams;

use crate::applications::netease::encryption;

use super::super::api;

#[derive(Deserialize)]
struct Json {
    code: i32,
    unikey: String,
}

pub async fn request(client: &isahc::HttpClient) -> anyhow::Result<String> {
    let params = QueryParams::from_vec(vec![
        ("type", "1"),
    ]);

    let res_str = api::request(
        isahc::http::method::Method::POST, 
        "/weapi/login/qrcode/unikey",
        &params, 
        encryption::EncryptionMethod::WEAPI, 
        client,
        api::USERPLATFORM::PC
    ).await?;

    into(res_str)
}

fn into(res_str: String) -> anyhow::Result<String> {
    let obj: Json = serde_json::from_str(&res_str)?;

    match obj.code {
        200 => Ok(format!("https://music.163.com/login?codekey={}", obj.unikey)),
        _ => anyhow::bail!("failed to connect")
    }
}

#[cfg(test)]
mod tests {
    use crate::applications::{self, netease::GetResponse};

    #[actix_rt::test]
    async fn request_works() {
        let app = applications::netease::App::new();
        if let GetResponse::Qrcode(raw_data) =  app.get(applications::netease::GetRequest::Qrcode).await.unwrap() {
            let strs = raw_data
                .split('-')
                .collect::<Vec<&str>>();

            assert_eq!(strs.len(), 5);
            return;
        }
        panic!();
    }
}