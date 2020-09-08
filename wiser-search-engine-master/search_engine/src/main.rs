extern crate xml;
use regex::Regex;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let path = "/Users/suyinrong/Downloads/coolshell.xml";
    // let path = "/Users/suyinrong/Downloads/zhwiki-20200901-pages-articles-multistream1.xml"; 
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            // Ok(XmlEvent::StartElement { name, .. }) => {
            //     println!("{}+{}", indent(depth), name);
            //     depth += 1;
            // },
            Ok(XmlEvent::CData(ref temp_string)) => {
                let re = Regex::new(r"/[0-9]+/$").unwrap();
                let after = re.replace_all(temp_string, "");
                println!("temp = {}", after);
            },
            // Ok(XmlEvent::EndElement { name }) => {
            //     depth -= 1;
            //     println!("{}-{}", indent(depth), name);
            // },
            
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
