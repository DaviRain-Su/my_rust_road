pub mod pattern {

use std::io::Write;

pub fn find_matches(content: &str, pattern: &str, mut writer : impl Write) -> Result<(), std::io::Error>{
    for (_line_no, line) in content.lines().enumerate() {
        if line.contains(pattern){
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let stdout = std::io::stdout();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", stdout);
}

#[test]
fn find_a_match_use_string () {
    let mut result = Vec::new();
    // let mut result = String::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result,  b"lorem ipsum\n");
    let result = String::from_utf8(result);
    println!("{}", result.unwrap());
}

}