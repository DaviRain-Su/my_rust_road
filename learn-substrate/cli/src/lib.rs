#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

pub fn main() {
    // let yaml = load_yaml!("./cli.yml");
    // let matches = clap::App::from_yaml(yaml).get_matches();
    let yaml = load_yaml!("./cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let log_pattern = matches.value_of("log").unwrap_or("");
    init_logger(log_pattern);

    if let Some(_) = matches.subcommand_matches("collator") {
        info!("Running collator.");
        return;
    }

    if let Some(_) = matches.subcommand_matches("validator") {
        info!("Running collator.");
        return;
    }

    println!("No command given.\n");
    let _ = clap::App::from_yaml(yaml).print_long_help();
}

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

    // builder.init().expect("Logger initialized only once.");
    builder.init();
}
