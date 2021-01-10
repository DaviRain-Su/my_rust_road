use std::io::{self, Read, stdout};
use termion::raw::IntoRawMode;


fn to_ctrl_byte(c: char) -> u8 {
    let bytes = c as u8;
    bytes & 0b0001_1111
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        // if c == 'q' {
        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
