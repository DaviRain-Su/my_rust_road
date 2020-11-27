use std::env;

fn main() {
    let mut argv = env::args();
    let arg : String = argv.nth(1).unwrap(); // error 1
    let n : i32 = arg.parse().unwrap(); // error 2
    println!("{}", 2 * n);
    main_find();

    let n = double_number("10");
    match n {
        Ok(n) => {
            assert_eq!(n, 20);
            println!("n = {}", n);
        }
        Err(err) => println!("Error: {:?}", err),
    }
}

// Searches 'haystack' for the Unicode character 'needle'. If one is found, the
// byte offset of the character is returned. Otherwise, 'None' is returned
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn main_find() {
    let file_name = "foobar.rs";
    // let file_name = "foobarrs";
    // match find(file_name, '.') {
    //     None => println!("No file extension found."),
    //     Some(i) => println!("File extension: {}", &file_name[i+1..])
    // }
    println!("File extension: {:?}", extension_explicit(file_name));
    println!("File extension: {:?}", extension(file_name));
    assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or("rs"), "rs");
    assert_eq!(extension("foobar.csv").unwrap_or_else(|| "rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or_else(|| "rs"), "rs");
}

// Return the extension of the given file name, where the extension is defined 
// as all characters succeeding the first '.', 
// If 'file_name' has no '.', the 'None' is returned. 
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..])
    }
}

// Return the extension of the given file name, where the extension is defined 
// as all characters succeeding the first '.', 
// If 'file_name' has no '.', the 'None' is returned. 
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
    match file_name(file_path) {
        None => None,
        Some(name) => match extension(name) {
            None => None,
            Some(ext) => Some(ext),
        }
    }
}

fn file_name(file_path: &str) -> Option<&str> {
    // implementation elided 
    unimplemented!()
}

fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}

// fn double_number(number_str: &str) -> i32 {
//     2 * number_str.parse::<i32>().unwrap()
// }

use std::num::ParseIntError;

// fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
//     match number_str.parse::<i32>() {
//         Ok(n) => Ok(2 * n),
//         Err(err) => Err(err),
//     }
// }

type Result<T> = std::result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}