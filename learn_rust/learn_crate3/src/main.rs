mod mod_a {
    #[derive(Debug)]
    pub struct A {
        pub number : i32,
        name: String,
    }
    impl A  {
        pub fn new() -> Self {
            Self {
                number: 0,
                name: String::from("hello"),
            }
        }
        pub fn print(&self) {
            println!("number : {}, name : {}", self.number, self.name);
        }
    }
    
    pub mod mod_b {
        pub fn print() {
            println!("mod B!!");
        }

        pub mod mod_c {
            pub fn print() {
                println!("mod C!!");
                super::print();
            }
        }
    }
}

use mod_a::A;
fn main() {
    // let a = mod_a::A::new();
    let a = A::new();
    println!("a = {:?}", a);
    a.print();
    let number = a.number;
    println!("number = {}", number);
    // let name = a.name; // field `name` of struct `mod_a::A` is private


    mod_a::mod_b::mod_c::print();
    mod_a::mod_b::print();

    println!("Hello, world!");
}
