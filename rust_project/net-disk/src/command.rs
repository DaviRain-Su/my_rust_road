
#[derive(Debug)]
enum Commands {
    CD(Option<String>),
    LS(Option<String>),
    PUTS(Option<String>),
    GETS(Option<String>),
    REMOVE(Option<String>),
    PWD,
}

impl Commands {
    pub fn new(commands: &str) -> Self {
        let commands  = commands.trim().chars().map(|val| {
            val.to_lowercase().to_string()
        }).collect::<String>();
        let commands = commands.split(' ').collect::<Vec<&str>>();
        debug!("commands = {:?}", commands);

        let commands_len = commands.len();

        let command = commands[0];
        let commands = commands.iter().map(|val| val.to_string()).collect::<Vec<String>>();

        if command == "cd" && (commands_len == 1 || commands_len == 2) {
            Commands::CD(Some(commands))
        }else if command == "ls" && (commands_len == 2 || commands_len == 1) {
            Commands::LS(Some(commands))
        }else if command == "puts" && commands_len == 2 {
            Commands::PUTS(Some(commands))
        }else if command == "gets" && commands_len == 2 {
            Commands::GETS(Some(commands))
        }else if command == "remove"  && commands_len == 2{
            Commands::REMOVE(Some(commands))
        }else if command == "pwd" && commands_len == 1 {
            Commands::PWD(Some(commands[0].to_string()))
        }else {
            Commands::OTHER(format!("No this command : {:?}", commands)) 
        }
    }
}