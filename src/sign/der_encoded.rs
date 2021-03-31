/*
 * @Author: your name
 * @Date: 2021-03-31 10:09:09
 * @LastEditTime: 2021-03-31 10:10:27
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \rsa_demo\src\sign\der_encoded.rs
 */

 pub fn der_encoded(file_content: &str) -> String{
    file_content
    .lines()
    .filter(|line| !line.starts_with("-"))
    .fold(String::new(), |mut data, line| {
        data.push_str(&line);
        data
    })
 }