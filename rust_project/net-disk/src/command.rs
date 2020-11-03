
#[derive(Debug)]
enum Commands {
    CD(Option<String>),
    LS(Option<String>),
    PUTS(Option<String>),
    GETS(Option<String>),
    REMOVE(Option<String>),
    PWD,
}

