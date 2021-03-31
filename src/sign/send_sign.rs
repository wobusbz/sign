/*
 * @Author: your name
 * @Date: 2021-03-31 10:28:19
 * @LastEditTime: 2021-03-31 10:40:46
 * @LastEditors: your name
 * @Description: In User Settings Edit
 * @FilePath: \rsa_demo\src\sign\send_sign.rs
 */
use super::{der_encoded::der_encoded, hash::hasher};
use rsa::{RSAPrivateKey, PaddingScheme, Hash};

pub fn send_sign(file_content : &str, source: &str) ->String{
    let der_bytes = base64::decode(&der_encoded(file_content)).expect("private decode failed");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).unwrap();
    let result = private_key.sign(PaddingScheme::PKCS1v15Sign{hash:Option::from(Hash::SHA2_256)},  &hasher(source)).unwrap();
    base64::encode(&result)
}