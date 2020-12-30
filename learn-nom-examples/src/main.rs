use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn main() {
    println!("color = {:?}", hex_color("#2F14DF").ok());
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223
            }
        ))
    );
}

#[test]
fn test_any() {
    let a = [1, 2, 3];

    println!("result = {}", a.iter().any(|&x| x > 0));
    println!("result = {}", a.iter().any(|&x| x > 5));

    let a = [1, 2, 3];
    let mut iter = a.iter();
    // a.iter().any(|&x| x > 0);

    println!("iter = {:?}", iter.any(|&x| x > 5));
    println!("ret = {:?}", iter);
    println!("ret = {:?}", iter.next());

    let a = [1, 2, 3];
    let mut iter = a.iter();

    println!("ret = {:?}", iter.any(|&x| x != 2));
    println!("iter = {:?}", iter);
    println!("iter.next  = {:?}", iter.next());
}
