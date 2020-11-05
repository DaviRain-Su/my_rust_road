
// extern crate net_disk;

use log::debug;
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

use std::net::TcpStream;
use std::io::{self, prelude::*, BufReader, Error, Read, Write};

// use crate::net_disk::users; 

use crate::users;


/// 登录注册取消枚举体
#[derive(Debug, Deserialize, Serialize)]
pub enum LRC {
    LOGIN,
    REGISTRY,
    CANCEL,
}

impl LRC {
    pub fn deal_lrc(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error>{
        let mut buf = Vec::new();

        match *self {
            // 处理登录时消息
            LRC::LOGIN => {
                let bytes_read = stream.read_until(b'\n', &mut buf)?;
                if bytes_read == 0 {
                    return Ok(());
                }
                
                // 接受传输过来的User信息，和数据库中的用户信息进行对比
                let res_user: users::User  = serde_json::from_slice(&buf)?;
                // 根据用户名将获得salt, 将salt 发送给客户端
                // 再接受客户端密码和salt加密后的字符串
                // 将得到的加密后的字符串和数据中的加密后的字符串进行对比
                // 将结果返回给客户端
                
                // 验证登录成功之后进入业务的处理逻辑

                // 发送成功还是失败的消息
                
                Ok(())
            },
            // 处理注册时的消息
            LRC::REGISTRY => {
                let bytes_read = stream.read_until(b'\n', &mut buf)?;
                if bytes_read == 0 {
                    return Ok(());
                }

                let res_user: users::User  = serde_json::from_slice(&buf)?;
                // 查询数据库

                Ok(())
            },
            //处理取消时的消息
            LRC::CANCEL => {
                Ok(())
            }
        }
    }
}