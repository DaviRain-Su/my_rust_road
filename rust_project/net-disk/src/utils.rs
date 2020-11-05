use rand;
use rand::Rng;

/// 生成随机数长度为参数的长度length
pub fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut rand_string = String::new();

    (0..length).for_each(|_| {
        let flag = rng.gen::<u8>() % 3;

        match flag {
            0 => {
                rand_string.push((rng.gen::<u8>() % 26 + ('a' as u8)) as char);
            }
            1 => {
                rand_string.push((rng.gen::<u8>() % 26 + ('A' as u8)) as char);
            }
            2 => {
                rand_string.push((rng.gen::<u8>() % 26 + ('0' as u8)) as char);
            }
            _ => panic!("faild "),
        }
    });

    rand_string
}

#[test]
fn test_generat_string() {
    let length = 10;

    let result = generate_string(length);
    println!("result = {}", result);
}
