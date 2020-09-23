// 对任何实现了特定trait的类型有条件的实现trait

trait GetName{
    fn get_name(&self) -> &str;
}

trait PrintName {
    fn print_name(&self);
}

// T 是实现特定trait的类型, 再为T实现PrintName
impl <T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("name = {}", self.get_name());
    }
}

struct Student {
    name: String,
}

impl GetName for Student {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {

    let s = Student{ name: "dede".to_string()};
    s.print_name();
    println!("Hello, world!");
}
