use std::io::Write;
use std::str::FromStr;

fn main() {
    
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
    // println!("gcd(14, 15) = {}", gcd(14, 15));
}


/// 关键字fn 定义了一个gcd （greatest common divisor, 最大公约数）的函数， 
/// 其接受两个参数n和m，类型均为u64, 即无符号64位整数。 
/// isize和usize分别表示指针大小的有符号和无符号整数，在32位平台是32位，在64位平台是64位。
/// 
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Rust只在函数体内推断变量类型，函数参数和返回值则必须明确写出类型。
    assert!(n != 0 && m != 0);
    while m != 0 { // Rust不需要用圆括号括住条件表达式，但需要花括号括住条件满足后要执行的语句
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // 如果函数体重最后一行代码是一个表达式，且表达式末尾没有分号，则这个表达式的值就是函数的返回值。
    // 实际上，用花括号括起来的任何代码都可以看做一个表达式
    // return语句一般只用于函数的中间提前返回。
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15),1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17 , 3 * 7 * 11 * 13 * 19), 3 * 11);
}