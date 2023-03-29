use serde::Deserialize;
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
    pub id: u64,
    pub name: String,
    pub cover_img_url: String,
    pub author: String,
}

#[derive(Deserialize)]
struct Json {
    pub playlists: Vec<PlayListJson>,
    pub code: i64
}

#[derive(Deserialize)]
struct PlayListJson {
    pub id: u64,
    pub name: String,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    pub creator: CreatorJson
}

#[derive(Deserialize)]
struct CreatorJson {
   pub  nickname: String,
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
    let obj: Json = serde_json::from_str(&res_str)?;

    let mut result = vec![];
    match obj.code {
        200 => {
            for playlist in obj.playlists {
                result.push(Playlist {
                    id: playlist.id,
                    name: playlist.name,
                    cover_img_url: playlist.cover_img_url,
                    author: playlist.creator.nickname

                })
            }

            Ok(result)
        },
        _ => Err(anyhow::anyhow!("bad code"))
    }
    // anyhow::Ok(vec![Playlist::default()])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn request_works() {
    	// guest api
    	let client = isahc::HttpClient::new().unwrap();
    	let requestData = PlayListData {
    	    order: None,
    	    cat: None,
    	    limit: None,
    	    offset: None,
    	};
    	let playlists = request(requestData, &client).await.unwrap();
    	println!("{:#?}", playlists);
    }
    #[test]
    fn into_works() {
        let json_str = "{\"code\":200,\"playlists\":[{\"name\":\"助眠辑 | 自然音，伴灵动乐符萦绕耳畔\",\"id\":2075587022,\"trackNumberUpdateTime\":1533916733093,\"status\":0,\"userId\":44530116,\"createTime\":1517015611709,\"updateTime\":1533916733093,\"subscribedCount\":522185,\"trackCount\":104,\"cloudTrackCount\":0,\"coverImgUrl\":\"http://p1.music.126.net/sixunTcvD_IXeVqxZnpHkA==/109951163452086313.jpg\",\"coverImgId\":109951163452086320,\"description\":\"歌单收录以自然声效为背景的纯音乐，海浪、雨水、溪流、虫鸣、鸟叫、车辆川流......带上耳机，愿能此声能将室外噪音减到最低，给你心如止水的平静，体验一场心灵Spa，携这份安稳沉沉入睡。\\n\\n歌曲分布：\\n01-16 雷雨声背景纯音乐\\n17-28 蝉鸣背景纯音乐\\n29-65 海水、溪流背景纯音乐\\n66-90 鸟鸣与溪水背景纯音乐\\n91-101 日常生活收音\\n102-105 海豚声背景纯音乐\\n\\n音乐人推荐：\\nDan Gibson，植地雅哉，Della，α·Pav\\n\\n封面：\\nVincent Willem van Gogh 丨 星月夜\\n\\nMay you rest in a deep and dreamless slumber.  愿你陷入一夜无梦的沉睡。\\n\\nGoodNight.\\n晚安。\\n\\n2018.02.07夜\",\"tags\":[\"轻音乐\",\"放松\",\"安静\"],\"playCount\":28405026,\"trackUpdateTime\":1679484737766,\"specialType\":0,\"totalDuration\":0,\"creator\":{\"defaultAvatar\":false,\"province\":1000000,\"authStatus\":0,\"followed\":false,\"avatarUrl\":\"http://p1.music.126.net/AN5tbjwES1lZI0hUWMvWUg==/109951164873118898.jpg\",\"accountStatus\":0,\"gender\":2,\"city\":1003000,\"birthday\":959875200000,\"userId\":44530116,\"userType\":207,\"nickname\":\"名侦探-柯北\",\"signature\":\"Be water.\",\"description\":\"\",\"detailDescription\":\"\",\"avatarImgId\":109951164873118900,\"backgroundImgId\":109951164387155260,\"backgroundUrl\":\"http://p1.music.126.net/CDfa13FIaQlcK328ucJpog==/109951164387155268.jpg\",\"authority\":0,\"mutual\":false,\"expertTags\":[\"华语\",\"流行\",\"欧美\"],\"experts\":{\"2\":\"资讯(生活)\"},\"djStatus\":0,\"vipType\":11,\"remarkName\":null,\"authenticationTypes\":1581120,\"avatarDetail\":{\"userType\":207,\"identityLevel\":4,\"identityIconUrl\":\"https://p5.music.126.net/obj/wo3DlcOGw6DClTvDisK1/4761340194/0903/b735/7c7a/0dddcdf78047d397d24125840e54ab5b.png\"},\"anchor\":false,\"avatarImgIdStr\":\"109951164873118898\",\"backgroundImgIdStr\":\"109951164387155268\",\"avatarImgId_str\":\"109951164873118898\"},\"tracks\":null,\"subscribers\":[{\"defaultAvatar\":false,\"province\":1000000,\"authStatus\":0,\"followed\":false,\"avatarUrl\":\"http://p1.music.126.net/QfhcTpxZWWa50fgK751LDQ==/109951167929023131.jpg\",\"accountStatus\":0,\"gender\":2,\"city\":1003100,\"birthday\":-2209017600000,\"userId\":4964307769,\"userType\":0,\"nickname\":\"冷圈战士1点0\",\"signature\":\"\",\"description\":\"\",\"detailDescription\":\"\",\"avatarImgId\":109951167929023140,\"backgroundImgId\":109951162868126480,\"backgroundUrl\":\"http://p1.music.126.net/_f8R60U9mZ42sSNvdPn2sQ==/109951162868126486.jpg\",\"authority\":0,\"mutual\":false,\"expertTags\":null,\"experts\":null,\"djStatus\":0,\"vipType\":11,\"remarkName\":null,\"authenticationTypes\":0,\"avatarDetail\":null,\"anchor\":false,\"avatarImgIdStr\":\"109951167929023131\",\"backgroundImgIdStr\":\"109951162868126486\",\"avatarImgId_str\":\"109951167929023131\"}],\"subscribed\":null,\"commentThreadId\":\"A_PL_0_2075587022\",\"newImported\":false,\"adType\":0,\"highQuality\":true,\"privacy\":0,\"ordered\":true,\"anonimous\":false,\"coverStatus\":3,\"recommendInfo\":null,\"socialPlaylistCover\":null,\"recommendText\":null,\"shareCount\":14195,\"coverImgId_str\":\"109951163452086313\",\"alg\":\"alg_sq_offline\",\"commentCount\":2537},{\"name\":\"那些你熟悉却又不知道名字的轻音乐\",\"id\":26467411,\"trackNumberUpdateTime\":1503235871338,\"status\":0,\"userId\":7017009,\"createTime\":1409379332621,\"updateTime\":1641178920444,\"subscribedCount\":2644459,\"trackCount\":44,\"cloudTrackCount\":0,\"coverImgUrl\":\"http://p1.music.126.net/8fQ9jzTJnMweLNm7VoyPSw==/6623458045881301.jpg\",\"coverImgId\":6623458045881301,\"description\":\"只听一小会你就会爱上它。音乐就像生活，有开心也会有低落的时候，anyway，enjoy it ~\",\"tags\":[\"轻音乐\",\"学习\",\"治愈\"],\"playCount\":104097352,\"trackUpdateTime\":1679496785670,\"specialType\":0,\"totalDuration\":0,\"creator\":{\"defaultAvatar\":false,\"province\":440000,\"authStatus\":0,\"followed\":false,\"avatarUrl\":\"http://p1.music.126.net/o4_ZxcOrFStGrW_M1QH2rA==/109951164398321845.jpg\",\"accountStatus\":0,\"gender\":1,\"city\":440300,\"birthday\":662659200000,\"userId\":7017009,\"userType\":0,\"nickname\":\"Lelow\",\"signature\":\"道路漫长，你要善良。\",\"description\":\"\",\"detailDescription\":\"\",\"avatarImgId\":109951164398321840,\"backgroundImgId\":109951163153437020,\"backgroundUrl\":\"http://p1.music.126.net/mlfNJS_zMZckIoHW4MrIaQ==/109951163153437018.jpg\",\"authority\":0,\"mutual\":false,\"expertTags\":null,\"experts\":null,\"djStatus\":0,\"vipType\":0,\"remarkName\":null,\"authenticationTypes\":0,\"avatarDetail\":null,\"anchor\":false,\"avatarImgIdStr\":\"109951164398321845\",\"backgroundImgIdStr\":\"109951163153437018\",\"avatarImgId_str\":\"109951164398321845\"},\"tracks\":null,\"subscribers\":[{\"defaultAvatar\":false,\"province\":1000000,\"authStatus\":0,\"followed\":false,\"avatarUrl\":\"http://p1.music.126.net/x5pz10TsYSHuYGUb_XZmGg==/109951166617939738.jpg\",\"accountStatus\":0,\"gender\":1,\"city\":1010000,\"birthday\":894816000000,\"userId\":1378438596,\"userType\":0,\"nickname\":\"真的别再恶心我了\",\"signature\":\"\",\"description\":\"\",\"detailDescription\":\"\",\"avatarImgId\":109951166617939740,\"backgroundImgId\":109951166470157260,\"backgroundUrl\":\"http://p1.music.126.net/MXvCglkJK41YQU7c_YL2bA==/109951166470157259.jpg\",\"authority\":0,\"mutual\":false,\"expertTags\":null,\"experts\":null,\"djStatus\":0,\"vipType\":0,\"remarkName\":null,\"authenticationTypes\":0,\"avatarDetail\":null,\"anchor\":false,\"avatarImgIdStr\":\"109951166617939738\",\"backgroundImgIdStr\":\"109951166470157259\",\"avatarImgId_str\":\"109951166617939738\"}],\"subscribed\":null,\"commentThreadId\":\"A_PL_0_26467411\",\"newImported\":false,\"adType\":0,\"highQuality\":true,\"privacy\":0,\"ordered\":true,\"anonimous\":false,\"coverStatus\":3,\"recommendInfo\":null,\"socialPlaylistCover\":null,\"recommendText\":null,\"shareCount\":37037,\"alg\":\"alg_sq_offline\",\"commentCount\":6761}]}";
        
        let playlists = into(json_str.to_string()).unwrap();
        assert_eq!(playlists.len(), 2);
        assert_eq!(playlists[0].author, "名侦探-柯北");
        assert_eq!(playlists[0].id, 2075587022);
        
    }
}
