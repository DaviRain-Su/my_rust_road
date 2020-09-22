fn main() {
    // bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);
    let is_false = false;
    println!("is_false = {}", is_false);

    println!("is_true = {}", is_true);

    // char 在rust中，char是32位的
    let a = 'a';
    println!("a = {}", a);

    let b = '你';
    println!("b = {}", b);

    // 数字类型， i8, i16, i32, i64, i128,
    // u8, u16, u32, u64, u128,
    // f32, f64
    let c = -111;
    println!("c = {}", c);

    // 自适应类型 和平台有关系
    //
    println!("usize = {}", 1usize);
    println!("isize = {}", -1isize);

    println!("max = {}", usize::max_value());

    // 数组
    // [Type: size] size 也是数组类型的一部分
    //
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
    show(&arr);

    // 元组

    let tup: (i32, f32, char) = (-3, 3.4, 'a');
    println!("tup = {:?}", tup);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn show(arr: &[u32]) {
    println!("--------------");
    for i in arr {
        println!("{}", i);
    }
}
