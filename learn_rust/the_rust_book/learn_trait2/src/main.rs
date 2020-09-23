// trait_bound语法
// 指定多个triat bound
// 返回 triat的类型
// trait GetInFormation {
//     fn get_name(&self) -> &str;
//     fn get_age(&self) -> u32;
// }

// fn print_information(item: impl GetInFormation) {
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }

// triat bound
// fn print_information<T: GetInFormation> (item: T) {
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }

trait GetName {
    fn get_name(&self) -> &str;
}

use std::fmt::Debug;

trait GetAge: Debug {
    fn get_age(&self) -> u32;
}

// 写法1
// fn print_information<T: GetAge + GetName> (item: T) {
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }

// 写法2
fn print_information<T>(item: T)
where
    T: GetName + GetAge,
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl Student {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

impl GetName for Student {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn produce_item_with_age() -> impl GetAge {
    // let flag = true;
    //if` and `else` have incompatible types
    // if flag {
    //     Student {
    //         name: String::from("xiaoming"),
    //         age: 30,
    //     }
    // }else {
    //     Teacher{
    //         name: String::from("dad"),
    //         age: 34,
    //         subject: String::from("xde"),
    //     }
    // }
    Student {
        name: String::from("xiaoming"),
        age: 30,
    }
}
#[derive(Debug)]
pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl Teacher {
    pub fn new(name: String, age: u32, subject: String) -> Self {
        Self { name, age, subject }
    }
}

impl GetName for Teacher {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let s = Student::new("xiaoming".to_string(), 23);
    print_information(s);

    let s = produce_item_with_age();
    println!("s = {:#?}", s);

    println!("Hello, world!");
}
