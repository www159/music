use serde::Deserialize;
use urlqstring::QueryParams;

use crate::applications::netease::encryption;
use super::super::api;

#[derive(Debug)]
pub enum QrloginStatus {
    Expired,
    Scanning,
    Confirming,
    Success,
}

#[derive(Deserialize)]
struct Json {
    code: i32,
}

pub async fn request(unikey: &str, client: &isahc::HttpClient) -> anyhow::Result<QrloginStatus> {
    let params = QueryParams::from_vec(vec![
        ("type", "1"),
        ("key", unikey)
    ]);

    let res_str = api::request(
        isahc::http::method::Method::POST,
        "/weapi/login/qrcode/client/login",
        &params,
        encryption::EncryptionMethod::WEAPI,
        client,
        api::USERPLATFORM::PC
    ).await?;

    into(res_str)
}

fn into(res_str: String) -> anyhow::Result<QrloginStatus> {
    let obj: Json = serde_json::from_str(&res_str)?;

    match obj.code {
        800 => Ok(QrloginStatus::Expired),
        801 => Ok(QrloginStatus::Scanning),
        802 => Ok(QrloginStatus::Confirming),
        803 => Ok(QrloginStatus::Success),
        _ => anyhow::bail!("status code error"),
    }
}

#[cfg(test)]
mod tests {
    use crate::applications::{self, netease::{GetResponse, GetRequest, App, api::{get_qrcode::Qrcode, get_qrlogin_status::QrloginStatus}}};

    #[actix_rt::test]
    async fn request_works() {
        let app = App::new();
        if let GetResponse::Qrcode(qrcode) = app.get(GetRequest::Qrcode).await.unwrap() {
            if let GetResponse::QrloginStatus(status) = app.get(GetRequest::QrloginStatus(qrcode.unikey)).await.unwrap() {
                match status {
                    QrloginStatus::Scanning => {
                        return;
                    }
                    _ => {
                        panic!();
                    }
               }
            }
        }
        panic!();
    }
}