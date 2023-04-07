use anyhow::Ok;
use serde::{Serialize, Deserialize};
use urlqstring::QueryParams;

use crate::applications::netease::encryption::{self, EncryptionMethod};
use super::super::api;

#[derive(Debug, Deserialize)]
pub struct PlaylistDetailData {
    pub id: u64,
    pub size: Option<u32>,
}

#[derive(Debug, Serialize, Default)]
pub struct PlaylistDetail {
    pub id: u64,
    pub name: String,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    pub description: String,
    #[serde(rename = "createTime")]
    pub create_time: u64,
    #[serde(rename = "trackUpdateTime")]
    pub track_update_time: u64,
    pub songs: Vec<Song>,
}

#[derive(Debug, Default, Serialize)]
pub struct Song {
    pub id: u64,
    pub name: String,
    pub singers: Vec<String>,
    pub album: String,
    #[serde(rename = "albumId")]
    pub album_id: u64,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    pub duration: u64,
    // pub copyright: String
}

#[derive(Deserialize)]
struct Json {
    playlist: PlaylistJson,
    // privileges: Vec<PrivilegeJson>
}

#[derive(Deserialize)]
struct PlaylistJson {
    pub id: u64,
    pub name: String,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    pub description: String,
    #[serde(rename = "createTime")]
    pub create_time: u64,
    #[serde(rename = "trackUpdateTime")]
    pub track_update_time: u64,
    pub tracks: Vec<TrackJson>
}

#[derive(Deserialize)]
struct TrackJson {
    pub id: u64,
    pub name: String,
    #[serde(rename = "ar")]
    pub singers: Vec<ArJson>,
    #[serde(rename = "al")]
    pub album: AlJson,
    #[serde(rename = "dt")]
    pub duration: u64,
}

#[derive(Deserialize)]
struct ArJson {
    pub name: String
}

#[derive(Deserialize)]
struct AlJson {
    pub name: String,
    pub id: u64,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
}

pub async fn request(playlist_detail_data: PlaylistDetailData, client: &isahc::HttpClient) -> anyhow::Result<PlaylistDetail> {
    let id = playlist_detail_data.id.to_string();
    let size = playlist_detail_data.size.unwrap_or(1000).to_string();

    let params = QueryParams::from_vec(vec![
        ("id", id.as_str()),
        ("n", size.as_str()),
    ]);

    let res_str = api::request(
        isahc::http::method::Method::POST, 
        "/weapi/v6/playlist/detail", 
        &params,
        EncryptionMethod::WEAPI, 
        client, 
        api::USERPLATFORM::PC)
        .await?;

    into(res_str)
}

fn into(res_str: String) -> anyhow::Result<PlaylistDetail> {
    let obj: Json = serde_json::from_str(&res_str)?;
    let Json { playlist, .. } = obj;
    let PlaylistJson { tracks, .. } = playlist; 

    let mut songs = vec![];

    for track in tracks {
        songs.push(Song {
            id: track.id,
            name: track.name,
            album: track.album.name,
            album_id: track.album.id,
            cover_img_url: track.album.pic_url,
            duration: track.duration,
            singers: track.singers.iter().map(|singer| singer.name.clone()).collect(),
            // singers: vec![String::from("")],
        })
    }

    Ok(PlaylistDetail {
        id: playlist.id,
        name: playlist.name,
        cover_img_url: playlist.cover_img_url,
        description: playlist.description,
        create_time: playlist.create_time,
        track_update_time: playlist.track_update_time,
        songs,
    })
}

#[cfg(test)]
mod tests {
    use crate::applications;
    use super::*;

    #[actix_rt::test]
    async fn request_works() {
        let mut app = applications::netease::App::new();
        app.load_cookie();

        let response = request(PlaylistDetailData { 
            id: 26467411,
            size: None,
        }, &app.client).await.unwrap();

        println!("{:#?}", response);
    }
}
