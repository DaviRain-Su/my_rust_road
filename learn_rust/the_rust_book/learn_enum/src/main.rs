
// 1 类似于C语言的方式的定义
#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,

}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddKind,
    address: String,
}

// 2 rust 语言提倡的定义方式
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}
// 3 可以是不同的类型
#[derive(Debug)]
enum  IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 4 经典用法
enum Message {
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
}
// 等同于
// struct  QuitMessage; 
// struct MoveMessage {
// x: i32, 
// y: i32,
// }
// struct WriteMessage(String);
// struct ChangeMessage(i32, i32, i32);

impl  Message {
    fn print(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => {
                println!("Move x = {}, y = {}", x, y);
            },
            Message::Change(a, b, c) => {
                println!("Change a = {}, b = {}, c = {}", a, b, c);
            }
            Message::Write(ref s) => {
                println!("Write = {}", s);
            }
        }
    }
}

fn main() {

    let i1 = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("i1 = {:#?}", i1);

    let i2 = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };
    println!("i2 = {:#?}", i2);

    let i3 = IpAddr2::V4(String::from("127.0.0.1"));
    let i4 = IpAddr2::V6(String::from("::1"));
    println!("i3 = {:#?}, \n i4 = {:#?}", i3, i4);
   
    let i5 = IpAddr3::V4(127, 0, 0, 1);
    let i6 = IpAddr3::V6(String::from("::1"));
    println!("i5 = {:#?}", i5);
    println!("i6 = {:#?}", i6);

    let write = Message::Write(String::from("hello, world"));
    write.print();
    let move_temp = Message::Move{x : 1, y: 3};
    move_temp.print();
    let quit = Message::Quit;
    quit.print();
    let change = Message::Change(1, 2, 3);
    change.print();
  


    
    
    println!("Hello, world!");

}
