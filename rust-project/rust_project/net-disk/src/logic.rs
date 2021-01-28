use log::debug;
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

use std::io::{self, prelude::*, BufReader, Error, Read, Write};
use std::net::TcpStream;

use super::command::{Commands, CommandsReturnCode, InitReturnCode};

use crate::db::estable_connection;
use crate::db::models::{UserInfo, UserPath, UserRequest};
use crate::users::{LoginUser, RegistryUser};
use crate::db::schema::user_info::columns::username;
use rand::prelude::StdRng;

/// 登录注册取消枚举体
#[derive(Debug, Deserialize, Serialize)]
pub enum LRC {
    LOGIN,
    REGISTRY,
    QUIT,
    OTHER,
}

impl LRC {

    /// 处理是登录 注册 取消消息
    pub fn deal_lrc(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
        match *self {
            // 处理登录时消息
            LRC::LOGIN => {
                self.login(stream)?;
                Ok(())
            }
            // 处理注册时的消息
            LRC::REGISTRY => {
                self.registry(stream)?;
                Ok(())
            }
            //处理取消时的消息
            LRC::QUIT => {
                self.quit(stream)?;
                Ok(())
            }
            LRC::OTHER => unimplemented!(),
        }
    }


    pub fn deal_cmd(&self, stream: &mut BufReader<TcpStream>, username: &str) -> Result<(), Error> {
        let mut buf = Vec::new();
        recv_message(stream, &mut buf)?;
        debug!("deal cmd, buf = {:?}", buf);

        // 解析传过来的命令
        let cmd: Commands = serde_json::from_slice(&buf)?;
        debug!("deal cmd, cmd = {:?}", cmd);

        match cmd {
            Commands::CD(ref cd) => self.deal_cd(stream, cd , username),
            Commands::LS(ref ls) => self.deal_ls(ls),
            Commands::RM(ref rm) => self.deal_rm(rm),
            Commands::PWD(ref pwd) => self.deal_pwd(pwd),
            Commands::GETS(ref gets) => self.deal_gets(gets),
            Commands::PUTS(ref puts) => self.deal_puts(puts),
            Commands::OTHERS(ref others) => self.deal_others(others),
        }
    }
    ///
    /// cd ..
    /// cd ~
    /// cd -
    /// cd /path/
    /// other error
    pub fn deal_cd(&self, stream: &mut BufReader<TcpStream>, args: &Option<Vec<String>>, username: &str) -> Result<(), Error> {
        let mut old_path_name = String::new();
        old_path_name.push_str(username);
        debug!("old_path_name = {:?}", old_path_name);

        let mut root_path = String::new();
        root_path.push_str(&format!("{}:~$", username));
        debug!("root_path = {:?}", root_path);

        let mut buf = Vec::new();
        recv_message(stream, &mut buf)?;

        let old_path: String = serde_json::from_slice(&buf)?;
        debug!("old_path = {:?}", old_path);

        buf.clear();
        recv_message(stream, &mut buf)?;
        let mut cur_path : String = serde_json::from_slice(&buf)?;
        debug!("cur_path = {:?}", cur_path);

        let args = args.unwrap();

        if args[1] == " " && cur_path == root_path
            || args[1] == "." && cur_path == root_path
            || args[1] == ".." && cur_path == root_path
        {
            cur_path = root_path.clone();
            send_message(stream, &cur_path)?;
            return Ok(());
        }

        // cd space
        if args[1] == " " && cur_path == old_path {
            cur_path = root_path.clone();
            send_message(stream, &cur_path)?;
            return Ok(());
        }

        // cd .
        if args[1] == "." && cur_path == old_path {
            cur_path = old_path.clone();
            send_message(stream, &cur_path)?;
            return Ok(());
        }

        // cd ..
        if args[1] == ".." && cur_path == old_path {
            let mut tmp_path = cur_path.clone();
            let position = tmp_path.rfind("/").unwrap();
            let mut tmp_path = &tmp_path[..position].to_string();
            tmp_path.push('$');
            cur_path = tmp_path.clone();

            send_message(stream, &cur_path)?;
            return Ok(());
        }

        // cd path
        if args[1] != " " && cur_path == old_path {
            unimplemented!()
        }


        unimplemented!();
    }

    ///
    /// ls .
    /// ls ..
    /// ls ~
    /// ls /path/
    /// other error
    pub fn deal_ls(&self, stream: &mut BufReader<TcpStream>,  cur_dir_num: u64) -> Result<(), Error> {
        let mut file_list : Vec<String> = Vec::new();

        // 查询数据中cur_dir_num下的所有文件
        db_file_query_user_path(cur_dir_num, &mut file_list);
        send_message(stream, &file_list)?;
        return Ok(());
    }

    /// pwd
    pub fn deal_pwd(&self, args: &Option<String>) -> Result<(), Error> {
        unimplemented!();
    }

    /// gets file
    /// gets file1 file2 file3 扩展
    ///
    pub fn deal_gets(&self, args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!();
    }

    /// puts file
    /// puts file1 file2 file3
    ///
    pub fn deal_puts(&self, args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!();
    }

    /// rm file
    /// rm -rf dir
    ///
    pub fn deal_rm(&self, args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!();
    }

    /// other error
    pub fn deal_others(&self, args: &String) -> Result<(), Error> {
        unimplemented!();
    }

    /// process login
    pub fn login(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
        let mut buf = Vec::new();
        debug!("In login buf = {:?}", buf);

        // 接受消息
        recv_message(stream, &mut buf)?;
        debug!("In login bug = {:?}", buf);

        // 接受传输过来的User信息，和数据库中的用户信息进行对比
        let res_user: LoginUser = serde_json::from_slice(&buf[..])?;
        debug!("In login res_user = {:?}", res_user);

        // 连接建立
        // TODO 如果用户传入过来一个没有注册的用户需要错误处理
        let conn = estable_connection();
        let ret_result = UserInfo::by_username(res_user.get_name(), &conn).unwrap();

        // 得到salt
        let salt = ret_result.salt;
        let username = ret_result.username;
        debug!("In login salt = {:?}", salt);
        debug!("In login crypto password = {:?}", ret_result.cryptpassword);

        // 发送salt
        send_message(stream, &salt)?;

        // 接收加密后的密码
        buf.clear();
        debug!("In login buf = {:?}", buf);

        recv_message(stream, &mut buf)?;

        // 需要使用serde_json 来解析发送的字符串
        let crypto_salt_password : String = serde_json::from_slice(&buf)?;
        debug!("In login crypto password = {:?}", crypto_salt_password);

        // 进行密码验证
        if crypto_salt_password == ret_result.cryptpassword {
            // 发送成功消息
            let normal = InitReturnCode::NORMAL;
            debug!("In login normal = {:?}", normal);
            send_message(stream, &normal)?;
            debug!("In login, will in deal_cmd");
            // 进入业务逻辑处理
            self.deal_cmd(stream, &username)?;
        } else {
            let error = InitReturnCode::ERROR;
            debug!("In login error = {:?}", error);
            send_message(stream, &error)?;
            // 发送失败消息
        }

        // 根据用户名将获得salt, 将salt 发送给客户端
        // 再接受客户端密码和salt加密后的字符串
        // 将得到的加密后的字符串和数据中的加密后的字符串进行对比
        // 将结果返回给客户端
        // 验证登录成功之后进入业务的处理逻辑
        // 发送成功还是失败的消息
        Ok(())
    }

    /// process registry
    pub fn registry(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
        let mut buf = Vec::new();
        debug!("In Registry, buf = {:?}", buf);
        debug!("Start at registry!");

        recv_message(stream, &mut buf)?;
        debug!("In Registry, buf = {:?}", buf);

        let res_user: RegistryUser = serde_json::from_slice(&buf)?;
        debug!("In Registry res_user = {:?}", res_user);

        // 数据库连接建立
        let conn = estable_connection();

        // 应该先从数据中查找有没有这个用户有没有，有的话不执行创建用户
        if UserInfo::by_username(res_user.get_name(), &conn).is_none() {
            // 数据库中没有这个用户名，执行插入新的用户
            // 向数据库中插入数据(新用户的插入)
            let result = UserInfo::create(
                res_user.get_name(),
                res_user.get_password(),
                res_user.get_salt(),
                res_user.get_cryptpassword(),
                &conn,
            );

            // 得到插入用户后信息
            debug!("In Registry result = {:?}", result);

            // 返回发送成功的返回值
            let ret_code = InitReturnCode::NORMAL;
            debug!("ret_code = {:?}", ret_code);
            send_message(stream, &ret_code)?;
        } else {
            let result = UserInfo::by_username(res_user.get_name(), &conn).unwrap();
            debug!("In Registry result = {:?}", result);

            // 返回创建失败的消息，因为用户已经存在了
            let ret_code = InitReturnCode::ERROR;
            debug!("ret_code = {:?}", ret_code);
            send_message(stream, &ret_code)?;
        }
        Ok(())
    }

    /// process quit
    pub fn quit(&self, stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
        let normal = InitReturnCode::NORMAL;
        send_message(stream, &normal)?;
        Ok(())
    }
}

pub fn send_message<T: Serialize>(stream: &mut BufReader<TcpStream>, val: &T) -> Result<(), Error> {
    stream
        .get_mut()
        .write_all(serde_json::to_string(val).unwrap().as_bytes())?;

    stream.get_mut().write_all(b"\n")?;
    Ok(())
}

pub fn recv_message(stream: &mut BufReader<TcpStream>, buf: &mut Vec<u8>) -> Result<(), Error> {
    let bytes_read = stream.read_until(b'\n', buf)?;
    if bytes_read == 0 {
        return Ok(());
    }
    Ok(())
}
