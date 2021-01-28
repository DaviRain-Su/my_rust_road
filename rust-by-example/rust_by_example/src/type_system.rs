/// # 类型系统
///
///
/// Rust提供了多种机制，用于改变或定义原生类型或用户定义类型
///
/// - 原生类型的类型转换
/// - 指定字面量的类型
/// - 使用类型推断
/// - 给类型取别名
///
/// # 类型转换
///
/// Rust不提供原生类型之间的隐式类型转换，可以使用as关键字进行显示类型转换
///
///
pub fn type_cast_example() {
    //不显示类型转换产生的溢出警告
    #![allow(overflowing_literals)]

    let decimal = 65.4321_f32;

    let integer : u8 = decimal as u8;
    let chacter = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, chacter);

    // 当把任何类型转换为无符号类型T时，会不断加上或减去(std::T::MAX + 1)
    //直到值位于新类型T的范围内

    println!("1000 as a u16 is : {}", 1000 as u16);

    //1000 - 256 - 256 -256 = 232
    println!("1000 as u8 is : {}", 1000 as u8);

    // -1 + 256 = 255
    println!(" -1 as a u8 is: {}", (-1i8) as u8);

    println!("1000 mode 256 is : {}", 1000 % 256);

    println!("128 as a i16 is : {}", 128 as i16);

    println!("128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);

    println!("232 as a i8 is : {}", 232 as i8);
}

/// #字面量
///
///
///
pub fn no_declare_type_example() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of x in bytes: {}", std::mem::size_of_val(&x));
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));
}

/// # 类型推断
///
///
pub fn type_infer_example() {
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}

/// #别名
///
/// 可以使用type语句给已有的类型取个新的名字，类型的名字
/// 必须遵守驼峰命名法，否则编译器将给出错误.
///
///  别名主要用于是避免冗长的模板代码，type IoResult<T> = Result<T, IoError>;
pub fn type_name_example() {
    type NanoSecond = u64;
    type Inch = u64;

    //这个属性屏蔽警告
    #[allow(non_camel_case_types)]
    type u64_t = u64;

    let nanoseconds : NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64_t;

    //类型别名不能提供额外的类型安全，应为别名不是新的类型
    println!("{} nanoseonds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}