// use mylib::factory::product_refigerator;
// use mylib::factory::product_washing_machine as A; // 使用了as
use mylib::factory::*;
fn main() {
    
    // mylib::factory::product_refigerator::produce_refigerator();// 绝对路径
    product_refigerator::produce_refigerator(); // 相对路径 使用use
    // A::produce_washing_machine();
    product_washing_machine::produce_washing_machine();

    mylib::produce_refigerator();
    mylib::produce_washing_machine();
    println!("Hello, world!");
}
