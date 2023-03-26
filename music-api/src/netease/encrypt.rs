// unpotimized encrypt

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::string::String;
use rand::Rng; 

use openssl::symm;
use openssl::rsa::{self, Rsa, Padding};
use base64::engine::general_purpose;
use base64::Engine;


//SECTION take recommend sonlist as example 
struct post_payload {
    params: String,
    encSecKey: String,
}

lazy_static! {
    static ref CRYPTION_MAP: HashMap<&'static str, &'static str> = {
        let cryption_map = HashMap::new();
        cryption_map
    };

    static ref BASE62: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    static ref AES_IV: String = String::from("0102030405060708");
    static ref AES_PRESET_KEY: String = String::from("0CoJUm6Qyw8W8jud");
}

pub fn weapi_encrypt(
    serilized_message: &str,
    rsa_key: &str,
    modules: &str,

) -> String {
    let random_str = random_str(&16);

    let mut enc_text_encrypted = aes_cbc_encrypt(serilized_message.as_bytes(), AES_PRESET_KEY.as_bytes());

    enc_text_encrypted = general_purpose::STANDARD
        .encode(&enc_text_encrypted)
        .as_bytes()
        .to_vec();

    enc_text_encrypted = aes_cbc_encrypt(&enc_text_encrypted, random_str.as_bytes());

    enc_text_encrypted = general_purpose::STANDARD
        .encode(&enc_text_encrypted)
        .as_bytes()
        .to_vec();

    let enc_sec_key = rsa_encrypt(
        &random_str.as_bytes().iter().rev().copied().collect::<Vec<u8>>(), 
        rsa_key.as_bytes()); 
    String::from("value")
}

fn random_str(size: &usize) -> String {
    let mut rng = rand::thread_rng();
    let mut str_ret = String::from("");

    for _ in 1..*size+1 {
        let pos = rng.gen_range(0..BASE62.len());
        let char = BASE62.chars().nth(pos).unwrap();
        str_ret.push(char);
    }
    str_ret
}

fn aes_cbc_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    symm::encrypt(symm::Cipher::aes_128_cbc(), key, Some(AES_IV.as_bytes()), message).unwrap()
}

fn rsa_encrypt(message: &[u8], key: &[u8]) -> String {
    let rsa = Rsa::public_key_from_pem(key).unwrap();

    let prefix = vec![0u8; 128 - message.len()];

    let rsa_message = [&prefix[..], message].concat();

    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];

    rsa.public_encrypt(&rsa_message, &mut buf, Padding::NONE).unwrap();

    hex::encode(buf)
    
}

//~SECTION
#[cfg(test)]
mod encrypt_tests {
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
        println!("ase IV is: {:#?}", AES_IV.as_bytes());
        let encrypted = aes_cbc_encrypt(message.as_bytes(), random_str.as_bytes());

        let encrypted_default = symm::encrypt(
            symm::Cipher::aes_128_cbc(),
             random_str.as_bytes(), 
             Some(AES_IV.as_bytes()), 
             message.as_bytes()).unwrap();

        assert_eq!(encrypted, encrypted_default);

        let decrypted = symm::decrypt(
            symm::Cipher::aes_128_cbc(), 
            random_str.as_bytes(),
            Some(AES_IV.as_bytes()),
            &encrypted)
            .unwrap();

        let decrypted_str = String::from_utf8(decrypted).unwrap();

        println!("decryption is: {}", decrypted_str);
        assert_eq!(message, decrypted_str);
    }

    #[tests]
    fn reserve_str_test() {
        let text = String::from("I have a apple");
    }
}