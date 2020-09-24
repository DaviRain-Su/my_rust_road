use std::str;
/// # 字符编码 P246
///
/// 使用str模块中提供的from_utf8方法并其传递一个UTF-8字节序列&[0xE9u8, 0x81u8, 0x93u8]作为
/// 参数，将其转换为字符串"道", 在Rust中，使用u8来表示字节类型，如果此处没有加u8后缀，Rust
/// 也会通过from_utf8的函数签名推导出此数组参数为u8类型数组，也可以通过Stirng::from("\u{9053}")
/// 方法将一个十六进制形式的Unicode码位转换为字符串"道"
///
#[test]
fn test_str() {
    let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);
    assert_eq!("道", String::from("\u{9053}"));
    let unicode_x = 0x9053;
    let utf_x_hex = 0xe98193;
    let utf_x_bin = 0b111010011000000110010011;
    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: 0x{:x}", utf_x_bin);
}

/// # 字符 P247
///
/// Rust中使用char类型表示单个字符。char类型使用整数值与Unicode标量值一一对应
///
///
/// Example: 字符与标量值一一对应
///
/// 声明字符'道',注意，这里使用单引号来定义字符，使用双引号定义的是字符串字面量，在Rust
/// 中每个char类型的字符都代表了一个有效的u32类型的整数，但不是每个u32类型的整数都能代表
/// 一个有效的字符，因为并不是每个整数都属于Unicode标量值。
///
/// 通过as将char类型转换为u32类型，那么字符tao对应的u32整数值为36947.
/// 通过char类型内建的escape_unicode方法也可以得到其Unicode标量值。
///
/// 为了能够存储任何Unicode标量值，Rust规定每个字符都占4字节。
///
#[test]
fn test_char() {
    let tao = '道';
    let tao_u32 = tao as u32;
    assert_eq!(36947, tao_u32);
    println!("U+{:x}", tao_u32);
    println!("{}", tao.escape_unicode());
    assert_eq!(char::from(65), 'A');
    assert_eq!(std::char::from_u32(0x9053), Some('道'));
    assert_eq!(std::char::from_u32(36947), Some('道'));
    assert_eq!(std::char::from_u32(1290101010), None);
}

/// 将字符转换为字符串要注意字节长度
///
/// 定义了一个可变数组b，将其作为参数传入字符内建的encode_utf8方法
/// 将字符转换为一个字符串字面量，这是需要注意的是，如果将数组b的长度改为1或2，
/// 则无法将tao转换为字符串，因为字符'道'的UTF-8编码占三个字节，所以，如果要
/// 转换为合法的字符串，则数组b的长度最少为3。
/// 通过字符内建的len_uft8方法，也可以获得字符tao按UTF-8编码的字节长度
///
/// 需要注意的是只有包含单个Unicode标量值（实际码位）的才能被声明为字符
///
///
#[test]
fn test_str_tran_len() {
    let mut b = [0; 4];
    let tao = '道';
    let tao_str = tao.encode_utf8(&mut b);
    //encode_utf8: need 3 bytes to encode U+9053, but the buffer has 2'
    assert_eq!("道", tao_str);
    assert_eq!(3, tao.len_utf8());

    let e = '†';
    println!("{} : {}", e as u32, e);
}

/// char中常用的一些方法来处理字符
///
/// - is_digit(16), 用于判断给定字符是否属于十六进制形式，如果参数为10，则判断是否为十进制
/// - to_digit(16), 用于将给定字符转换为十六进制，如果参数为10，则将给定字符转换为十进制。
/// - is_lowercase, 用于判断给定字符是否为小写，作用于Unicode字符集中具有Lowercase属性的字符
/// - is_uppercase, 用于判断给定字符是否为大写的，作用于Unicode字符集中具有Uppercase属性的字符。
/// - to_lowercase, 用于将给定字符转换为小写的，作用于Unicode字符集中具有Lowercase属性的字符
/// - is_whitespace, 用于判断给定字符（或16进制形式的码点)是否为空格字符
/// - is_alphabetic, 用于判断给定字符是否为字符，汉字也算是字母
/// - is_alphaumberic, 用于判断给定字符是都为字母、数字。
/// - is_control, 用于判断给定字符是否为控制符
/// - is_numberic, 用于判断给定字符是否为数字
/// - excape_default, 用于转义\t, \r, \n, 单引号、双引号、反斜杠等特殊符号
///
#[test]
fn test_char_func() {
    assert_eq!(true, 'f'.is_digit(16));
    assert_eq!(Some(15), 'f'.to_digit(16));
    assert!('a'.is_lowercase());
    assert!(!'道'.is_lowercase());
    assert!(!'a'.is_uppercase());
    assert!('A'.is_uppercase());
    assert!(!'中'.is_uppercase());
    // assert_eq!('i', 'I'.to_lowercase());
    // assert_eq!('B', 'b'.to_uppercase());
    assert!(' '.is_whitespace());
    assert!('\t'.is_whitespace());
    assert!('\n'.is_whitespace());
    assert!('\u{A0}'.is_whitespace());
    assert!(!'越'.is_whitespace());
    assert!('a'.is_alphabetic());
    assert!('京'.is_alphabetic());
    assert!(!'1'.is_alphabetic());
    assert!('7'.is_alphanumeric());
    assert!('K'.is_alphanumeric());
    assert!('藏'.is_alphanumeric());
    assert!(!' '.is_control());
    assert!(!'q'.is_control());
    assert!('7'.is_numeric());
    assert!(!'藏'.is_numeric());
    println!("{}", '\r'.escape_default());
}

/// # 字符串分类
///
/// 字符串是由字符组成的有限序列，字符可以用整数值直接表示Unicode标量值，然而字符串
/// 却不能，因为字符串不能确定大小，所以在Rust中字符串是UTF-8编码序列，处于安全的kaol，在
/// Rust中字符串分为以下几种类型：
/// - str, 表示固定长度的字符串
/// - Sting, 表示可增长的字符串
/// - Cstr, 表示由C分配而被Rust借用的字符串，一般用于和C语言交互
/// - CString, 表示由Rust分配且可以传递给C函数使用的C字符串，同样用于和C语言交互
/// - OsStr,表示和操作系统相关的字符串，这是为了兼容Windows系统
/// - OsSting,表示OsStr的可变版本，与Rust字符串可以相互转换
/// - Path, 表示路径，定义std::path模块中，Path包装了OsStr
/// - PathBuf,与Path配对，是Path的可变版本，PathBuf包装了OsString
///
/// 在Rust中最常见的的字符串是str和String。
/// str属于动态大小类型DST, 在编译器是不可变的UTF-8字节序列，创建后无法再为其追加内容
/// 或更改其内容。&str类型的字符串可以存储在任意地方。
/// - 静态存储区。又代表性的是字符串字面量，&'static str 类型的字符串被直接存储到已
/// 编译的可执行文件宏，随程序一起加载启动
/// - 堆分配，如果&str类型的字符串是通过堆String类型的字符串取切片生成的，则存储在堆上，
/// 因为String类型的字符串是堆分配的，&str只不过是在其堆上的切片。
/// - 栈分配，比如使用str::from_utf8方法，就可以将栈分配的[u8; N]数组转换为一个&str字符串
///
/// &str是一个引用类型，而String类型的字符串拥有所有权.
/// String是由标准库提供的可变字符串，可以在创建后为其追加内容或更改其内容。
/// String类型本质为一个成员变量是Vec<u8>类型的结构体，所以他是直接将字符内存放于堆中的。
///
/// String类型由三部分组成： 指向堆中字节序列的指针，as_ptr方法，记录堆中字节序列的字节长度len方法
/// 和堆分配的容量capacity方法。
#[test]
fn test_str_type() {
    let mut a = String::from("fooo");
    println!("{:p}", a.as_ptr()); // as_ptr获取的堆中字节序列的指针地址
    println!("{:p}", &a); // 通过引用操作得到的是地址为字符串字面量在上上指针的地址
    assert_eq!(a.len(), 4); // 通过len方法获取的是堆中字节序列的字节数，而非字符个数
    a.reserve(10); // reserver方法可以为字符串再次分配容量，
    assert_eq!(a.capacity(), 14);
}

/// 创建字符串的各种方法
///
/// 通过String::from方法使用字符串字面量作为参数来创建字符串，这是因为String类型实现了From triat
///
/// 通过String::with_capacity方法来创建空字符串，但是与String::new 方法不同的是，
/// with_capacity方法接收一个usize类型的参数，用于指定创建字符串预先要在堆上分配的
/// 容量空间。
///
/// 如果预先知道最终要创建的字符串长度，则用此方法可以降低分配堆空间的频率，这里需要注意的是，
/// 容量支持存储空间的一种刻度，实际申请的堆内存空间为每个字符的字节大小乘以容量值。
///
///
#[test]
fn test_create_string() {
    let string = String::new();
    assert_eq!("", string);

    let string = String::from("hello rust");
    assert_eq!("hello rust", string);

    let string = String::with_capacity(20);
    assert_eq!("", string);

    let str: &'static str = "the tao of rust";
    let string: String = str.chars().filter(|c| !c.is_whitespace()).collect(); // chars方法返回的迭代器实现了FromIterator trait
    assert_eq!("thetaoofrust", string);

    let string = str.to_owned(); // to_owned方法利用&str切片序列生成新的String字符串
    assert_eq!("the tao of rust", string);

    let string = str.to_string(); // to_string方法是对String::from的包装
    let str = &string[11..15];
    assert_eq!("rust", str);
}

/// # 8.1.4 字符串的两种处理方式
///
/// Rust中的字符串不能使用索引访问其中的字符，因为字符串是UTF-8字节序列，
/// 到底返回字节还是码点是一个问题。
///
/// rust提供了bytes和chars两个方法来分别返回按字节和按字符迭代的迭代器。
/// 在rust中对字符串的操作大致分为两种方式，按字节处理和按字符处理
///
///
#[test]
fn test_operator_str() {
    let str = "boos";
    let mut chars = str.chars(); // 按码位进行迭代
    while let Some(c) = chars.next() {
        println!("c = {}", c);
    }
    let mut bytes = str.bytes(); // 按字节进行迭代
    assert_eq!(4, bytes.len()); // len() 方法返回的是字符串字节长度，而非字符长度
    while let Some(b) = bytes.next() {
        println!("b = {}", b);
    }
}

/// # 通过get,get_mut访问字符
///
/// 字符串不能通过索引来访问字符，但是rust提供了， get和get_mut，可以指定索引范围
/// 来获取字符串切片，并且rust默认会检查字符串的序列是否为有效的utf-8序列
///
/// 通过给get方法传递索引范围，获取到了预期的字符串切片，注意这里返回的是Option类型
///
/// is_char_boundary方法来验证某个索引位置是否为合法的字符边界
#[test]
fn test_iter_str() {
    let mut v = String::from("hello");
    assert_eq!(Some("h"), v.get(0..1));
    assert_eq!(Some("e"), v.get(1..2));
    println!("{:?}", v.get_mut(4..));
    // assert!(v.get_mut(4..).is_none());
    assert!(v.is_char_boundary(4));
    // assert!(v.get_mut(..5).is_none());
    // assert!(v.get_mut(..).is_none());
}

/// 在使用字符串内奸的split_at和split_at_mut方法切割字符串时，要注意，一定要使用合法的字符串
/// 边界索引，否则就会引起线程崩溃
///
///
/// 在日常处理字符串时，要注意是按字节还是按字符进行的，以避免发生预期之外的错误
///
#[test]
fn test_split_str() {
    let s = "Per Martin-ldf";
    let (first, last) = s.split_at(12);
    println!("first = {}", first);
    println!("last = {}", last);
}

/// # 8.1.5 字符串的修改
///
/// 追加、插入、链接、更新和删除
///
/// ## 追加字符串
///
/// push()
/// push_str()
///
/// 使用push方法为String类型字符串hello追加字符，使用push_str方法为hello追加&str
/// 类型的字符串切片，
///
/// push和push_str在内部实现上其实是类似的，因为String本质上对Vec<u8>动态数组的包装
/// 所以对于push来说，如果字符是单字节的，则将字符转换为u8类型直接追加到Vec<u8>尾部；
/// 如果是多字节的，则转换为UTF-8字节序列，通过Vec<u8>的extend_from_slice方法来扩展
/// 因为push_str接收的是&str类型的字符串切片，所以直接使用extend_from_slice方法来扩展S
/// String类型字符串的内部Vec<u8>数组
///
#[test]
fn append_str_ex() {
    let mut hello = String::from("hello, ");
    hello.push('R');
    hello.push_str("ust!");
    println!("{}", hello);
}

/// Extend 追加字符串
///
/// String类型的字符串实现了Extend迭代器，所以可以使用extend方法，其参数为迭代器
#[test]
fn test_iter_append() {
    let mut message = String::from("hello");
    message.extend([',', 'r', 'u'].iter()); // Iter迭代器
    message.extend("st ".chars()); // Chars迭代器
    message.extend("w o r l d".split_whitespace()); // SplitWhitespace迭代器
    println!("{}", message);
}


/// 插入字符串
/// 
/// 如果想要从字符串的某个位置开始插入一段字符串，则需要使用insert, insert_str方法
///
/// 
/// insert方法的参数为要插入的位置和字符
/// insert_str方法的参数为要插入的位置和字符串切片
/// 
/// 需要注意的是， insert和 insert_str是基于字节序列的索引记性操作的，其内部实现会通过
/// is_char_boundary方法来判断插入的位置是否为合法的字符边界，如果插入的位置非法，则会引发线程崩溃。
/// 
#[test]
fn inset_str_ex () {
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');
    s.insert_str(0, "bar");
    println!("s = {}", s);
}

