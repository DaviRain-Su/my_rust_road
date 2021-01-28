// 使用trait bound 有条件的实现方法
trait GetName {
    fn get_name(&self) -> &str;
}
trait GetAge {
    fn get_age(&self) -> u32;
}
#[derive(Debug)]
struct PeopleMatchInfomation<T, U> {
    master: T,
    student: U,
}

impl<T, U> PeopleMatchInfomation<T, U>
where
    T: GetAge + GetName,
    U: GetName + GetAge,
{
    fn print_all_information(&self) {
        println!("master name = {}", self.master.get_name());
        println!("master age = {}", self.master.get_age());
        println!("student name = {}", self.student.get_name());
        println!("student age = {}", self.student.get_age());
    }
}
#[derive(Debug)]
struct Teacher {
    name: String, 
    age: u32,
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
#[derive(Debug)]
struct Student {
    name: String, 
    age: u32,
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

fn main() {
    let s = Student{ name: String::from("xiaoming") , age: 23};
    let t = Teacher{ name: "xiaohuang".to_string(), age: 23};
    let m = PeopleMatchInfomation{master: t, student: s};
    println!("m = {:#?}", m);
    m.print_all_information();
    println!("Hello, world!");
}
