// 方法中的声明周期
struct StuA<'a> {
    pub name: &'a str,
}

impl<'a> StuA<'a> {
    fn do_something(&self) -> i32 {
        3
    }

    fn do_something2(&'a self, s: &str) -> &'a str {
        self.name
    }

    fn do_something3<'b>(&self, s: &'b str) -> &'b str {
        s
    }
}
fn main() {
    let s = String::from("hello");
    let a = StuA { name: &s };
    println!("{}", a.do_something());

    let ss = a.do_something2(&s);
    println!("ss = {}", ss);

    let ss = a.do_something3(&s);
    println!("ss = {}", ss);

    println!("Hello, world!");
}
