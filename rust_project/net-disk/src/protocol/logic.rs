// extern crate net_disk;

use log::debug;
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

use std::io::{self, prelude::*, BufReader, Error, Read, Write};
use std::net::TcpStream;

// use crate::net_disk::users;

use super::command::Commands;
use crate::users::User;

/// 登录注册取消枚举体
#[derive(Debug, Deserialize, Serialize)]
pub enum LRC {
    LOGIN,
    REGISTRY,
    CANCEL,
}

impl LRC {
    pub fn deal_lrc(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
        let mut buf = Vec::new();

        match *self {
            // 处理登录时消息
            LRC::LOGIN => {
                let bytes_read = stream.read_until(b'\n', &mut buf)?;
                if bytes_read == 0 {
                    return Ok(());
                }

                // 接受传输过来的User信息，和数据库中的用户信息进行对比
                let res_user: User = serde_json::from_slice(&buf)?;
                // 根据用户名将获得salt, 将salt 发送给客户端
                // 再接受客户端密码和salt加密后的字符串
                // 将得到的加密后的字符串和数据中的加密后的字符串进行对比
                // 将结果返回给客户端

                // 验证登录成功之后进入业务的处理逻辑

                // 发送成功还是失败的消息

                Ok(())
            }
            // 处理注册时的消息
            LRC::REGISTRY => {
                let bytes_read = stream.read_until(b'\n', &mut buf)?;
                if bytes_read == 0 {
                    return Ok(());
                }

                let res_user: User = serde_json::from_slice(&buf)?;
                // 插入数据库中数据
                // 返回插入成功的结果

                Ok(())
            }
            //处理取消时的消息
            LRC::CANCEL => Ok(()),
        }
    }

    pub fn deal_cmd(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
        let mut buf = Vec::new();
        let bytes_read = stream.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let cmd: Commands = serde_json::from_slice(&buf)?;
        match cmd {
            Commands::CD(ref cd) => self.deal_cd(),
            Commands::LS(ref ls) => self.deal_ls(),
            Commands::RM(ref rm) => self.deal_rm(),
            Commands::PWD(ref pwd) => self.deal_pwd(),
            Commands::OTHERS(ref others) => self.deal_others(),
            Commands::GETS(ref gets) => self.deal_gets(),
            Commands::PUTS(ref puts) => self.deal_puts(),
        }

    }

    pub fn deal_cd(&self) -> Result<(), Error> {
        unimplemented!();
    }
    pub fn deal_ls(&self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn deal_pwd(&self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn deal_gets(&self) -> Result<(), Error> {
        unimplemented!();
    }
    pub fn deal_puts(&self) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn deal_rm(&self) -> Result<(), Error> {
        unimplemented!();
    }
    pub fn deal_others(&self) -> Result<(), Error> {
        unimplemented!();
    }
}
