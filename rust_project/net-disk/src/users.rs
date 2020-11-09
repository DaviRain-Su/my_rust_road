use serde_derive::Deserialize;
use serde_derive::Serialize;

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::utils::generate_string;

const SALT_LEN: usize = 10;

/// name 账户的名字
///
/// password 账户的密码
///
/// salt 随机值用于加密用的, salt长度默认设置的是10
///
/// cryptpassword 将密码和salt经过sh256加密之后的输出
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryUser {
    username: String,
    password: String,
    salt: String,
    cryptpassword: String,
}

impl RegistryUser {
    pub fn new(username: &str, password: &str) -> Self {
        let username = username.trim();
        let password = password.trim();

        // 生成随机值
        let salt = generate_string(SALT_LEN);

        // 将随机值拼接在password上
        let password_salt = format!("{}{}", password, salt);

        //create a sha256  object
        let mut hasher = Sha256::new();

        //write input message
        hasher.input_str(&password_salt);

        // read hash digest
        let cryptpassword = hasher.result_str();

        Self {
            username: username.into(),
            password: password.into(),
            salt,
            cryptpassword,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_salt(&self) -> &str {
        &self.salt
    }
    pub fn get_cryptpassword(&self) -> &str {
        &self.cryptpassword
    }
}

#[test]
fn create_user_test() {
    let name = "davi rain".to_string();
    let password = "123456".to_string();

    let user = RegistryUser::new(&name, &password);

    println!("user = {:?}", user);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

impl LoginUser {
    pub fn new(username: &str, password: &str) -> Self {
        // 删除用户名和密码前后的换行符和空格
        let username = username.trim();
        let password = password.trim();
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
}
