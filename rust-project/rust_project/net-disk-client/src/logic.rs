use serde::Deserialize;
use serde::Serialize;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Error};
use std::net::TcpStream;

use log::{debug, info};

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::command::{CommandsReturnCode, InitReturnCode, Commands};
use crate::users::{RegistryUser, LoginUser};

/// 登录注册取消枚举体
#[derive(Debug, Deserialize, Serialize)]
pub enum LRC {
    LOGIN,
    REGISTRY,
    QUIT,
    OTHER,
}

impl LRC {
    pub fn new(input_cmd: &str) -> Self {
        //截取输入的命令中有换行符，空格，将所有的字符串变为小写的
        let input_cmd = input_cmd.trim().to_lowercase();
        if input_cmd == "login" {
            LRC::LOGIN
        } else if input_cmd == "registry" {
            LRC::REGISTRY
        } else if input_cmd == "quit" {
            LRC::QUIT
        } else {
            // 如果输入的不是上面的任何一个命令报错
            LRC::OTHER
        }
    }

    pub fn deal(&self, stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
        match *self {
            LRC::LOGIN => {
                login(stream)?;
                Ok(())
            }
            LRC::REGISTRY => {
                registry(stream)?;
                Ok(())
            }
            LRC::QUIT => {
                quit(stream)?;
                Ok(())
            }
            LRC::OTHER => {
                other(stream)?;
                Ok(())
            }
        }
    }
}

pub fn login(stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
    let mut username = String::new();
    let mut password = String::new();

    info!("请输入用户名: ");
    get_intput(&mut username);
    debug!("username = {:?}", username);

    info!("请输入密码: ");
    get_intput(&mut password);
    debug!("password = {:?}", password);

    let new_user = LoginUser::new(&username, &password);
    debug!("new user = {:?}", new_user);

    send_message(stream, &new_user)?;

    let mut salt = Vec::new();
    recv_message(stream, &mut salt)?;
    debug!("salt = {:?}", salt);
    
    // 需要使用serde_json 来解析发送的字符串
    let salt : String = serde_json::from_slice(&salt)?;
    debug!("salt = {:?}", salt);

    let password = password.trim();

    let password_salt = format!("{}{}", password, salt);

    // 制作加密密码
    // create a sha256  object
    let mut hasher = Sha256::new();

    // write input message
    hasher.input_str(&password_salt);

    // read hash diggest
    let cryptpassword = hasher.result_str();
    debug!("cryptpassword = {}", cryptpassword);
    send_message(stream, &cryptpassword)?;

    let mut ret_result = Vec::new();
    recv_message(stream, &mut ret_result)?;
    debug!("ret_result = {:?}", ret_result);
    
    let ret: InitReturnCode = serde_json::from_slice(&ret_result)?;
    debug!("ret = {:?}", ret);

    // 解析返回的返回值
    // ret.parse();
    match ret {
        InitReturnCode::NORMAL => {
            // 处理业务
            deal_cmd(stream)?;

        },
        InitReturnCode::ERROR => {
            deal_error(stream)?;
        },
    }
    Ok(())
}

pub fn deal_cmd(stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
    let mut input = String::new();
    get_intput(&mut input);
    let command = Commands::new(&input);
    command.deal_command(stream)?;
    Ok(())
}

pub fn deal_error(_stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
    unimplemented!()
}

pub fn registry(stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
    let mut username = String::new();
    let mut password = String::new();

    info!("请输入用户名: ");

    // 输入用户名
    get_intput(&mut username);
    debug!("username = {:?}", username);

    // 输入密码
    info!("请输入密码: ");
    get_intput(&mut password);
    debug!("password = {:?}", password);

    // 用户创建
    let new_user = RegistryUser::new(&username, &password);
    debug!("new user = {:?}", new_user);

    // 发送用户信息
    send_message(stream, &new_user)?;

    // 接受返回值
    let mut ret_result = Vec::new();
    recv_message(stream, &mut ret_result)?;
    debug!("ret_result = {:?}", ret_result);
    
    let ret: InitReturnCode = serde_json::from_slice(&ret_result)?;
    debug!("ret = {:?}", ret);

    // 解析返回的返回值
    ret.parse();
    Ok(())
}

pub fn quit(_stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
    Ok(())
}

pub fn other(_stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
    Ok(())
}

pub fn get_intput(buf: &mut String) {
    io::stdin().read_line(buf).expect("Failed read from stdin");
}

pub fn send_message<T: Serialize>(stream: &mut BufReader<&TcpStream>, val: &T) -> Result<(), Error> {
    stream
        .get_mut()
        .write_all(serde_json::to_string(val).unwrap().as_bytes())?;

    stream.get_mut().write_all(b"\n")?;

    Ok(())
}

pub fn recv_message(stream: &mut BufReader<&TcpStream>, buf : &mut Vec<u8>) -> Result<(), Error> {
    let bytes_read = stream.read_until(b'\n', buf)?;
    if bytes_read == 0 {
        return Ok(());
    }
    Ok(())
}
