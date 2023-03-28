// unpotimized encrypt

use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use std::string::String;

use base64::engine::general_purpose;
use base64::Engine;
use openssl::bn::BigNum;
use openssl::rsa::{ Padding, Rsa};
use openssl::symm;

//SECTION take recommend sonlist as example
// struct post_payload {
//     params: String,
//     encSecKey: String,
// }

lazy_static! {
    static ref CRYPTION_MAP: HashMap<&'static str, &'static str> = {
        let cryption_map = HashMap::from([
            ("色", "00e0b"),
            ("流感", "509f6"),
            ("这边", "259df"),
            ("弱", "8642d"),
            ("嘴唇", "bc356"),
            ("亲", "62901"),
            ("开心", "477df"),
            ("呲牙", "22677"),
            ("憨笑", "ec152"),
            ("猫", "b5ff6"),
            ("皱眉", "8ace6"),
            ("幽灵", "15bb7"),
            ("蛋糕", "b7251"),
            ("发怒", "52b3a"),
            ("大哭", "b17a8"),
            ("兔子", "76aea"),
            ("星星", "8a5aa"),
            ("钟情", "76d2e"),
            ("牵手", "41762"),
            ("公鸡", "9ec4e"),
            ("爱意", "e341f"),
            ("禁止", "56135"),
            ("狗", "fccf6"),
            ("亲亲", "95280"),
            ("叉", "104e0"),
            ("礼物", "312ec"),
            ("晕", "bda92"),
            ("呆", "557c9"),
            ("生病", "38701"),
            ("钻石", "14af6"),
            ("拜", "c9d05"),
            ("怒", "c4f7f"),
            ("示爱", "0c368"),
            ("汗", "5b7a4"),
            ("小鸡", "6bee2"),
            ("痛苦", "55932"),
            ("撇嘴", "575cc"),
            ("惶恐", "e10b4"),
            ("口罩", "24d81"),
            ("吐舌", "3cfe4"),
            ("心碎", "875d3"),
            ("生气", "e8204"),
            ("可爱", "7b97d"),
            ("鬼脸", "def52"),
            ("跳舞", "741d5"),
            ("男孩", "46b8e"),
            ("奸笑", "289dc"),
            ("猪", "6935b"),
            ("圈", "3ece0"),
            ("便便", "462db"),
            ("外星", "0a22b"),
            ("圣诞", "8e7"),
            ("流泪", "01000"),
            ("强", "1"),
            ("爱心", "0CoJU"),
            ("女孩", "m6Qyw"),
            ("惊恐", "8W8ju"),
            ("大笑", "d"),
        ]);
        cryption_map
    };

    static ref BASE62: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    static ref AES_IV: String = String::from("0102030405060708");
    static ref AES_PRESET_KEY: String = String::from("0CoJUm6Qyw8W8jud");
    static ref RSA_PUBLIC_KEY: String = String::from("-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----");
    static ref RSA_PUBLIC_EXPONENT:String = String::from("010001");
    static ref RSA_MODULUS:String = String::from("00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7");
    static ref RSA_MODULUS_KEYS:Vec<&'static str> = vec!["色", "流感", "这边", "弱", "嘴唇", "亲", "开心", "呲牙", "憨笑", "猫", "皱眉", "幽灵", "蛋糕", "发怒", "大哭", "兔子", "星星", "钟情", "牵手", "公鸡", "爱意", "禁止", "狗", "亲亲", "叉", "礼物", "晕", "呆", "生病", "钻石", "拜", "怒", "示爱", "汗", "小鸡", "痛苦", "撇嘴", "惶恐", "口罩", "吐舌", "心碎", "生气", "可爱", "鬼脸", "跳舞", "男孩", "奸笑", "猪", "圈", "便便", "外星", "圣诞"];
    static ref RSA_PUBLIC_EXPONENT_KEYS: Vec<&'static str> = vec!["流泪", "强"];
    static ref AES_PRESET_KEY_KEYS: Vec<&'static str> = vec!["爱心", "女孩", "惊恐", "大笑"];
}

#[allow(unused)]
pub fn weapi_encrypt(
    serilized_message: &str,
    rsa_pub_exponent: &str,
    modulus: &str,
    aes_preset_key: &str,
) -> (String, String) {
    let random_str = random_str(&16);

    let mut enc_text_encrypted =
        aes_cbc_encrypt(serilized_message.as_bytes(), aes_preset_key.as_bytes());

    enc_text_encrypted = general_purpose::STANDARD
        .encode(&enc_text_encrypted)
        .as_bytes()
        .to_vec();

    enc_text_encrypted = aes_cbc_encrypt(&enc_text_encrypted, random_str.as_bytes());

    let enc_sec_key = rsa_encrypt(
        &random_str
            .as_bytes()
            .iter()
            .rev()
            .copied()
            .collect::<Vec<u8>>(),
        &rsa_pub_key_pem_gen(rsa_pub_exponent, modulus),
    );
    (general_purpose::STANDARD.encode(&enc_text_encrypted), enc_sec_key)
}

fn random_str(size: &usize) -> String {
    let mut rng = rand::thread_rng();
    let mut str_ret = String::from("");

    for _ in 1..*size + 1 {
        let pos = rng.gen_range(0..BASE62.len());
        let char = BASE62.chars().nth(pos).unwrap();
        str_ret.push(char);
    }
    str_ret
}

fn aes_cbc_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    symm::encrypt(
        symm::Cipher::aes_128_cbc(),
        key,
        Some(AES_IV.as_bytes()),
        message,
    )
    .unwrap()
}

fn rsa_encrypt(message: &[u8], pub_key: &[u8]) -> String {
    let rsa = Rsa::public_key_from_pem(pub_key).unwrap();

    let prefix = vec![0u8; 128 - message.len()];

    let rsa_message = [&prefix[..], message].concat();

    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];

    rsa.public_encrypt(&rsa_message, &mut buf, Padding::NONE)
        .unwrap();

    hex::encode(buf)
}

fn rsa_pub_key_pem_gen(rsa_pub_exponent: &str, modulus: &str) -> Vec<u8> {
    let rsa = Rsa::from_public_components(
        BigNum::from_hex_str(modulus).unwrap(),
        BigNum::from_hex_str(rsa_pub_exponent).unwrap(),
    )
    .unwrap();
    rsa.public_key_to_pem().unwrap()
}

#[allow(unused)]
fn encryption_map_collect(keys: &Vec<&str>) -> String {
    let mut result = String::from("");
        for str in keys.iter() {
            let val = CRYPTION_MAP.get(str).unwrap();
            result = result + val;
        }
    
    result
}

//~SECTION
#[cfg(test)]
mod encrypt_tests {
    use isahc::{HttpClient, ReadResponseExt, prelude::Configurable};
    use openssl::pkey::HasParams;

    use super::*;
    #[test]
    fn random_str_works() {
        let size = 19;
        let result = random_str(&size);
        assert_eq!(size, result.len());
        for char in result.chars() {
            assert!(BASE62.contains(char));
        }
    }

    #[test]
    fn aes_cbc_encrypt_works() {
        let size = 16;
        let random_str = random_str(&size);
        let message = "today fxxx";
        // println!("ase IV is: {:#?}", AES_IV.as_bytes());
        let encrypted = aes_cbc_encrypt(message.as_bytes(), random_str.as_bytes());

        let encrypted_default = symm::encrypt(
            symm::Cipher::aes_128_cbc(),
            random_str.as_bytes(),
            Some(AES_IV.as_bytes()),
            message.as_bytes(),
        )
        .unwrap();

        assert_eq!(encrypted, encrypted_default);

        let decrypted = symm::decrypt(
            symm::Cipher::aes_128_cbc(),
            random_str.as_bytes(),
            Some(AES_IV.as_bytes()),
            &encrypted,
        )
        .unwrap();

        let decrypted_str = String::from_utf8(decrypted).unwrap();

        // println!("decryption is: {}", decrypted_str);
        assert_eq!(message, decrypted_str);
    }

    #[test]
    fn reserve_str_test() {
        let text = String::from("I have a apple,你好");

        let text_bytes = text.as_bytes();
        let text_bytes_rev = text.as_bytes().iter().rev().copied().collect::<Vec<u8>>();

        assert_eq!(text_bytes.len(), text_bytes_rev.len());
        let mut i = 0;
        loop {
            if i == text_bytes.len() {
                break;
            }

            let j = text_bytes.len() - 1 - i;
            let byte = text_bytes[i];
            let byte_rev = text_bytes_rev[j];

            assert_eq!(byte, byte_rev);
            i = i + 1;
        }
    }

    #[test]
    fn rsa_pub_key_pem_gen_works() {
        let hex_exponent = "10001";
        let hex_modulus = "12345";
        let key = String::from_utf8(rsa_pub_key_pem_gen(hex_exponent, hex_modulus)).unwrap();

        assert!(key.contains("---BEGIN PUBLIC KEY---"));
        let rsa = Rsa::public_key_from_pem(key.as_bytes()).unwrap();
        assert!(rsa.n().eq(&BigNum::from_hex_str(hex_modulus).unwrap()));
    }

    #[test]
    fn weapi_encrypt_works() {
        let serilized_message = "{\"csrf_token\":\"c528c5a64b34793227f802f5eef69c9a\"}";
        let random_str = "7fhpTr1UE94ZSkJo";
        let aes_preset_key = &AES_PRESET_KEY;
        let rsa_pub_exponent = &RSA_PUBLIC_EXPONENT;
        let modulus = &RSA_MODULUS;
        let result_key = "db3563eb139dc280994673987f67d587af729917fe635af62ef3b63de565eed9bf3318418547f87e3a6965e97949bded028629c9dc9dc91e3bafba3c9ebc954526c6725be784f656045508724ff932aa8d1529455380e5e75b77f4ab61f973286406a5881e55857c91c1300129e326a5c7d1b2c9c19a411ad437e16d8a676ec1";
        let result_text = "tiRqIh6CgSFexcSDLg6SmLSWED6cCfPjS4tbmpWhmU8JDebZITG0KqcqVLCJGLpjJvqC6PUl///MSGFHYb9C9I40r+g6OQDsy2lglzsMDkXouKur4EjA7TpNmGevz/Av";

        let mut enc_text_encrypted =
            aes_cbc_encrypt(serilized_message.as_bytes(), aes_preset_key.as_bytes());

        enc_text_encrypted = general_purpose::STANDARD
            .encode(&enc_text_encrypted)
            .as_bytes()
            .to_vec();

        enc_text_encrypted = aes_cbc_encrypt(&enc_text_encrypted, random_str.as_bytes());

        let enc_text = general_purpose::STANDARD
            .encode(&enc_text_encrypted);

        assert_eq!(enc_text, result_text);

        let enc_sec_key = rsa_encrypt(
            &random_str
                .as_bytes()
                .iter()
                .rev()
                .copied()
                .collect::<Vec<u8>>(),
            &rsa_pub_key_pem_gen(&rsa_pub_exponent, &modulus),
        );

        // println!("{}", enc_sec_key);

        assert_eq!(enc_sec_key, result_key);
    }

    #[test]
    fn cryption_map_works() {
        let mut result = String::from("");
        for str in RSA_MODULUS_KEYS.iter() {
            let val = CRYPTION_MAP.get(str).unwrap();
            result = result + val;
        }
        assert_eq!(result, *RSA_MODULUS);

        result = String::from("");
        for str in RSA_PUBLIC_EXPONENT_KEYS.iter() {
            let val = CRYPTION_MAP.get(str).unwrap();
            result = result + val;
        }
        assert_eq!(result, *RSA_PUBLIC_EXPONENT);

        result = encryption_map_collect(&AES_PRESET_KEY_KEYS);
        assert_eq!(result, *AES_PRESET_KEY);
    }

    #[test]
    fn window_dot_asrsea_simulate_works() {
        let message = "122131";
        let (_, encrypt_aes_key) = weapi_encrypt(
            message,
            &encryption_map_collect(&vec!["流泪", "强"]),
            &encryption_map_collect(&RSA_MODULUS_KEYS),
            &encryption_map_collect(&vec!["爱心", "女孩", "惊恐", "大笑"])
        );

        assert_eq!(encrypt_aes_key.len(), 256);
    }

    fn weapi_enc(message: &str) -> (String, String) {
        weapi_encrypt(
            message,
            &encryption_map_collect(&vec!["流泪", "强"]),
            &encryption_map_collect(&RSA_MODULUS_KEYS),
            &encryption_map_collect(&vec!["爱心", "女孩", "惊恐", "大笑"])
        )
    }

    #[test]
    fn test_recommend_resource_api() {
        use urlqstring::QueryParams;

        // ANCHOR - test song list 
        
        let params = QueryParams::from_vec(vec![
            ("cat", "全部"),
            ("order", "hot"),
            ("offset", "0"),
            ("limit", "50"),
        ]);

        let csrf = "c528c5a64b34793227f802f5eef69c9a";

        let json = params.add_query_string("csrf_token", "").json();

        let (params, enc_sec_key) = weapi_enc(&json);

        // let cookie = "_ntes_nnid=7e922546bbf2ec6589d41114d333da5f,1678520593206; _ntes_nuid=7e922546bbf2ec6589d41114d333da5f; NMTID=00O2g5huToSbpF5G0Q8kfd_D5r2O-cAAAGGz5-rfw; WEVNSM=1.0.0; WNMCID=dzfbkg.1678520593377.01.0; WM_TID=ewx4g1lxRipAAURQFUbUebD6kq6NoIxf; __snaker__id=vLybqdwhQpbTnaA8; YD00000558929251:WM_NI=3tzZ6BbJMYNgDtRBXaZFMFP7qMhwYsxSpAW+BXp5CSPyrGnjqs63uSS+zINTEoavgBCPDeYGDj3TJSe/pBpoO9THnCX4f5Sfbx0XnZT0yvHAF93SrDie7wAyAI8X6E88QXk=; YD00000558929251:WM_NIKE=9ca17ae2e6ffcda170e2e6eeb8b5458befb9d7f259f4e78fb2d85a978b8e87c545acbde1aef974bcee9aaef42af0fea7c3b92af1acb6bbb54aa29683bacd6d88ac9ea3db6b9a9a83a7d33db393a898dc79bc8dc0a2d6218288c0d6c25994a7bea8ed62b3adfad4f85991f5b6aacd6087b1af97e64a81908c86dc4d88b6fdb5c959a7f1c0a3f3658de79ca9ed499cecaad6d26ea2a800b9ed5298f0b797cb4b86e7fdb4f150abbbf998c4738397f8b9b54ffce89fb9ea37e2a3; YD00000558929251:WM_TID=CfzIInFg5wBEFRVRQUbQKwJ+Jj0wnFM/; __csrf=c528c5a64b34793227f802f5eef69c9a; MUSIC_U=15c5ca7d070c1e292675366f29b0da358f4c861a5670d631753172e34fed4a17993166e004087dd3c90eb8006f9a69241845719034b42d0359941a9e0a8e405921bc8e2ef03d8d9dd4dbf082a8813684; ntes_kaola_ad=1; gdxidpyhxdE=zxyP6A1XMzZNu1WX4MThKppgqjZBvivgGDMZpCER48I6vUhLW+nusyuv\\JdmO6s2agOcsb4M+TD1cL2EMd+Uij6rPBrVWdlLhQjg\\sUJdixn5xdVw9gQ0Pk003uVcxIzm\\pBVZ/igk3Wbrv3c2qCRKDtnB7MQx7o0uvfTKh/qCsyEWNg:1679726323241; playerid=36574408; WM_NI=LSA+PEOx2qOXnGu9DyCQY90cUv3dtNfzbuWA5pHV95l82NNstyFsvmJYz9ozwxnYBzI+PJ57hyYf0GisMl3ypxYqNbTFuTfwSUZXV1+2YvOZZbibAuGkRliNCgm5xrADd0k=; WM_NIKE=9ca17ae2e6ffcda170e2e6ee85d54aa389f995f96eaeb08bb2d45f838f8fb1c46ebcb48793e425b1a69fb4c72af0fea7c3b92a908d8fccd34f8da8fe92ce54ae99bbdac773818f97dab36997aafd85f746a6a6ae96d64bf7b088b7bc3fb0e8feafcd79b8a78a94f5678a87b6daee72b5a8a2bac1728bf5a0abb83daca79787b34dabad838db273b4a89792cd6a989abbb5e57d918b96b9d53be9ee9797ce529abea783c47f85b59e8dec4681b9a3d6fb7a839cacd1ea37e2a3; _iuqxldmzr_=33; JSESSIONID-WYYY=fUn/1e5fUoglp5PMwMx0nY\nhZ9k/vEo\\poJZ83tAD5dzd17Vt\\XKc3qwp1afYAh\\GRTxNGdNtah2xCIVr95D15jqRw5RCTZlCaopf/YsbvoN\\zK7uO0tmmAyMnlcg\\BSD5trhQIeX6BtCCPwz7dtzHx+081x2vZDOIYI1ufyVaihQ7b:1679929953756";

        let cookie_simp = "_ntes_nnid=7e922546bbf2ec6589d41114d333da5f,1678520593206; _ntes_nuid=7e922546bbf2ec6589d41114d333da5f; NMTID=00O2g5huToSbpF5G0Q8kfd_D5r2O-cAAAGGz5-rfw; WEVNSM=1.0.0; WNMCID=dzfbkg.1678520593377.01.0; WM_TID=ewx4g1lxRipAAURQFUbUebD6kq6NoIxf; __snaker__id=vLybqdwhQpbTnaA8; YD00000558929251:WM_NI=3tzZ6BbJMYNgDtRBXaZFMFP7qMhwYsxSpAW+BXp5CSPyrGnjqs63uSS+zINTEoavgBCPDeYGDj3TJSe/pBpoO9THnCX4f5Sfbx0XnZT0yvHAF93SrDie7wAyAI8X6E88QXk=; YD00000558929251:WM_NIKE=9ca17ae2e6ffcda170e2e6eeb8b5458befb9d7f259f4e78fb2d85a978b8e87c545acbde1aef974bcee9aaef42af0fea7c3b92af1acb6bbb54aa29683bacd6d88ac9ea3db6b9a9a83a7d33db393a898dc79bc8dc0a2d6218288c0d6c25994a7bea8ed62b3adfad4f85991f5b6aacd6087b1af97e64a81908c86dc4d88b6fdb5c959a7f1c0a3f3658de79ca9ed499cecaad6d26ea2a800b9ed5298f0b797cb4b86e7fdb4f150abbbf998c4738397f8b9b54ffce89fb9ea37e2a3; YD00000558929251:WM_TID=CfzIInFg5wBEFRVRQUbQKwJ+Jj0wnFM/; __csrf=c528c5a64b34793227f802f5eef69c9a; MUSIC_U=15c5ca7d070c1e292675366f29b0da358f4c861a5670d631753172e34fed4a17993166e004087dd3c90eb8006f9a69241845719034b42d0359941a9e0a8e405921bc8e2ef03d8d9dd4dbf082a8813684; ntes_kaola_ad=1; gdxidpyhxdE=zxyP6A1XMzZNu1WX4MThKppgqjZBvivgGDMZpCER48I6vUhLW+nusyuv\\JdmO6s2agOcsb4M+TD1cL2EMd+Uij6rPBrVWdlLhQjg\\sUJdixn5xdVw9gQ0Pk003uVcxIzm\\pBVZ/igk3Wbrv3c2qCRKDtnB7MQx7o0uvfTKh/qCsyEWNg:1679726323241; playerid=36574408; WM_NI=LSA+PEOx2qOXnGu9DyCQY90cUv3dtNfzbuWA5pHV95l82NNstyFsvmJYz9ozwxnYBzI+PJ57hyYf0GisMl3ypxYqNbTFuTfwSUZXV1+2YvOZZbibAuGkRliNCgm5xrADd0k=; WM_NIKE=9ca17ae2e6ffcda170e2e6ee85d54aa389f995f96eaeb08bb2d45f838f8fb1c46ebcb48793e425b1a69fb4c72af0fea7c3b92a908d8fccd34f8da8fe92ce54ae99bbdac773818f97dab36997aafd85f746a6a6ae96d64bf7b088b7bc3fb0e8feafcd79b8a78a94f5678a87b6daee72b5a8a2bac1728bf5a0abb83daca79787b34dabad838db273b4a89792cd6a989abbb5e57d918b96b9d53be9ee9797ce529abea783c47f85b59e8dec4681b9a3d6fb7a839cacd1ea37e2a3; _iuqxldmzr_=33; JSESSIONID-WYYY=fUn/1e5fUoglp5PMwMx0nY\\nhZ9k/vEo\\poJZ83tAD5dzd17Vt\\XKc3qwp1afYAh\\GRTxNGdNtah2xCIVr95D15jqRw5RCTZlCaopf/YsbvoN\\zK7uO0tmmAyMnlcg\\BSD5trhQIeX6BtCCPwz7dtzHx+081x2vZDOIYI1ufyVaihQ7b:1679929953756";

        let body = QueryParams::from_vec(vec![
            ("params", &params),
            ("encSecKey", &enc_sec_key)
        ]).stringify();

        let user_agent = "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 6 Build/LYZ28E) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36";

        let url = format!("https://music.163.com/weapi/playlist/list?csrf_token={}", csrf);
        let request = isahc::Request::post(&url)
                    // .header("Cookie", "os=pc; appver=2.7.1.198277")
                    // .header("Accept", "*/*")
                    // .header("Accept-Language", "en-US,en;q=0.5")
                    // .header("Connection", "keep-alive")
                    // .header("Content-Type", "application/x-www-form-urlencoded")
                    // .header("Host", "music.163.com")
                    // .header("Referer", "https://music.163.com")
                    // .header("User-Agent", user_agent)
                    // .header("Cookie", cookie_simp)
                    .body(body)
                    .unwrap();

        let client = isahc::HttpClient::builder().cookies().build().unwrap();
        
        let mut reponse = client.send(request).unwrap();

        let text = reponse.text().unwrap();

        let body = reponse.body();

        println!("reponse is {:#?}, body is {:#?}, val is {}", reponse, body, text);

    }
}
