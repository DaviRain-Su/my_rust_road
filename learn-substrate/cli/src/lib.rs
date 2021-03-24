//! substrate Cli library
//! substrate 命令行解析库

#![warn(missing_docs)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

/// Parse command line arguments and start the node
/// 解析命令行参数
pub fn main() {
    //使用yml文件的形式设置命令行参数
    let yaml = load_yaml!("./cli.yml");
    // 加载yml配置文件
    let matches = clap::App::from_yaml(yaml).get_matches();

    // TODO
    let log_pattern = matches.value_of("log").unwrap_or("");
    // 日志的初始化
    init_logger(log_pattern);

    // 匹配到collator
    if let Some(_) = matches.subcommand_matches("collator") {
        info!("Running collator.");
        return;
    }

    // 匹配到collator
    if let Some(_) = matches.subcommand_matches("validator") {
        info!("Running collator.");
        return;
    }

    // 打印帮助 如果没有任何的命令参数输入
    println!("No command given.\n");
    let _ = clap::App::from_yaml(yaml).print_long_help();
}


/// 日志初始化设置
fn init_logger(pattern: &str) {
    let mut builder = env_logger::Builder::new();
    // Disable info logging by default for some modules:
    builder.filter(Some("hyper"), log::LevelFilter::Warn);
    // Enable info for others.
    builder.filter(None, log::LevelFilter::Info);

    if let Ok(lvl) = std::env::var("RUST_LOG") {
        builder.parse_env(&lvl);
    }

    builder.parse_filters(pattern);

    builder.init();
}
