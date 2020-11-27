fn main() {

    // println!("Hello, world!");

}

#[test]
fn format_test() {
    println!("{}", 1);
    println!("{:o}", 9);
    println!("{:x}", 255);
    println!("{:p}", &0);
    println!("{:b}", 15);
    println!("{:e}", 10000f32);
    println!("{:E}", 10000f32);
    println!("{:?}", "test");
    println!("{:#?}", ("test1", "test2"));
    println!("{a} {b} {b}", a = "x", b = "y");
}

