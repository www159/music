use serde::{Deserialize, Serialize};
use urlqstring::QueryParams;
use crate::applications::netease::encryption;

use super::super::api;

#[derive(Debug, Default, Serialize)]
pub struct UserAccount {
    pub user_id: u64,
    pub nickname: String,
    pub avatar_img_url: String,
}

#[derive(Deserialize)]
struct Json {
    pub code: i32,
    pub profile: Option<ProfileJson>,
}

#[derive(Deserialize)]
struct  ProfileJson {
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub nickname: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_img_url: String,
}

pub async fn request(client: &isahc::HttpClient) -> anyhow::Result<UserAccount> {
    let res_str = api::request(
        isahc::http::method::Method::POST,
        "/api/nuser/account/get",
        &QueryParams::from_vec(vec![]),
        encryption::EncryptionMethod::WEAPI,
        client,
        api::USERPLATFORM::PC
    ).await?;

    into(res_str)
}

fn into(res_str: String) -> anyhow::Result<UserAccount> {
    let obj: Json = serde_json::from_str(&res_str)?;
    
    match obj.code {
        200 => {
            if let Some(profile) = obj.profile {
                return Ok(UserAccount {
                    user_id: profile.user_id,
                    avatar_img_url: profile.avatar_img_url,
                    nickname: profile.nickname
                });
            }
            else {
                anyhow::bail!("failed to request login status");
            }

        },
        _ => {
            anyhow::bail!("failed to connect");
        }
    } 
}

async fn request_cookie(csrf: &str, cookie: &str, client: &isahc::HttpClient) -> anyhow::Result<UserAccount> {
    let res_str = api::request_cookie(
        isahc::http::method::Method::POST,
        "/api/nuser/account/get",
        &QueryParams::from_vec(vec![]),
        encryption::EncryptionMethod::WEAPI,
        client,
        api::USERPLATFORM::PC,
        cookie,
        csrf
    ).await?;

    println!("{}", res_str);
    into(res_str)
}

#[cfg(test)]
mod tests {
    use cookie_store::CookieStore;
    use isahc::cookies::CookieJar;

    use crate::applications::netease;

    use super::*;
    #[actix_rt::test]
    async fn request_works() {
        const cookie_str: &str = "_ntes_nnid=7e922546bbf2ec6589d41114d333da5f,1678520593206; _ntes_nuid=7e922546bbf2ec6589d41114d333da5f; NMTID=00O2g5huToSbpF5G0Q8kfd_D5r2O-cAAAGGz5-rfw; WEVNSM=1.0.0; WNMCID=dzfbkg.1678520593377.01.0; WM_TID=ewx4g1lxRipAAURQFUbUebD6kq6NoIxf; __snaker__id=vLybqdwhQpbTnaA8; YD00000558929251:WM_TID=CfzIInFg5wBEFRVRQUbQKwJ+Jj0wnFM/; ntes_kaola_ad=1; playerid=36574408; YD00000558929251:WM_NI=zVbFO82UNXLRd1kmB1dXENA5KwadgQuelTf7BXttfVIN4CPkGp+P2ooQA9lT++BWjiRh9ZNQeLQaR9l2k0jArQofFWFrm3grtQwSuxzan3jvEr8hgzmn1fuXQNCBl/ldWEs=; YD00000558929251:WM_NIKE=9ca17ae2e6ffcda170e2e6eea5e568f5f1a282b86faebc8ea6c84a929f9a82d445f3f5b7d4f273af8eafacc22af0fea7c3b92aba8aaea7ce34f28b82b9cf5aa8efa58ec13ba8888bafc43f89a9af85e65bb7bba4b5e53d8288bc95e55e8b92fca4d36185b0e5b1b165ad8f8fdaef60e9f58eb8e96392b8f7bbcd5ba7949f82d133b0eea5d1e94bb28fa187c55d86aea3d4e67ab88fb9b4f874babfa2d4e97e87aef889e672afb2ff87f85493928da3ef25b0bc99a6f237e2a3; gdxidpyhxdE=jQLXRrGNd9XcECGZsY76fCdxilnejyoieV+mHGODnQ/9\\K119bU7oM1WYr5/Uf78N2z0kPIUm\\rJhUUustwrOPDpuwf\\O8iZogH0SMOwS7UDi1yLzpuS2nOJnID90t8PXQxyyR5Pq3Z6Ci2nZ5a9g7090asiE0f0Zg9ikxg2TrU0Ru23:1680377018210; NTES_YD_SESS=6tli3GVFWMgsMZA9vME6HZwjIAERB2h7_I0_A6LWt7SNVQXFV_orx59_ep0t_Uv6eeMMtHQFWt2Ircrdbc0BORwiUXTDKGz2Ukyo3i37Ds5ftk7B1OiM_zKjUWVTtePS9OXFSuZ4Gh5N41IdacFpjeMBOpeDB.ufWNbKpmnVOQl1eP.WbQ5BhQdORmEnbkGol5NInIIHfVV9OACAgr1wEhZwHA8SET_FiP2mJnnUuG13M; S_INFO=1680376192|0|0&60##|15857847909; P_INFO=15857847909|1680376192|1|music|00&99|null&null&null#hub&420100#10#0|&0||15857847909; __remember_me=true; __csrf=44c647d4eaff56ad6adc544c9668be65; MUSIC_U=15c5ca7d070c1e292675366f29b0da35e0e9228466eb6765ab68005c08c93e02993166e004087dd3bc0fdc4821a921c38d7b9bd77d76e8c359941a9e0a8e405921bc8e2ef03d8d9dd4dbf082a8813684; JSESSIONID-WYYY=cGsf9/RjvyUxIGUWXXuMY6hB+VYd5TblgVk2iqBj08QP5HeU0W8VATPT7+jTbNEE1+H2e/i9GKh6BOssr2mN5sHPa6X/fhGJFd7TZh\\nlfCwouJHVgzPWV\\1VDlf0gbVHQJRZ7S2J\\8rlPiQizUMzUrBAK59JXc8DrFgRB7imv9SXvA:1680408963792; _iuqxldmzr_=33; WM_NI=4hf1eAkQizlOYBHrYO0clFRJ4fzXP04Vw2Ye2eK1DZC55oDH7yvChAqI+ci0eoCNyBHLcpeMRkr38/hLoHeUWtJtLBsW6sRPdUezLGzreNNTDVYBG5NMMmdMdZs1/QndZ2U=; WM_NIKE=9ca17ae2e6ffcda170e2e6eecce653f48ef8a4b57386e78ea7d54f838a8e87c45cba8f9bb7fb7db3979ba6cc2af0fea7c3b92af8b8aad2e84ba98fa79bef459b9e8893dc45f4bda4a7d142f8bab6b9d33cf88ab9d3dc6ff2bb85d2c75281ebfb90c744f5a7baa4eb4bb397e584ec3ca7b18daffc4a8c94ad91e84792b5bed4bb25ac94a1a9e85f95ac8695cf4ab189fa94d15ca88bab9bcf4dbaaeabafcd60888a97a3c63f8e93838ee747a886bdd2ae5baebc96d3d837e2a3";
        let client = isahc::HttpClient::builder().cookies().build().unwrap();
        let res = request_cookie(&"44c647d4eaff56ad6adc544c9668be65", cookie_str, &client).await.unwrap();
        println!("{:#?}", res);
    }
}