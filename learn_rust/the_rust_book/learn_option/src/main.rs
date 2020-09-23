// 1 Option 是标准库中定义的一个枚举， 形式：
// enum Option<T> {
//     Some(T),
//     None,
// }
// 2 使用方式
fn main() {
    let _some_number = Some(5);
    let _some_string = Some(String::from("a string"));
    let _absent_number: Option<i32> = None;
    
    let x : i32 = 5;
    // let y : Option<i32> = Some(5);
    let y : Option<i32> = None;
    let mut temp = 0;
    match y {
        Some(i) =>  { 
            temp = i;
        },
        None => {
            println!("Do nothing");
        }
    }
    let sum = x + temp;
    println!("sum = {}", sum);
    let result = plus_one(y);
    println!("result = {:?}", result);

    if let Some(x) = plus_one(y) {
        println!("x  = {}", x);
    } else {
        println!("do nothing");
    }
    println!("Hello, world!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
