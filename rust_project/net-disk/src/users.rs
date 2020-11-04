// use serde::{Serialize, Deserialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use super::utils::generate_string;

const SALT_LEN : usize = 10;

/// name 账户的名字
/// password 账户的密码
/// salt 随机值用于加密用的, salt长度默认设置的是10
/// cryptpassword 将密码和salt经过sh256加密之后的输出
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
    salt: String,
    cryptpassword: String,
}


impl User {
    pub fn new(name: String, password: String) -> Self {
        
        // 生成随机值
        let salt = generate_string(SALT_LEN);

        let password_salt = format!("{}{}", password, salt);

        //create a sha256  object
        let mut hasher = Sha256::new();

        //write input message 
        hasher.input_str(&password_salt);
        
        // read hash diggest
        let cryptpassword = hasher.result_str();
        
        Self {
            name,
            password,
            salt,
            cryptpassword,
        }
    }
}


#[test]
fn create_user_test() {
    let name = "dairain".to_string();
    let password = "123456".to_string();

    let user = User::new(name, password);

    println!("user = {:?}", user);
}