use std::collections::hash_map;

use anyhow::Ok;
use serde_json::Value;
use urlqstring::QueryParams;
use super::super::api;
use super::super::encryption;

#[derive(Debug)]
pub struct PlayListData {
    pub order: Option<String>,
    pub cat: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Default)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub cover_img_url: String,
    pub author: String,
}
pub async fn request(songlistdata: PlayListData, client: &isahc::HttpClient) -> anyhow::Result<Vec<Playlist>> {
    // TODO use a macro    
    let order = songlistdata.order.unwrap_or("hot".to_string());
    let cat = songlistdata.cat.unwrap_or("全部".to_string());
    let offset = songlistdata.offset.unwrap_or(0).to_string();
    let limit = songlistdata.limit.unwrap_or(50).to_string();

    // let params_map = hash_map::HashMap::from([

    // ]);
    let params = QueryParams::from_vec(vec![
        ("order", order.as_str()),
        ("cat", cat.as_str()),
        ("offset", offset.as_str()),
        ("limit", limit.as_str())
    ]);

    let res_str = api::request(
        isahc::http::method::Method::POST,
        "/weapi/playlist/list",
        &params,
        encryption::EncryptionMethod::WEAPI,
        client,
        api::USERPLATFORM::PC
    ).await?;
    
    into(res_str)
    // Ok(serde_json::Value::default())
}

fn into(res_str: String) -> anyhow::Result<Vec<Playlist>> {
    let value = serde_json::from_str::<Value>(&res_str)?;
    if value
        .get("code")
        .ok_or_else(|| {
            anyhow::anyhow!("fail to resolve res json")
        })?
        .eq(&200) {
        let mut result = Vec::new();
    }
    Ok(vec![Playlist::default()])
}