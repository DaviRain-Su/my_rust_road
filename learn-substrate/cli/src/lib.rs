use clap::{Arg, App, SubCommand};

pub fn main() {
	let matches = App::new("Polkadot")
		.arg(Arg::with_name("log")
			.short("l")
			.value_name("LOG")
			.help("Sets logging.")
			.takes_value(true))
		.subcommand(SubCommand::with_name("collator"))
		.subcommand(SubCommand::with_name("validator"))
		.get_matches();

	let log_pattern = matches.value_of("log").unwrap_or("");
	init_logger(log_pattern);

	if let Some(_) = matches.subcommand_matches("collator") {
		println!("Running collator.");
		return;
	}

	if let Some(_) = matches.subcommand_matches("validator") {
		println!("Running collator.");
		return;
	}
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
