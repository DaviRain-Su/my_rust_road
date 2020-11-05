use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct CliConfig {
    config: Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    ip: Option<String>,
    port: Option<String>,
    thread_num: Option<String>,
}

impl Config {
    pub fn new(config_path: &str) -> Self {
        let path = Path::new(config_path);

        let file = File::open(path).expect("Could open config file!");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Could read file to String!!");

        // println!("contents = {}", contents);
        let cfg: CliConfig = toml::from_str(&contents).expect("Could parse config toml");

        Self {
            ip: cfg.config.ip,
            port: cfg.config.port,
            thread_num: cfg.config.thread_num,
        }
    }

    pub fn get_thread_num(&self) -> usize {
        self.thread_num.as_ref().unwrap().parse::<usize>().unwrap()
    }
    pub fn get_ip(&self) -> &str {
        self.ip.as_ref().unwrap()
    }
    pub fn get_port(&self) -> &str {
        self.port.as_ref().unwrap()
    }
    pub fn get_ip_port(&self) -> String {
        format!("{}:{}", self.get_ip(), self.get_port())
    }
}
