use std::time::Duration;

use isahc::prelude::Configurable;

//SECTION const

const TIME_OUT_SEC: u64 = 100;

//~SECTION

//ANCHOR Netease app
// resolve netease request and data deserlize
//#[derive(Default)]
pub struct NeteaseApp {
    client: isahc::HttpClient,
}



impl NeteaseApp {
    
    // ANCHOR new()
    // <1> try to get cookie from cache dir
    // <2> if get cookie, create http client with cookie, goto<3>, else goto<6>
    // <3> try to get login status with the cookie
    // <4> if the cookie is valid, save new cookie to cache dir goto<7>
    // <5> if the cookie expired, clean cookie file, goto<6>
    // <6> request with guest mode
    // <7> request with nomal mode
    // 
    // TODO implement nomal mode first
    //pub fn new() -> Self {
	//NeteaseApp::default()
    //}



}
