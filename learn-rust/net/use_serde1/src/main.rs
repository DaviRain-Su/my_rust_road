/// serder crate 是Serder生态的核心
/// serde_derive crate 提供必要的工具
/// 使用过程宏来派生 Serialize 和 Deserialize
/// 但是serde只提供序列化和反序列化的框架，具体操作还需要依赖具体的包
/// 如serde_json和serde_yaml等

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ServerConf {
    worker : u64,
    ignore: bool,
    auth_server: Option<String>,
}

impl ServerConf {
    pub fn new(worker: u64, ignore: bool, server: String) -> Self {
        Self {
            worker,
            ignore,
            auth_server: Some(server),
        }
    }
}

fn main() {
    let config  = ServerConf::new(100, true, String::from("123"));

    {
        println!("json!");
        let serializer = serde_json::to_string(&config).unwrap();
        println!("serialized : {:#?}", serializer);

        let deserialized : ServerConf = serde_json::from_str(&serializer).unwrap();
        println!("derserialized : {:#?}", deserialized);
    }
    println!("-----------------------------------");
    {
        println!("yaml!");
        let serializer = serde_yaml::to_string(&config).unwrap();
        println!("serialized : {:#?}", serializer);

        let deserialized : ServerConf = serde_yaml::from_str(&serializer).unwrap();
        println!("derserialized : {:#?}", deserialized);
    }
    println!("Hello, world!");
}
