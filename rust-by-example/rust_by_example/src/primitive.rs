

/// # 原生类型
///
///
/// ## 标量类型
///
/// - 有符号整型： i8, i16, i32, i64, isize
/// - 无符号整型： u8, u16, u32, u64, usize
/// - 浮点类型： f32, f64
/// - char： 单个Unicode字符， 'a', 'b', 每个都是4字节
/// - bool: 只能是true, false
/// - 单元类型： (), 其唯一可能的值就是()这个空元组， 不是复合类型，因为并不包含多个值
///
/// ## 复合类型
///
/// - 数组 ：[1, 2, 3]
/// - 元组： (1, 2, 3)
///
/// 变量都能够显式地给出类型说明， 数字还可以通过后缀或者默认方式来声明类型，
/// 整型默认为i32, 浮点类型默认为f64, 注意Rust还可以根据上下文来推断类型，
/// 仅当根据环境无法推断时，才按默认方式取整型数值i32, 浮点数值为f64
///
pub fn primitive_example() {
    let _logical : bool  = true;

    let _a_float : f64 =1.0;
    let _an_integer = 5i32;

    let _default_float = 3.0;
    let _defaut_integer = 7 ;
    let mut _inferred_type = 12;
    _inferred_type = 424848i64;

    let mut _mutable = 12;
    _mutable = 21;

    // mutable = true;

    let _mutable = true;

}

/// # 字面量和运算符
///
///
pub fn operator_and_type_example() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);
    // println!("1 - 2 = {}", 1u32 - 2); //error in debug in release ok

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}

/// # 元组
///
/// 元组是一个可以包含各种类型的值的组合，元组使用括号（）来构成，而每个元组自身又是一个类型标记为
/// (T1, T2, T3, ...)的值，其中T1, T2, 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多的值
///
pub fn tuple_type_example() {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    use std::fmt;

    impl  fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})\n", self.0, self.1)?;
            write!(f, "({} {})", self.2, self.3)
        }
    }

    impl Matrix {
        fn transpose(&self) -> Self {
            Matrix(self.0, self.2, self.1, self.3)
        }
    }

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true
    );

    println!("long tuple is {:?}", long_tuple);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let _tuple_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    //元组能打印的最大长度是12
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tupel : {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    println!("on element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: \n{}", matrix);
    println!("Transpose: \n{}", matrix.transpose());

}

/// # 数组和切片
///
///
/// 数组是一组拥有相同类型T的对象的集合， 在内存中是连续存储的，数组使用中括号[],
/// 来创建，且他们的大小在编译时就会被确定。数组的类型标记为[T;size]
///
/// 切片类型和数组类似，但其大小在编译时是不确定的，相反，切片是一个双字对象
/// ，第一个字是一个指向数组的指针，第二字是切片的长度，这个字的宽度和usize相同，
/// 由处理器架构决定，
/// slice可以用来借用数组的一部分，slice的类型标记为&[T]
///
///
pub fn slice_and_type_example() {
    use std::mem;

    fn analyze_slice(slice:&[i32]) {
        if slice.is_empty() {
            println!("the slice is empty!");
        }
        else {
            println!("first element of the slice : {}", slice[0]);
            println!("the slice has {} elements", slice.len());
        }
    }

    let xs  = [1, 2, 3, 4, 5];

    let ys = [0; 500];

    println!("first element of the array : {}", xs[0]);
    println!("second element of the array : {}", xs[1]);
    println!("array size : {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // println!("{}", xs[5]);

    let xs: [i32;0] = [];

    analyze_slice(&xs);
}
