fn main() {
    let a = 3;
    let b = 4;
    let m = 26;

    let message = "this is a simple example of affine cipher";
    println!("message = {}", message);
    let encrp_str = encryption_operation(a, b, m, message.into());
    println!("encrp_str = {:}", encrp_str);

    let dec = decrypt(a, b, m, encrp_str);
    println!("dec = {:?}", dec);
}

fn encryption_operation(a: i32, b: i32, m: i32, message: String) -> String {
    let message = message
        .trim()
        .replace(" ", "")
        .chars()
        .map(|val| val.to_lowercase().to_string())
        .collect::<String>();
    //println!("after message: {:?}", message);

    let message = message
        .chars()
        .map(|val| (((val as u8 as i32 - 97) * a + b) % m) as u8)
        .collect::<Vec<u8>>();
    //println!("message = {:?}", message);

    let message = message
        .into_iter()
        .map(|val| (val + 65) as char)
        .collect::<String>();
    //println!("message = {}", message);
    message
}

fn decrypt(a: i32, b: i32, m: i32, message: String) -> String {
    let mut i = 2;
    let a_1 = loop {
        if (i * a) % m == 1 {
            break i;
        }
        i += 1;
    };

    //    println!("a_1 = {}", a_1);

    let message = message
        .chars()
        .map(|val| {
            let mut c = (a_1 * (val as i32 - 65 - b)) % m;
            if c < 0 {
                c += m;
            }
            (c + 65) as u8 as char
        })
        .collect::<String>();
    //println!("message = {:?}", message);
    message
}
