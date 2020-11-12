use log::debug;
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use std::io::BufReader;
use std::net::TcpStream;
use std::io::Error;


#[derive(Debug, Deserialize, Serialize)]
pub enum Commands {
    CD(Option<Vec<String>>),
    LS(Option<Vec<String>>),
    PUTS(Option<Vec<String>>),
    GETS(Option<Vec<String>>),
    REMOVE(Option<Vec<String>>),
    PWD(Option<String>),
    OTHER(String),
}

impl Commands {
    pub fn new(commands: &str) -> Self {
        let commands = commands
            .trim()
            .chars()
            .map(|val| val.to_lowercase().to_string())
            .collect::<String>();
        let commands = commands.split(' ').collect::<Vec<&str>>();
        debug!("commands = {:?}", commands);

        let commands_len = commands.len();

        let command = commands[0];
        let commands = commands
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>();

        if command == "cd" && (commands_len == 1 || commands_len == 2) {
            
            Commands::CD(Some(commands))
        } else if command == "ls" && (commands_len == 1 || commands_len == 2) {
            Commands::LS(Some(commands))
        } else if command == "puts" && commands_len == 2 {
            Commands::PUTS(Some(commands))
        } else if command == "gets" && commands_len == 2 {
            Commands::GETS(Some(commands))
        } else if command == "remove" && commands_len == 2 {
            Commands::REMOVE(Some(commands))
        } else if command == "pwd" && commands_len == 1 {
            Commands::PWD(Some(commands[0].to_string()))
        } else {
            Commands::OTHER(format!("No this command : {:?}", commands))
        }
    }

    pub fn deal_command(&self, stream: &mut BufReader<&TcpStream>) -> Result<(), Error> {
        match *self { 
            Commands::CD(ref cd) => {
                self.deal_cd(stream, cd)?;
            },
            Commands::LS(ref ls) => {
                self.deal_ls(stream, ls)?;
            },
            Commands::PWD(ref pwd) => {
                self.deal_pwd(stream, pwd)?;
            },
            Commands::REMOVE(ref rm) => {
                self.deal_remove(stream, rm)?;
            },
            Commands::GETS(ref gets) => {
                self.deal_gets(stream, gets)?;
            },
            Commands::PUTS(ref puts) => {
                self.deal_puts(stream, puts)?;
            },
            Commands::OTHER(ref others) => {
                self.deal_other(stream, others)?;
            }
        }
        Ok(())
    }

    fn deal_cd(&self, _stream: &BufReader<&TcpStream>, _args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!()
    }
    fn deal_ls(&self, _stream: &BufReader<&TcpStream>, _args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!()
    }
    fn deal_pwd(&self, _stream : &BufReader<&TcpStream>, _args: &Option<String>) -> Result<(), Error> {
        unimplemented!()
    }
    fn deal_remove(&self, _stream: &BufReader<&TcpStream>, _args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!()
    }
    fn deal_gets(&self, _stream: &BufReader<&TcpStream>, _args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!()
    }
    fn deal_puts(&self, _stream: &BufReader<&TcpStream>, _args: &Option<Vec<String>>) -> Result<(), Error> {
        unimplemented!()
    }
    fn deal_other(&self, _stream: &BufReader<&TcpStream>, _args: &String) -> Result<(), Error> {
        unimplemented!()
    }

}


#[derive(Debug, Serialize, Deserialize)]
pub enum CommandsReturnCode {
    CDNORMAL(String),
    LSNORMAL(String),
    PUTSNORMAL(String),
    GETSNORMAL(String),
    RMNORMAL(String),
    PWDNORMAL(String),
    ERROR(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InitReturnCode {
    NORMAL,
    ERROR,
}

impl InitReturnCode {
    pub fn parse(&self) {
        match *self {
            InitReturnCode::NORMAL => {
                println!("用户创建成功");
            }
            InitReturnCode::ERROR => {
                println!("用户创建失败");
            }
        }
    }
}
