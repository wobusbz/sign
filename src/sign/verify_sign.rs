/*
 * @Author: your name
 * @Date: 2021-03-31 10:05:15
 * @LastEditTime: 2021-03-31 10:24:03
 * @LastEditors: your name
 * @Description: In User Settings Edi
 * @FilePath: \rsa_demo\src\sign\verify_sign.rs
 */

use super::{der_encoded::der_encoded, hash};
use rsa::{Hash, PaddingScheme, PublicKey, RSAPublicKey};

pub fn verify_sign(file_content: &str, sign: &str, source : String) ->bool{
    let der_encoded =  der_encoded(file_content);
    let der_bytes = base64::decode(&der_encoded).expect("decode  key failed");
    let public_key = RSAPublicKey::from_pkcs8(&der_bytes).unwrap();
    public_key.verify(
    PaddingScheme::PKCS1v15Sign {
        hash: Option::from(Hash::SHA2_256),
    },
    &hash::hasher(&source),
    &base64::decode(&sign).unwrap(),
    ).is_ok()
}