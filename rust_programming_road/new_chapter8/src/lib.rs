/// # 第8章 字符串与集合类型
///
/// 数据结构体是计算机存储和组织数据的方式。
/// 对于不同的场景，精心选择的数据结构可能带来更高的运行效率或存储效率。
/// 通常，通过确定数据结构来选择相应的算法，也可能通过算法来选择数据结构，
/// 不管是哪种情况，选择合适的数据结构都相当重要。
///
///
/// 程序中最常用的三大数据结构是字符串、数组和映射。
///
/// - 字符串是特殊的线性表，是由零个或者多个字符串组成的有限序列。
/// - 但是字符串和数组、映射的区别在于，字符串是被作为一个整体来关注和使用的；
/// 而数组和映射关注的最多的是其中的元素及它们之间的关系。所以
/// 数据和映射也被称为集合类型。
///  
///
/// 国际标准化组织制定了通用的多字节编码字符集，也就是
/// Unicode字符集，Unicode字符集相当于一张表。其中
/// 包含了世界上所有语言中可能出现的字符，每个字符对应一个非负数
/// ，该数字称为码点(code Point). 这些码点也分为不同的类型，
/// 包括标量值(Scala Value)、代理对码点，非字符码点，保留码点和私有码点。
///
/// 其中标量值最为常用，他是指实际存在对应字符的码位，其范围时0x0000~0xD7FF, 和
/// 0xE000~0x10FFFF两段。
///
/// Unicode字符集只规定了字符所对应的码点，却没有指定如何存储。
/// 如果直接存储码点，则太耗费空间了，因为Unicode字符集的每个
/// 字符都占4字节。传输效率非常低。虽然Unicode字符集解决了字符通用的问题，但是必须寻求
/// 另一种存储方式，在保证Unicode字符集通用的情况下更加节约流量和硬盘空间。这种存储方式就是码元
/// (code Unit)组成的序列。
///
/// 码元是指用于处理和交换编码文本的最小比特组合。比如计算机处理
/// 字符的最小单位1字节就是一个码元，通过将Unicode标量值和码元序列
/// 建立一一映射关系，就构成了编码表。在Unciode中一共有三种这样的字符编码表
/// UTF-8, UTF-16, UTF-32，他们正好对应1字节，2字节和4字节。
/// 对于UTF-16和UTF-32来说，因为他们的码元分别是2字节和4字节，所以就得
/// 考虑字节序问题；而对于UTF-8来说，一个码元只有1字节，所以不存在字节序问题，可以直接存储。
///
///
/// UTF-8编码规则：
///
/// - 当一个字符在ASCII码的范围内时，就用1字节表示，因为ASCII
/// 码中的字符最多使用7个比特位，所以前面需要补0.
///
/// - 当一个字符占用了n字节时，第一个字节的前n位设置为1，第n+1位设置为0，后面字节的前两位设置为10.
///
///
/// 将Unicode码位转换为字节序列的过程，就叫做编码(Encode).
/// 反过来，将编码字节序列转变为字符集中码位的过程，就叫做解码(Decode)
///
/// UTF-8编码的好处
///
/// 就是在实际传输过程中其占据的长度不是固定的，在保证Unicode通用型的情况下避免了流量和空间的浪费，
/// 而且还保证了在传输过程中不会错判字符。想一想，如果只是按Unicode
/// 码位存储，则在传输过程中是按固定的字节长度来识别字符的，
/// 如果在传输过程中出现问题，就会发生错判字符的可能，
/// UTF-8才能被广泛应用于互联中。
///
///
///
///
///
#[test]
fn test_example() {
    use std::str;
    //通过UTF_8获得字符串
    let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);
    // 通过Uncide获得字符串
    let tao = String::from("\u{9053}");
    assert_eq!("道", tao);

    let unicode_x = 0x9053;
    let utf_x_hex = 0xe98193;
    let utf_x_bin = 0b111010011000000110010011;
    println!("unicode_x : {:b}", unicode_x);
    println!("utf_x_hex : {:b}", utf_x_hex);
    println!("utf_x_bin : 0x{:x}", utf_x_bin);
}

/// 8.1.2 字符
///
/// Rust使用char类型表示单个字符。char类型使用整数值与Unicode标量值一一对应。
///
/// Rust中的char类型的大小是四字节的。
/// C/C++中的char类型大小是1字节。
///
/// 在Rust中每个char类型的字符都代表一个有效的u32类型的整数，
/// 但不是每个u32类型的整数都能代表一个有效的字符，因为并不是每个整数都属于
/// Unicode标量值， 如： assert_eq!(std::char::from_u32(1290101010), None); // error: literal out of range for `u32`
///
/// 为了能哦存储任何Unicode标量值，Rust规定每个字符都占4字节。
///
#[test]
// 字符与标量值一一对应
fn test_str_ex1() {
    let tao = '道';
    let tao_u32 = tao as u32; // 使用了as进行显示的类型转换了，
                              //Rust 是强类型的，没有默认的隐式类型转换，所有类型转换都需要显式的去写出来。
    assert_eq!(36947, tao_u32);
    println!("U+{:x}", tao_u32); // U+9053
    println!("{}", tao.escape_unicode()); // \u{9053}
                                          // for c in '❤'.escape_unicode() {
                                          //     print!("{}", c); // \, u, {, 2, 7, 6, 4, },
                                          // }
    assert_eq!(char::from(65), 'A'); // ASCII
    assert_eq!(std::char::from_u32(0x9053), Some('道'));
    assert_eq!(std::char::from_u32(36947), Some('道'));
    // assert_eq!(std::char::from_u32(0xe98193), Some('道'));
    assert_eq!(std::char::from_u32(1290101010), None); // error: literal out of range for `u32`
}

/// 将字符转换为字符串需要注意字节长度
///
/// 需要注意的是，只有包含单个Unicode标量值（实际码位）的才能被声明为字符
///
#[test]
fn test_str_ex2() {
    let mut b = [0; 3]; // >= 3
                        // let mut b = [0; 2];
                        // encode_utf8: need 3 bytes to encode U+9053, but the buffer has 2'
    let tao = '道';
    let tao_str = tao.encode_utf8(&mut b);
    assert_eq!("道", tao_str);
    assert_eq!(3, tao.len_utf8()); // 通过字符内建方法len_urf8获得，获得字符
                                   // 套按UTF-8编码的字节长度
}

/// char类型中的一些内建方法
///
/// - is_digit(16),用于判断给定字符是否属于十六进制形式，如果参数为10，则判断是否为十进制
/// - to_digit(16), 用于将给定字符转换为16进制形式，如果参数为10，则将给定字符转换为10进制
/// - is_lowercase, 用于判断给定字符是否为小写的，作用于Unicode字符集中具有Lowercase属性的字符
/// - is_uppercase，用于判断给定字符是否为大写的，作用于Uncode字符集中具有Uppercase属性的字符
/// - to_lowercase, 用于将给定字符转换为小写的，作用于Unicode字符集中具有Lowercase属性的字符
/// - to_uppercase, 用于将给定字符转换为大写的，作用于Unicode字符集中具有Uppercase属性的字符
/// - is_whitesapce, 用于判断给定字符是否为空格字符
/// - is_alphabetic, 用于判断给定字符是否为字母、汉字也算是字母
/// - is_alphanumeric, 用于判断给定字符是否为字母、数字
/// - is_control, 用于判断给定字符是否为控制符
/// - is_numbeic, 用于判断给定字符是否为数字。
/// - escape_default, 用于转义\t, \r, \n, 单引号，双引号，反斜杠等特殊符号。
///
#[test]
fn test_str_ex3() {
    // is_digit
    assert_eq!(true, 'f'.is_digit(16));
    assert!('1'.is_digit(10));
    assert!('f'.is_digit(16));
    assert!(!'f'.is_digit(10));
    // to_digit
    assert_eq!(Some(15), 'f'.to_digit(16));
    assert_eq!('f'.to_digit(10), None);
    assert_eq!('z'.to_digit(16), None);
    assert_eq!('1'.to_digit(10), Some(1));
    assert_eq!('f'.to_digit(16), Some(15));
    // is_lowercase
    assert!('a'.is_lowercase());
    assert!('a'.is_lowercase());
    assert!('δ'.is_lowercase());
    assert!(!'A'.is_lowercase());
    assert!(!'Δ'.is_lowercase());
    assert!(!'中'.is_lowercase());
    assert!(!' '.is_lowercase());
    //is_uppercase
    assert!(!'a'.is_uppercase());
    assert!('A'.is_uppercase());
    assert!(!'中'.is_uppercase());
    assert!(!'a'.is_uppercase());
    assert!(!'δ'.is_uppercase());
    assert!('A'.is_uppercase());
    assert!('Δ'.is_uppercase());
    assert!(!'中'.is_uppercase());
    assert!(!' '.is_uppercase());
    // to_lowercase
    // assert_eq!('i', 'I'.to_lowercase());
    // mismatched types
    // expected `char`, found struct `std::char::ToLowercase`rustc(E0308)
    // example
    // for c in 'İ'.to_lowercase() {
    //     print!("{}", c);
    // }
    assert_eq!('i', 'I'.to_lowercase().next().unwrap());
    // println!("i = {}", 'I'.to_lowercase().next().unwrap());
    //to_uppercase
    assert_eq!('B', 'b'.to_uppercase().next().unwrap());
    // is_whitespace
    assert!(' '.is_whitespace());
    assert!('\t'.is_whitespace());
    assert!('\n'.is_whitespace());
    assert!('\u{A0}'.is_whitespace());
    assert!(!'越'.is_whitespace());
    // is_alpabetic
    // Returns true if this char has the Alphabetic property.
    assert!('a'.is_alphabetic());
    assert!('中'.is_alphabetic());
    assert!(!'1'.is_alphabetic());
    // is_alphanumeric
    assert!('7'.is_alphanumeric());
    assert!('K'.is_alphanumeric());
    assert!('藏'.is_alphanumeric());
    // is_control
    assert!(!' '.is_control());
    assert!(!'q'.is_control());
    // is_numberic
    assert!('7'.is_numeric());
    assert!(!'曾'.is_numeric());
    //excape_default
    println!("{}", '\r'.escape_default());
    println!("{}", '\t'.escape_default());
    println!("{}", '\n'.escape_default());
    println!("{}", '\''.escape_default());
    println!("{}", '\"'.escape_default());
    println!("{}", '\\'.escape_default());
}

/// ### 8.1.3 字符串分类
///
/// 字符串是由字符组成的有限序列，字符可以用整数值直接表示Unicode标量值，
/// 然而字符串不能，因为字符串不能确定大小，所以在Rust中字符是UTF-8编码序列，
///
/// 出于安全的考虑，在Rust中字符串分为了以下几种类型：
/// - Str ,表示固定长度的字符串
/// - String， 表示可增长的字符串
/// - CStr， 表示由C分配而被Rust借用的字符串，一般用于和C语言进行交互
/// - CString，表示由Rust分配且可以传递给C函数使用的C字符串，同样用于和C语言进行交互。
/// - OsStr, 表示和操作系统相关的字符串，这是为了兼容Windows系统。
/// - OsString, 表示OsStr的可变版本，与Rust字符串可以相互转换。
/// - Path， 表示路径，定义std::path模块，Path包装了OsStr.
/// - PathBuf, 跟Path配对，是Path的可变版本，PathBuf包装了OsSting。
///
///
/// str属于动态大小类型DST， 在编译期不能确定其大小， 所以在程序中
/// 最常见到的str是切片(slice)类型&str, &str代表的是不可变的UTF-8字节序列。
/// 创建后无法再为其追加或更改其内容。
///
/// &str类型的字符串可以存放在任意地方：
/// - 静态存储， 最有代表性的是字符串字面量，&'static str类型的字符串
/// 被直接存储在已经编译的可执行文件中，随着程序一起被加载启动。
///
/// - 堆分配，如果&str类型的字符串是通过堆String类型的字符串切片生成的，则
/// 存储在堆上。因为String类型的字符串是堆分配的，&str只不过是其堆上的切片。
///
/// - 栈分配， 比如使用str::from_utf8方法，就可以将栈分配的[u8;N]数组转换为一个&str
/// 字符串： let tao = std::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
///
/// &str是一个引用类型，
/// String类型的字符串拥有所有权，String是由标准库提供的可变字符串，
/// 可以在创建后为其追加内容或更改内容。String类型本质为一个成员变量是
/// Vec<u8>类型的结构体，所以他是直接将字符存放在堆中的。
///
/// String类型由三部分组成：
/// - 指向堆中字节序列的指针(as_ptr方法)
/// - 记录堆中字节序列的字节长度(len方法)
/// - 对分配的容量(capacity方法)
///
///
///
///
///
///
#[test]
fn test_str_three_part() {
    let mut a = String::from("foorbar");
    println!("{:p}", a.as_ptr());   // 0x7ff2b8d04250
    println!("{:p}", &a);           // 0x7000012ee588
    assert_eq!(a.len(), 7); // 注意这是字节数
    a.reserve(10); // rserve方法可以为字符串再次分配容量
    assert_eq!(a.capacity(), 17);
}

/// 创建字符串的多个方法
/// 
#[test]
fn test_construct_string() {
    // 1. Sting::new()
    let string = String::new();
    assert_eq!("", string);
    
    // 2. String::from()
    let string = String::from("hello rust");
    assert_eq!("hello rust", string);

    let string = String::with_capacity(10);
    assert_eq!("", string);

    let str : &'static str = "the tao of rust";
    let string : String = str
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    assert_eq!("thetaoofrust", string);
    // to_owned
    // Creates owned data from borrowed data, usually by cloning.
    let s: &str = "a";
    let _ss: String = s.to_owned();

    let v: &[i32] = &[1, 2];
    let _vv: Vec<i32> = v.to_owned();
    //--------------------------
    let string = str.to_owned();
    assert_eq!("the tao of rust", string);
    // to_string
    // Converts the given value to a String.
    let i = 5;
    let five = String::from("5");

    assert_eq!(five, i.to_string());

    let string = str.to_string();
    let str = &string[11..15];
    assert_eq!("rust", str);
    
}

/// 8.1.4 字符串的两种处理方式
/// 
/// rust中的字符串不能使用索引访问其中的字符，因为字符串是UTF-8字节序列，
/// 这里说到底还是返回字节还是码点的问题，
/// 
/// rust分别提供了bytes和chars两个方法来分别返回按字节和按字符迭代的迭代器。
/// 
/// 所以，在Rust中对字符串的操作大致分为两种方式：按字节处理和按字符处理
/// 
/// 
#[test]
fn test_opeator_two_way(){
    let str = "bool";
    // chars
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        println!("c = {}", c);
    }
    // bytes
    assert_eq!(4, str.len());
    let mut bytes = str.bytes();
    while let Some(c) = bytes.next() {
        println!("c = {}", c);
    }
}

/// 访问字符的两种方式， get, get_mut
/// 可以通过索引范围来获取字串切片，并且Rust默认会
/// 检查字符串的序列是否为有效的UTF-8序列
/// 
#[test]
fn test_observe_two_way() {
    let mut v = String::from("hello");
    assert_eq!(Some("h"), v.get(0..1));
    assert_eq!(Some("e"), v.get(1..2));
    assert_eq!(Some("llo"), v.get(2..));
    assert!(v.is_char_boundary(3));
}

/// 字符串内建的split_at和split_at_mut方法分割字符串
/// 需要注意的是，一定要使用合法的字符串边界索引，否则会引起线程崩溃
/// 
#[test]
fn test_split_example() {
    let s = "Per Martin-lof";
    let (first, last) = s.split_at(12);
    assert_eq!("Per Martin-l", first);
    assert_eq!("of", last);
}

/// ## 8.1.5 字符串的修改
/// 
/// 一般情况下，如果需要修改字符串，则使用Strign类型，修改字符串大致分为追加、插入、连接、更新和删除5中情形。
/// 
/// ### 追加字符串
/// 
/// 对于追加的情形，Rust提供了push和push_str两个方法
/// 
#[test]
fn test_str_push() {
    let mut hello = String::from("Hello, ");
    hello.push('R');
    hello.push_str("ust!");
    assert_eq!("Hello, Rust!", hello);
}

/// 通过迭代器为String追加字符串，因为String实现了Extend迭代器
/// 
#[test]
fn test_extend_str() {
    let mut message = String::from("hello");
    message.extend([',', 'r','u'].iter());
    message.extend("st ".chars());
    message.extend("w o r l d".split_whitespace());
    assert_eq!("hello,rust world", &message);
}

/// 插入字符串
/// 
/// 通过insert ---> char
/// insert_str ----> &str
/// 
/// 需要注意的是，insert, insert_str是基于字节序列
/// 的索引进行操作的，其内部实现会通过is_char_boundary方法
/// 来判断插入的位置是否为合法的字符边界，如果插入的位置是非法的，则会引发线程
/// 崩溃。
/// 
#[test]
fn test_insert_string () {
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');
    s.insert_str(2, "bar");
    assert_eq!("foobar", s);
}

/// 连接字符串
/// 
/// String 类型的字符串也实现了Add<&str> 和AddAssign<&str>
/// 两个triat，这意味着可以使用"+", "+="操作符来连接字符串
/// 
#[test]
fn test_add_string() {
    let left = "the tao".to_string();
    let mut right = "Rust".to_string();
    assert_eq!(left + " of " + &right, "the tao of Rust");
    right += "!";
    assert_eq!(right, "Rust!");
}

/// ### 更新字符串
/// 
/// 因为Rust不支持直接按索引操作字符串中的字符，一些常规的算法在Rust中必然无法使用，
/// 比如像修改某个字符串中符合条件的字符为大写，就无法直接通过索引来操作，只能通过迭代器的方法或者某些unsafe方法
/// 
#[test]
fn test_update_string () {
    let s = String::from("foobar");
    let mut result = s.into_bytes();
    (0..result.len()).for_each(|i|{
        if i % 2 == 0 {
            result[i] = result[i].to_ascii_lowercase();
        }else {
            result[i] = result[i].to_ascii_uppercase();
        }
    });

    println!("{}", String::from_utf8(result).unwrap());


    let s = String::from("foobar");
    let s : String = s.chars().enumerate().map(|(i, c)|{
        if i % 2 == 0 {
            c.to_lowercase().to_string()
        }else {
            c.to_uppercase().to_string()
        }
    }).collect();
    println!("{}", s);
}


/// 删除字符串 
/// 
#[test]
fn test_delete_string () {
    let mut s = String::from("hello");
    // remove 也是按字节处理字符串的，如果给定的索引位置不是合法的字符边界，那么线程就会崩溃
    s.remove(3);  
    assert_eq!("helo", s);
    // pop
    assert_eq!(Some('o'), s.pop());
    assert_eq!(Some('l'), s.pop());
    assert_eq!(Some('e'), s.pop());
    assert_eq!("h", s);
    let mut s = String::from("hello");
    // truncate 方也是按字节进行操作的，所以使用时需要注意，如果给定的
    // 的索引位置不是合法的字符边界，则同样会引发线程崩溃
    s.truncate(3);
    assert_eq!("hel", s);
    s.clear();
    assert_eq!("", s);
    let mut s = String::from("a is aplha, b is beta");
    let beta_ofset = s.find('b').unwrap_or(s.len());
    let t : String = s.drain(..beta_ofset).collect();
    assert_eq!(t, "a is aplha, ");
    assert_eq!(s, "b is beta");
    s.drain(..);
    assert_eq!(s, "");
}