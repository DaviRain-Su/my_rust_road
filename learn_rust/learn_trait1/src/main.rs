// 1. trait 用于定义于其他类型共享的功能，类似于其他语言中的接口
// 1.1 可以通过trait以抽象的方式定义共享的行为
// 1.2 可以通过trait bounds指定泛型是任何拥有特定行为的类型
// 2 定义triat
pub trait GetInFormation {
    fn get_name(&self) -> &str{
        "hello, world"
    }
    fn get_age(&self) -> u32;
}

trait School {
    fn get_school_name(&self) -> &str {
        "🏫❄️😁"
    }
}
// 3 实现triat
pub struct Student {
    pub name: String,
    pub age: u32,
}
impl Student {
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
        }
    }
}
impl GetInFormation for Student {
  
    fn get_name(&self) -> &str {
        &self.name
    }   
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl School for Student {}

pub struct Teacher {
    pub name: String,
    pub age: u32, 
    pub subject: String,
}
impl Teacher {
    pub fn new(name: String, age: u32, subject: String) -> Self {
        Self {
            name,
            age,
            subject,
        }
    }
}

impl  GetInFormation for Teacher {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}
impl School for Teacher {
    fn get_school_name(&self) -> &str {
        "😀"
    }
}
// 4 默认实现: 可以在定义triat的时候提供默认的行为，trait的类型可以使用默认的行为。
// 5 triat作为参数

fn print_information(item: impl GetInFormation) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    let s = Student::new("xiaoming".to_string(), 10);
    let t = Teacher::new("nini".to_string(), 32, "math".to_string());
    println!("Hello, world!");
   
    

    println!("s.name = {}, s.age = {}", s.get_name(), s.get_age());
    println!("t.name = {}, t.age = {}", t.get_name(), t.get_age());

    // print_information(s);
    // print_information(t);

   println!("schole = {}", s.get_school_name());
   println!("schole = {}", t.get_school_name());
}
