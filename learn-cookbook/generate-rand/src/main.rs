// #[macro_use]
use rand::Rng;

/// 生成随机数
fn generate_rand_number() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}",rng.gen::<f64>());
}

/// 生成范围的随机数
fn generate_range_rand_number () {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}

/// 均匀分布值
fn generate_distributions_number() {
    use rand::distributions::{Distribution, Uniform};

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn main() {
    // generate_range_rand_number();
    generate_distributions_number();
}
