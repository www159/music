use rand_core::RngCore;


///ANCHOR - Netease Cloud Music encryption methods
pub enum EncryptionMethod {
    // uri with "*/weapi/*"
    WEAPI,
}


//SECTION WEAPI

static BASE62: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

static RSA_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----";

static AES_PRESET_KEY: &str = "0CoJUm6Qyw8W8jud";

static AES_IV: &str = "0102030405060708";


/// weapi encryption
/// - param data: stringified json
/// - return (enc_data, enc_sec_key)
pub fn weapi_encryption(data: &str) -> (String, String) {
    // `data` should be json string
    #[cfg(feature = "music-dev")]
    let _ = serde_json::from_str(data).unwrap();
    
    use openssl::symm;
    use openssl::rsa::{ Rsa, Padding };
    use rand_core::OsRng;
    use base64::engine::general_purpose;
    use base64::Engine;

    // gen random bytes with len 16
    let mut random_key_arr = [0u8; 16];
    OsRng.fill_bytes(&mut random_key_arr);
    let random_key = random_key_arr
        .iter()
        .map(|i| BASE62.as_bytes()[(i % 62) as usize])
        .collect::<Vec<u8>>(); 

    // aes encryption with key `AES_PRESET_KEY`
    let enc_text_byte = symm::encrypt(
        symm::Cipher::aes_128_cbc(),
        AES_PRESET_KEY.as_bytes(),
        Some(AES_IV.as_bytes()),
        data.as_bytes()).unwrap();

    // base64 encode
    let enc_text = general_purpose::STANDARD
        .encode(enc_text_byte);

    // aes encryption with key `random_key`
    let enc_text_byte = symm::encrypt(
        symm::Cipher::aes_128_cbc(),
        &random_key,
        Some(AES_IV.as_bytes()),
        &enc_text.as_bytes()).unwrap();

    // RSA encryption
    // encrypt `random_str`
    // if `random_str` is "abcd"
    // the openssl rsa `from` buf is `0{124}dcba`
    let enc_sec_key = {
        let rsa = Rsa::public_key_from_pem(RSA_PUBLIC_KEY_PEM.as_bytes()).unwrap();

        let prefix = vec![0u8; 128 - random_key.len()];

        let random_key_128 = [&prefix[..], &random_key.iter().rev().copied().collect::<Vec<u8>>()].concat();

        let mut to_buf = vec![0; rsa.size() as usize];

        let size = rsa.public_encrypt(&random_key_128, &mut to_buf, Padding::NONE).unwrap();

        // encode the `rsa-encoded-bytes` to `hex string` at last
        hex::encode(to_buf)       
    };
    
    let enc_text = general_purpose::STANDARD
        .encode(enc_text_byte);

    (enc_text, enc_sec_key)
}
//~SECTION