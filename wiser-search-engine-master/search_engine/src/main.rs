extern crate xml;

use std::fs::File;
use std::io::BufReader;
use regex::Regex;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let path = "/Users/suyinrong/Downloads/coolshell.xml";
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    let mut results = String::new();
    let re = Regex::new(r"<.*?>").unwrap();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                // println!("{}", name);
                depth += 1;
            },
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            },
            Ok(XmlEvent::CData(ref temp_string)) => {
                let result = re.replace_all(temp_string, "");
                println!("{}", result);
                // results.push_str(&result);
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    println!("{}", results);
}