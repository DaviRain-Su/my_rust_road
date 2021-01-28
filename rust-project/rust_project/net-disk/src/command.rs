use log::debug;
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub enum Commands {
    CD(Option<Vec<String>>),
    LS(Option<Vec<String>>),
    PUTS(Option<Vec<String>>),
    GETS(Option<Vec<String>>),
    RM(Option<Vec<String>>),
    PWD(Option<String>),
    OTHERS(String),
}

impl Commands {
    pub fn new(commands: &str) -> Self {
        let commands = commands
            .trim()
            .chars()
            .map(|val| val.to_lowercase().to_string())
            .collect::<String>();
        debug!("commands = {:?}", commands);

        let commands = commands.split(' ').collect::<Vec<&str>>();
        debug!("commands = {:?}", commands);

        let commands_len = commands.len();
        debug!("command len = {}", commands_len);

        let command = commands[0];
        debug!("command = {}", command);

        let commands = commands
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>();
        debug!("commands = {:?}", commands);

        if command == "cd" && (commands_len == 1 || commands_len == 2) {
            // 提前处理一些逻辑错误
            Commands::CD(Some(commands))
        } else if command == "ls" && (commands_len == 2 || commands_len == 1) {
            // 提前处理一些逻辑错误
            Commands::LS(Some(commands))
        } else if command == "puts" && commands_len == 2 {
            // 提前处理一些逻辑错误
            Commands::PUTS(Some(commands))
        } else if command == "gets" && commands_len == 2 {
            // 提前处理一些逻辑错误
            Commands::GETS(Some(commands))
        } else if command == "remove" && commands_len == 2 {
            // 提前处理一些逻辑错误
            Commands::RM(Some(commands))
        } else if command == "pwd" && commands_len == 1 {
            // 提前处理一些逻辑错误
            Commands::PWD(Some(commands[0].to_string()))
        } else {
            // 提前处理一些逻辑错误
            Commands::OTHERS(format!("No this command : {:?}", commands))
        }
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
