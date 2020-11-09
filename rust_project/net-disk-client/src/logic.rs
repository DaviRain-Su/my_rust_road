use std::io::{self, Error};
use std::io::BufReader;
use std::net::TcpStream;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde::Serialize;
use serde::Deserialize;
use std::io::prelude::*;

use log::{debug, info};

use crate::command::{InitReturnCode, CommandsReturnCode};
use crate::users::User;

/// 登录注册取消枚举体
#[derive(Debug, Deserialize, Serialize)]
pub enum LRC {
    LOGIN,
    REGISTRY,
    QUIT,
    OTHER,
}

impl LRC {
    pub fn new(input_cmd : &str) -> Self {
        //截取输入的命令中有换行符，空格，将所有的字符串变为小写的
        let input_cmd = input_cmd.trim().to_lowercase();
        if input_cmd == "login" {
            LRC::LOGIN
        }else  if input_cmd == "registry" {
            LRC::REGISTRY
        }else if input_cmd == "quit" {
            LRC::QUIT
        }else {
            // 如果输入的不是上面的任何一个命令报错
            LRC::OTHER
        }
    }

    pub fn deal(&self, stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
        match *self  {
            LRC::LOGIN => {
                // unimplemented!()
                login(stream)?;
                Ok(())
            },
            LRC::REGISTRY => {
                // unimplemented!();
                registry(stream)?;
                Ok(())
            },
            LRC::QUIT => {
                // unimplemented!();
                quit(stream)?;
                Ok(())
            },
            LRC::OTHER => {
                // unimplemented!();?
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
    // 输入用户名
    get_intput(&mut username);
    debug!("username = {:?}", username);
    // 输入密码
    info!("请输入密码: ");
    get_intput(&mut password);
    debug!("password = {:?}",  password);

    // 用户创建
    let new_user = User::new(username, password);
    debug!("new user = {:?}", new_user);

    // 发送用户信息
    send_message(stream, &new_user);

    //接受返回值
    let ret_result = recv_message(stream);
    debug!("ret_result = {:?}", ret_result);
    let ret : InitReturnCode = serde_json::from_slice(&ret_result)?;
    debug!("ret = {:?}", ret);

    // 解析返回的返回值
    ret.parse();
    Ok(())
}


pub fn registry(stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {

    Ok(())
}

pub fn quit(stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {

    Ok(())
}

pub fn other(stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {

    Ok(())
}


pub fn get_intput(buf: &mut String) {
    io::stdin().read_line(buf).expect("Failed read from stdin");
}

pub fn send_message<T: Serialize>(stream: &mut BufReader<&TcpStream>,  val : &T) {
    stream
    .get_mut()
    .write_all(serde_json::to_string(val).unwrap().as_bytes())
    .expect("Failed to write to server");
    stream
    .get_mut()
    .write_all(b"\n")
    .expect("Failed to write to server");
}

pub fn recv_message(stream: &mut BufReader<&TcpStream>) -> Vec<u8> {
    let mut buf : Vec<u8> = Vec::new();
    let bytes_read = stream.read_until(b'\n', &mut buf).unwrap();
    if bytes_read == 0 {
        debug!("recv successful!");
    }
    buf
}