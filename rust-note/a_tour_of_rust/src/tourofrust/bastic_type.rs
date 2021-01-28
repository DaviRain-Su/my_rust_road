/// # 基本类型
///
/// - 布尔类型,bool表示true或false
/// - 无符号整型,u8, u32, u64, u128表示正整数
/// - 无符号整型, i8, i32, i64, i128表示正负整数
/// - 指针大小的整数, usize isize表示内存中内容的索引和大小
/// - 浮点数, f32, f64
/// - 元组(tuple), (value, value, ...) 用于在栈上传递固定序列的值
/// - 数组, 在编译时已知的具体固定长度的相同元素的集合
/// - 切片(slice), 在运行时已知长度的相同元素的集合
/// - str(string slice), 在运行时已知长度的文本
pub fn bastic_type_example() {
    let a: bool = true;
    let x1: u8 = 23;
    let x2: u32 = 34;
    let x3: u64 = 456;
    let x4: u128 = 8888;
    let y1: i8 = -1;
    let y2: i64 = -23;
    let y3: i128 = 232;
    // let u_ptr: usize;
    // let i_ptr: isize;
    let f1: f32 = 323.4;
    let f2: f64 = 2323.2323;
    let tuple = (1, 2, 3, 4, 5, "hello");
    let vec = [1, 2, 3, 4, 5, 6, 7];
    let str_example = "hello world!";
    println!(
        "
            a = {:?},
            x1 = {:?}, 
            x2 = {:?},
            x3 = {:?}, 
            x4 = {:?}, 
            y1 = {:?}, 
            y2 = {:?}, 
            y3 = {:?}, 
            f1 = {:?}, 
            f2 = {:?}, 
            tuple = {:?},
            tuple.0 = {:?}, 
            vec = {:?}, 
            str_emaple = {:?}",
        a, x1, x2, x3, x4, y1, y2, y3, f1, f2, tuple, tuple.0, vec, str_example
    );
}

/// # 基本类型转换
///
/// 使用as关键字进行基本类型转换
pub fn basic_type_transform() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("a u8 as u32 + b u32 = {}", c);

    let t = true;
    println!("true as u8 = {}", t as u8);
    let f = false;
    println!("false as u8 = {}", f as u8);
}

/// # 常量
///
/// 常量允许我们高效地指定一个在代码中会被多次使用的公共值。
/// 不同于像变量一样在使用的时候会被复制值，常量会在编译期间直接用他们的值来替换文本标识符，
/// 不同于变量，常量必须始终具有显式的类型
///
/// 常量总是遵循全大写蛇形命名法 SCREAMING_SNALE_CASE
pub fn const_type() {
    const PI: f32 = 3.14159;
    println!(
        "To make an apple {} from scratch, \
            you must first create a universe.",
        PI
    );
    
}

/// # 数组
///
/// 数组是相同类型元组的固定集合
///
/// 一个数组的数据类型[T; N], T是元素类型，N是编译时已知的固定长度
///
/// 可以使用[x]运算符来检索单个元素，其中x是元素的usize索引(从0开始)
pub fn array_type() {
    let num = [1, 2, 3, 4];
    println!("num = {:?}", num);
    println!("num[0] = {}", num[0]);
}

/// # 函数
///
/// 函数有0个或者多个参数
///
/// 函数总是遵循蛇形命名法
///
pub fn function_type() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let ret = add(1, 2);
    println!("add(1, 2) = {}", ret);
}

/// # 函数返回多个值
///
/// 函数可以通过元组返回多个值
///
/// 元组可以通过他们的索引来获取，也可以通过后面的模式匹配来解构
///
pub fn extra_tuple() {
    fn swap(x: i32, y: i32) -> (i32, i32) {
        (y, x)
    }

    let ret1 = swap(1, 2);
    println!(
        "ret1 = {:?}, ret1.0 = {}, ret1.1 = {}",
        ret1, ret1.0, ret1.1
    );

    let ret2 = swap(ret1.0, ret1.1);
    println!(
        "ret2 = {:?}, ret2.0 = {}, ret2.1 = {}",
        ret2, ret2.0, ret2.1
    );
}

/// # 函数返回空值
///
/// 如果没有为函数指定返回类型，它将返回一个为空的元组， 称为单元类型， （）。
///
pub fn return_nothing() {
    fn make_nothing1() -> () {
        println!("make nothing1");
    }
    fn make_nothing2() {
        println!("make nothing2");
    }

    let a = make_nothing1();
    let b = make_nothing2();

    println!("a = {:?}, b = {:?}", a, b);
}