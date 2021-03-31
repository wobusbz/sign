/*
 * @Author: your name
 * @Date: 2021-03-31 10:17:20
 * @LastEditTime: 2021-03-31 10:20:25
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \rsa_demo\src\sign\hash.rs
 */
 use crypto::{sha2::Sha256, digest::Digest};
 use std::iter::repeat;

 pub fn hasher(source : &str) ->Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.input_str(source);
    let mut buf: Vec<u8> = repeat(0).take((hasher.output_bits()+7)/8).collect();
    hasher.result(&mut buf);
    buf
 }