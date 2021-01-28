fn main() {
    println!("Hello, world!");
    let (a, b) = (2, 3);
    other_func1(a, b);
    println!("result = {}", other_func2(a, b));
    other_fun();

    // 语句是执行一些操作，但是不返回值的指令
    // let y = 1; //语句不返回值
    // let x = (let y = 1);
    //
    // 表达式会计算一些值
    let y = {
        let x = 1;
        x + 1
    };

    println!("y = {}", y);
}

fn other_fun() {
    println!("This is a func");
}

fn other_func1(a: i32, b: i32) {
    println!("a = {}, b = {}", a, b);
}

fn other_func2(a: i32, b: i32) -> i32 {
    a + b
}
