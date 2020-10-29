fn parse_int(s: String) -> u64 {
    s.parse::<u64>().expect("Could not parse as integer")
}

fn main() {
    // work fine
    let _ = parse_int("1".to_owned());
    // panics
    let _ = parse_int("abcd".to_owned());
}
