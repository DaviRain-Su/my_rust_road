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
    println!("{:p}", a.as_ptr()); // 0x7ff2b8d04250
    println!("{:p}", &a); // 0x7000012ee588
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

    let str: &'static str = "the tao of rust";
    let string: String = str.chars().filter(|c| !c.is_whitespace()).collect();
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
fn test_opeator_two_way() {
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
    let v = String::from("hello");
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
    message.extend([',', 'r', 'u'].iter());
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
fn test_insert_string() {
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
fn test_update_string() {
    let s = String::from("foobar");
    let mut result = s.into_bytes();
    (0..result.len()).for_each(|i| {
        if i % 2 == 0 {
            result[i] = result[i].to_ascii_lowercase();
        } else {
            result[i] = result[i].to_ascii_uppercase();
        }
    });

    println!("{}", String::from_utf8(result).unwrap());

    let s = String::from("foobar");
    let s: String = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect();
    println!("{}", s);
}

/// 删除字符串
///
#[test]
fn test_delete_string() {
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
    let t: String = s.drain(..beta_ofset).collect();
    assert_eq!(t, "a is aplha, ");
    assert_eq!(s, "b is beta");
    s.drain(..);
    assert_eq!(s, "");
}

/// ### 8.1.6 字符串的查找
///
/// 分类：
///
/// - 存在性判断， contains, start_with, ends_with
/// - 位置匹配，find, rfind.
/// - 分割字符串, split, rspilt, split_terminator
/// - 捕获匹配, matches, rmatches, match_indices, rmatch_indices
/// - 删除匹配，trime_matches, trim_left_matches, trim_right_matches
/// - 替代匹配， replace, replacen
///
/// 1. 存在性判断
///
/// 通过contains方法判断字符串中是否存在符合指定条件的字符，该方法返回bool类型
///
/// std::str模块中contains方法的源码展示
///
/// pub fn contains<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool {
///     core_str::StrExt::contains(self, pat)
/// }
/// contains的参数pat是一个泛型，并且有一个Pattern<'a>限定，
/// Pattern<'a>是一个专门用于搜索&'a str字符串的模式triat. Rust中
/// 的char类型， String， &str, &&str, &[char]类型， 以及FnMut(char) -> bool的闭包
/// 均已实现了该triat，因为contains才可以接受不同类型的值作为参数。
///  
///
///
///
#[test]
fn test_contains() {
    let bananas = "bananas";
    // contains
    // Returns true if the given pattern matches a sub-slice of this string slice.
    // Returns false if it does not.
    // The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
    assert!(bananas.contains('a'));
    assert!(bananas.contains("an"));
    assert!(bananas.contains(char::is_lowercase));
    // start_with
    // Returns true if the given pattern matches a prefix of this string slice.
    // Returns false if it does not.
    assert!(bananas.starts_with('b'));
    assert!(bananas.starts_with("bana"));
    assert!(!bananas.starts_with("nana"));
    // ends_with
    // Returns true if the given pattern matches a suffix of this string slice.
    // Returns false if it does not.
    assert!(!bananas.ends_with("nana"));
    assert!(bananas.ends_with("anas"));
    assert!(!bananas.ends_with("nana"));
}

/// 2. 位置匹配
///
/// 如果想要查找指定字符串中字符所在的位置，则可以使用find方法，
///
/// find方法同样可以接受pattern参数，
///
/// find方法默认是从左向右按字符进行遍历查找的，最终返回Option<usize>类型的位置索引
/// 如果没有找到，则会返回None
///
/// rfind方法，表示从右到作来匹配字符串
///
#[test]
fn test_finds() {
    let s = "love 老虎 leopard";
    assert_eq!(s.find('v'), Some(2));
    assert_eq!(s.find('老'), Some(5));
    assert_eq!(s.find('虎'), Some(8));
    assert_eq!(s.find("leopard"), Some(12));
    assert_eq!(s.rfind('l'), Some(12));
    assert_eq!(s.find(char::is_whitespace), Some(4));
    assert_eq!(s.find(char::is_lowercase), Some(0));
}

/// 3 分割字符串
///
/// 通过制定的模式来分割字符串，使用split系列的方法
///
#[test]
fn test_splits() {
    let s = "love 老虎 leopard";
    let v = s
        .split(|c| (c as u32) >= (0x4E00 as u32) && (c as u32) <= (0x9FA5 as u32))
        .collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "abc1defXghi"
        .split(|c| c == '1' || c == 'X')
        .collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "May had a little lambda"
        .splitn(3, ' ')
        .collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "A.B.".split(".").collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "A.B.".split_terminator('.').collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "A..B..".split(".").collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "A..B..".split_terminator('.').collect::<Vec<&str>>();
    println!("v = {:?}", v);
}

/// 4 捕获匹配
///
/// 在处理字符串时，最常见的一个需求就是得到字符串中匹配某个条件的字符，通常
/// 通过正则表达式来完成。在Rust中，通过pattern参数配合matches系列的方法可以
/// 获得同样的效果
///
#[test]
fn test_matches() {
    let v = "abcXXXabcYYYabc".matches("abc").collect::<Vec<&str>>();
    println!("v = {:?}", v);
    let v = "1abc2abc3"
        .rmatches(char::is_numeric)
        .collect::<Vec<&str>>();
    println!("v = {:?}", v);

    let v = "abcXXabcYYabc".match_indices("abc").collect::<Vec<_>>();
    println!("v = {:?}", v);

    let v = "abcXXabcYYabc".rmatch_indices("abc").collect::<Vec<_>>();
    println!("v = {:?}", v);
}

/// 删除匹配
///
/// 在std::str模块中提供了trim系列方法，可以删除字符串两头的指定字符
///
///
/// trim, trime_left, trim_right这三个方法并不能使用pattern参数，只是
/// 固定地清楚空格、制表符和换行符。
/// Rust提供了trim_matches系列方法支持pattern参数，可以指定自定义的删除规则
#[test]
fn test_trims() {
    let s = " Hello\tworld\t";
    assert_eq!("Hello\tworld", s.trim());
    // assert_eq!("Hello\tworld\t", s.trim_left());
    // assert_eq!(" Hello\tworld", s.trim_right());
}

/// trim_matches系列方法使用
///
#[test]
fn test_trim_matches() {
    assert_eq!("Hello\tworld\t".trim_matches('\t'), "Hello\tworld");
    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
    assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");
    let x: &[char] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
    assert_eq!(
        "1foo1barXX".trim_matches(|c| c == '1' || c == 'X'),
        "foo1bar"
    );
    // assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
    // assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");
    // let x: &[char] = &['1', '2'];
    // assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
    // assert_eq!(
    //     "1foo1barXX".trim_right_matches(|c| c == '1' || c == 'X'),
    //     "1foo1bar"
    // );
}

/// 替代匹配
///
/// 使用trim_matches系列方法可以满足基本的字符串删除匹配需求
/// 但是其只能取去除字符串两头的字符，无法去除字符串内部包含的字符
/// 可以通过reolace系列方法来实现此需求
///
#[test]
fn test_replaces() {
    let s = "Hello\tworld\t";
    assert_eq!("Hello world ", s.replace("\t", " "));
    assert_eq!("Hello world", s.replace("\t", " ").trim());
    let s = "this is old old 123";
    assert_eq!("this is new new 123", s.replace("old", "new"));
    assert_eq!("this is new old 123", s.replacen("old", "new", 1));
    assert_eq!("this is ald ald 123", s.replacen('o', "a", 3));
    assert_eq!(
        "this is old old new23",
        s.replacen(char::is_numeric, "new", 1)
    )
}

/// 8.1.7 与其他类型相互转换
///
/// 将字符串转换为其他类型
///
/// 可以通过std::str模块中提供的parse泛型方法将字符串转换为指定类型
///
///
/// 其实parse方法内部是使用FromStr::from_str方法来实现
/// 转换的，FromStr是一个triat，其名命名复合Rust的一致性惯例
///
/// pub triat FromStr {
///     type Err;
///     fn from_str(s: &str) -> Reuslt<Self, Self::Err>;
/// }
///
/// 在FromStr中定义了一个from_str方法，实现了此triat的类型，可以通过from_str
/// 将字符串转换为该类型，返回一个Result类型，该类型会在解析失败时返回Err.
/// Rust为一些基本的原生类型，布尔类型以及IP地址等少数类型实现了FromStr,
/// 对于自定义的类型需要是自己手动实现
///
#[test]
fn test_pase() {
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);
    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);
}

/// 手动实现自定义结构体的例子
///
#[test]
fn test_self_define_struct() {
    // use std::convert::From;
    // use std::str::FromStr;
    // use std::num::ParseFloatError;
    // #[derive(Debug, PartialEq)]
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }

    // impl FromStr for Point {
    //     type Err = ParseFloatError;
    //     fn from_str(s: &str) -> Result<Self, Self::Err> {
    //         let coords : Vec<&str> = s.trim_matches(|p| p == '{' || p == '}')
    //                         .split(",")
    //                         .collect();

    //         let x_fromstr = coords[0].parse::<i32>()?;
    //         let y_fromstr = coords[1].parse::<i32>()?;

    //         Ok(Point { x: x_fromstr, y: y_fromstr })
    //     }
    // }

    // let p = Point::from_str("{1, 2}");
    // assert_eq!(p.unwrap(), Point{x: 1, y: 2});
    // let p = Point::from_str("{3, u}");
    // // Err(ParseIntError { kind: InvalidDigit})
    // println!("{:?}", p);
}

/// 将其他类型转换为字符串
///
/// 如果想把其他类型转换为字符串，则可以使用format!宏
/// format!宏与println!及write！宏类似，同样可以通过规则生成String
/// 类型的字符串
///
/// 基本的格式化规则
///
/// - 填充字符串宽度，格式为{:number}, 其中number表示数字，
/// 如果number的长度小于字符串长度，则什么都不做；如果number的长度大于字符串的长度，
/// 则会默认填充空格来扩展字符串的长度
///
/// - 截取字符串，格式{:.number}, 注意number前面有符号".",
/// number代表要截取的字符长度，也可以和填充格式配合使用
///
/// - 对齐字符串，格式为{:>}, {:^}, {:>}, 分别表示左对齐、位于中间和右对齐。
/// 也可以和其他格式代码配合使用
///
/// 可以直接在冒号后面使用"="和"*"代替默认的空格填充，
/// format!格式化字符串是按照字符来处理的，不管字符串多长，对于里面的Unicode码位都以单个字符来处理的。
///
#[test]
fn test_format() {
    let s = format!("{}Rust", "Hello");
    println!("s = {}", s);
    let s = format!("{:5}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:5.3}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:10}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:<12}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:>12}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:^12}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:^12.5}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:=^12.5}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:*^12.5}", "HelloRust");
    println!("s = {}", s);
    let s = format!("{:5}", "th\u{e9}");
    println!("s = {}", s);
}

/// 关于整数和浮点数的格式化代码
///
///
/// 针对整数提供的格式化代码
///
/// - 符号+, 表示强制输出整数的正负符号
/// - 符号#, 用于显示进制的前缀，比如十六进制显示0x, 二进制显示0b
/// - 数字0， 用于把默认填充的空格替换为数字0
///
///
#[test]
fn test_integer_format() {
    let s = format!("{:+}", 1234);
    println!("s = {}", s);
    let s = format!("{:+x}", 1234);
    println!("s = {}", s);
    let s = format!("{:+#x}", 1234);
    println!("s = {}", s);
    let s = format!("{:b}", 1234);
    println!("s = {}", s);
    let s = format!("{:#b}", 1234);
    println!("s = {}", s);
    let s = format!("{:#20b}", 1234);
    println!("s = {}", s);
    let s = format!("{:<#20b}", 1234);
    println!("s = {}", s);
    let s = format!("{:^#20b}", 1234);
    println!("s = {}", s);
    let s = format!("{:>#15x}", 1234);
    println!("s = {}", s);
    let s = format!("{:>+015x}", 1234);
    println!("s = {}", s);
}

/// 关于浮点数，一个格式化代码
///
///  浮点数格式化主要注意以下两点
/// - 指定小数点后的有效位，符号".“代表的是指定浮点数小数点后的有效位，
/// 这是需要注意的是，在指定有效位是会四舍五入，
/// - 科学计数法，使用{:e}可以将浮点数格式化为科学技术法的表示形式
#[test]
fn test_float_format() {
    let s = format!("{:.4}", 1234.5678);
    println!("s = {}", s);
    let s = format!("{:.2}", 1234.5618);
    println!("s = {}", s);
    let s = format!("{:.2}", 1234.5678);
    println!("s = {}", s);
    let s = format!("{:<10.4}", 1234.5678);
    println!("s = {}", s);
    let s = format!("{:^12.2}", 1234.5678);
    println!("s = {}", s);
    let s = format!("{:0^12.2}", 1234.5678);
    println!("s = {}", s);
    let s = format!("{:e}", 1234.5678);
    println!("s = {}", s);
}

/// 以上所有的格式化规则，对于println!和write!宏均适用，
/// 前面展示的都是字符串，整数和浮点数等内置类型的格式化，如果要对于自定义类型格式化，
/// 则需要事先Display triat
///
#[test]
fn test_self_define_type() {
    use std::fmt::{self, Display, Formatter};
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(
                f,
                "{}: {:.3}{} {:.3}{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    let city = City {
        name: "Beijing",
        lat: 39.90469,
        lon: -116.40717,
    };
    println!("city = {}", city);
}

/// 8.1.8 回顾
///
/// 问题是：有一个数字方阵，求出对角线位置的所有数字之和
///
/// 使用原生字符串声明语法r"...", 将此数字方阵定义为为字符串
/// 然后按行遍历其字符即可得到结果
///
#[test]
fn test_sum() {
    let s = r"1234
                5678
                9876
                4321";
    let (mut x, mut y) = (0, 0);
    for (idx, val) in s.lines().enumerate() {
        let val = val.trim();
        let left = val.get(idx..idx + 1).unwrap().parse::<u32>().unwrap();
        let right = val
            .get((3 - idx)..(3 - idx + 1))
            .unwrap()
            .parse::<u32>()
            .unwrap();
        x += left;
        y += right;
    }

    assert_eq!(38, x + y);
}

/// 8.2 集合类型
///
/// Rust标准库中提供的集合类型包括以下几种:
///
/// - Vec<T>， 动态可增长数组
/// - VecDeque<T>, 基于环形缓冲区的先进先出双端队列实现
/// - LinkedList<T>, 双向链表实现
/// - BinaryHeap<T>, 二叉堆(最大堆)实现，可用作优先队列
/// - BTreeMapK<K, V>, 基于B树的有序映射集实现，按Key排序
/// - HashSet<T>, 无序集合实现
/// - BTreeSet<T>, 基于B树的有序集合实现
///
/// 以上最常用的集合类型为Vec<T>和HashMap<K, V>
///
/// 8.2.1 动态可增长数组
///
///
/// Rust中数组有两种类型：
///
/// - 原生类型array，拥有固定的长度，类型签名[T; N]
/// array中的元素可以在栈上存储
/// - 动态可增长数组Vector, 是可增长的动态数组，类型签名Vec<T>，在运行时才可知道大小
/// vector中的元素只能在堆上分配
///
///
/// Vector中的一下操作
///
/// 基本操作与内存分配
///
/// 创建Vector和创建String类型字符串的方法很类似，
/// 因为String类型的字符串本省就是对Vec<u8>类型的包装
///
///
#[test]
fn test_basic_vector_operator() {
    // fn new() -> Self
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    // fn len() -> usize
    assert_eq!(vec.len(), 2);
    // 索引， 但是一般不推荐使用索引， 而是使用get， get_mut
    assert_eq!(vec[0], 1);
    // fn pop() -> Option<T>
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);
    assert_eq!(vec.get(0), Some(&7));
    assert_eq!(vec.get(10), None);
    // vec[10]; // error
    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec, [7, 1, 2, 3]);

    assert_eq!(vec.get(0..2), Some(&[7, 1][..]));
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec2, []);
    // let mut vec = vec![1, 2, 3];
    // let mut vec2 = vec![4, 5, 6];
    // vec.append(&mut vec2);
    // assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    // assert_eq!(vec2, []);
    // fn swap()
    vec.swap(1, 3);
    assert_eq!(vec, [7, 3, 2, 1, 4, 5, 6]);
    let slice = [1, 2, 3, 4, 5, 6, 7];
    // copy_from_slice
    vec.copy_from_slice(&slice);
    // println!("vec = {:?}", vec);
    // println!("slice = {:?}", slice);
    assert_eq!(vec, slice);
    let mut vec = vec![1, 2, 3, 4];
    let slice = [4, 3, 2, 1];
    // clone_from_slice
    vec.clone_from_slice(&slice);
    println!("vec = {:?}", vec);
    println!("slice = {:?}", slice);
    assert_eq!(vec, slice);
    let src = [1, 2, 3, 4];
    let mut dst = [0, 0];
    // clone_from_slice
    dst.clone_from_slice(&src[2..]);

    assert_eq!(src, [1, 2, 3, 4]);
    assert_eq!(dst, [3, 4]);
}

/// with_capacity 预分配内存的方式来创建Vector数组
///
/// 使用Vec::with_capacity方法和String::with_capacity方法类型，
/// 可以预分配堆内存
/// 实际上真正分配的堆内存大小等于数组中元素类型所占字节与给定容量值之积；
///
/// 使用shrink_to_fit方法，预分配的堆内存被释放了，
/// 实际上，该方法只有在vec数组中元素被清空之后才会被释放预分配的堆内存
/// ，当vec数组中元素并为占满容量空间时，就会压缩未被使用的那部分空间，
/// 相当于重新分配堆空间。
///
/// 在日常编程中，使用Vec::with_capacity方法来创建
/// vector数组可以有效地避免频繁申请堆内存带来的性能损耗、
///
#[test]
fn test_with_capacity() {
    let mut vec = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i);
    }
    vec.truncate(0);
    assert_eq!(10, vec.capacity());
    for i in 0..10 {
        vec.push(i);
    }
    vec.clear();
    assert_eq!(10, vec.capacity());
    vec.shrink_to_fit();
    assert_eq!(0, vec.capacity());
    for i in 0..10 {
        vec.push(i);
        //output : 4/4/4/4/8/8/8/8/16/16/
        print!("{:?}/", vec.capacity());
    }
}

/// 对于不占字节的，属于零大小类型ZST，那么他是怎么存储的呢？
/// 对于使用Vec::new方法创建的空数组，如果没有分配堆内存，
/// 那么他的指针指向哪里？
///
///
/// 因为此时并为预分配内存，所以其内部指针并非
/// 指向堆内存，但它也不是空指针，Rust在这里做了空指针优化。
///
#[test]
fn test_zero_type() {
    struct Foo;
    let mut vec = Vec::new();
    vec.push(Foo);
    assert_eq!(vec.capacity(), std::usize::MAX);
}

/// 查找与排序
///
/// 数组也支持字符串中提供的一些查找方法，比如contains,
/// start_with, ends_with方法
///
#[test]
fn test_search_sort() {
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));
    // 注意的是starts_with, ends_with, 传入的是切片
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(v.ends_with(&[]));
    // 支持对空数组做查找start_with, ends_with
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));
    assert!(v.ends_with(&[]));
}

/// binary_search系列泛型方法
///
#[test]
fn test_binary_search() {
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));
    let r = s.binary_search(&1);
    assert!(match r {
        Ok(1..=4) => true,
        _ => false,
    });
    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));

    let s = [
        (0, 0),
        (2, 1),
        (4, 1),
        (5, 1),
        (3, 1),
        (1, 2),
        (2, 3),
        (4, 5),
        (5, 8),
        (3, 13),
        (1, 21),
        (2, 34),
        (4, 55),
    ];
    assert_eq!(s.binary_search_by_key(&13, |&(_a, b)| b), Ok(9));
}

/// Sort系列方法和Sort_unstable系列方法
/// 
/// sort， sort_by和sort_by_key方法，其内部所用算法为
/// 自适应迭代归并排序(Adaptive/iterative Merge Sort)算法
/// 灵感来自Python语言中的TimSort算法 该算法为稳定排序算法，
/// 即序列中等价的元素在排序之后相对位置并不改变。
/// 其时间复杂度为O(n), 最坏时间复杂度O(nlogn)
/// 
/// sort系列方法均可被直接替换为sort_unsatble, sort_unstable_by
/// 和sort_unstable_by_key方法，
/// 但是sort_unstable系列方法其内部实现的排序算法为模式消除快速
/// 排序(patten-defeating QuickSort)算法， 该算法为不稳定排序算法，
/// 也就是说，序列中等价的元素在排序之后相对位置有可能发生变化，
/// 其时间复杂度O(n), 最坏情况为O(nlogn)， 在不考虑稳定性的情况下，推荐使用
/// sort_unsatable系列方法，其性能要高于sort系列方法，
/// 因为他们不会占用额外的内存
/// 
/// 不管是sort系列方法还是sort_unstable系列方法，其命名规则和binary_search系列方法相似的
/// 所以他们在语义上也是相同的，
/// xxx_by方法表示接收返回Ordering类型的闭包参数，
/// xxx_by_key方法接收的闭包参数覆盖范围更广，适合表示任意检索(排序)条件。
/// 
#[test]
fn test_sorted() {
    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort();
    println!("v = {:?}", v);
    v.sort_by(|a, b| a.cmp(b));
    println!("v = {:?}", v);
    v.sort_by(|a, b| b.cmp(a));
    println!("v = {:?}", v);
    // let mut v = [5, 4, 1, 3, 2];
    // v.sort_by(|a, b| a.cmp(b));
    // assert!(v == [1, 2, 3, 4, 5]);

    // // reverse sorting
    // v.sort_by(|a, b| b.cmp(a));
    // assert!(v == [5, 4, 3, 2, 1]);
    v.sort_by_key(|k| k.abs());
    println!("v = {:?}", v);
}

/// 与排序和比较相关的triat
/// 
/// 在Rust中把比较操作也抽象为一些triat吗定义在std::cmd模块中。
/// 该模块中定义的trait是基于数学集合论中的二元关系偏序、全序和等价的。
/// 
/// 偏序的定义，对于非空集合中的a, b, c来说，满足下面条件为偏序关系。
/// 
/// - 自反性: a <= a
/// - 反对称性: 如果a<=b且b<= a, 则a=b.
/// - 传递性: 如果a <=b 且 b<=c,则a<= c.
/// 
/// 全序的定义，对于非空集合中的a, b, c来说，满足下面条件的为全序关系。
/// 
/// - 反对称性: 若a<=b且b<=a, 则a==b
/// - 传递性: 如果a <=b 且 b<=c,则a<= c.
/// - 完全性: a < b, b < a, 或a== b必须满足其一， 表示任何与元素都可以相互比较。
/// 
/// 全序实际上是一种特殊的偏序。
/// 
/// 等价的定义，对于非空集合中的a, b, c, 来说，满足下面条件为等价关系。
/// 
/// - 自反性： a == b
/// - 对称性: a == b, 意味着b == a
/// - 传递性: 若a == b且b == c, 则a == c
/// 
/// 在Rust中一共涉及四个triat和一个枚举体来表示上述二元关系。
/// 四个triat分别是PartialEq, Eq, PartialOrd, Ord, 这些triat
/// 的关系可以总结为以下几点： 
/// - PartialEq代表部分等价关系，其中定义了eq, ne两个方法，分别 表示"==", "!="操作
/// - Eq代表等价关系，该triat继承自ParialEq, 但是其中没有任何方法，
/// 它实际上相当于标记实现了Eq的类型拥有等价关系。
/// - PartialOrd对应于偏序，其中定义了partial_cmp， lt, le, gt, ge五个方法。
/// - Ord对应于全序， 其中定义了cmp, max, min三个方法
/// 
/// 还有一个枚举体为Ordering, 用于表示比较结果，其中定义了
/// 小于、等于和大于三种状态。
/// 
/// PartialEq triat
/// 
/// #[lang = "eq"]
/// pub triat ParialEq<Rhs = Self> 
/// where Rhs: ?Sized, 
/// {
///     fn eq(&self, other: &Rhs) -> bool;
///     // ne 方法定义了默认的实现
///     fn ne(&self, other: &Rhs) -> bool { ... }
/// }
/// // Eq实际上起标记的作用，没有任何实际的方法
/// pub triat Eq: PartialEq<Self> {}
///
///  PrtialOrd trait
/// 
/// pub triat PartialOrd<Rhs = Self>: PartialEq<Rhs> 
/// where Rhs: ?Sized,
/// {
///     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>,
///     fn lt(&self, other: &Rhs) -> bool {...}
///     fn le(&self, other: &Rhs) -> bool { ...}
///     fn gt(&self, other: &Rhs) -> bool {...}
///     fn ge(&self, other: &Rhs) -> bool { ...}
/// }
/// 
/// pub enum Ordering {
///     Less,
///     Equal, 
///     Greater,
/// }
/// 
/// partial_cmp 方法表示具体的比较规则，注意其返回Option<Ordering>类型，
/// 因为对于偏序的比较来说，并不是所有元素都具有可比性，有些元素的比较结果
/// 可能为None。
/// 如果要给某个类型实现PartialOrd,只需要实现partial_cmp方法
/// 
/// Ord定义
/// 
/// pub trait Ord: Eq + PartialOrd<Seld> {
///     fn cmp(&self, other: &Self) -> Ordering,
///     fn max(self, other: Self) -> Self { ... }
///     fn min(self, other: Self) -> Self { ... }
/// }
/// 
/// Ord继承自Eq, 和ParialOrd，这是因为全序的比较必须满足三个条件;
/// 反对称性，传递性和完全性，其中完全性一定是每个元素都可以相互比较。
/// 例如： 在浮点数中，用于定义特殊情况值而使用的NaN, 其本身就不可比较，
/// 因为NaN != NaN, 它不满足全序的完全性，所以浮点数只能实现PartialEq, PartialOrd, 
/// 而不能实现Ord, 如果要实现Ord， 只需要实现cmp方法即可，因为max和min都有默认实现。
/// 注意cmp方法返回Ordering类型，而不是Option<Ordering>类型，因为对于
/// 全序关系来说，每个元素都是可以获得合法的比较结果的。
/// 
/// 在Rust中基本的原生数字类型和字符串均都实现了上述的trait
///  
/// 
/// 如果要在自定义类型中实现相关triat，则必须搞清楚全序和偏序关系，然后
/// 在实现相应的trait， 可以手工实现，可以使用#[derve]来自动派生。
#[test]
fn test_partical_cmp() {
    use std::cmp::Ordering;
    let result = 1.0.partial_cmp(&2.0);
    assert_eq!(result, Some(Ordering::Less));
    let result = 1.cmp(&1);
    assert_eq!(result, Ordering::Equal);
    let result = "abc".partial_cmp(&"ABC");
    assert_eq!(result, Some(Ordering::Greater));
    let mut v : [f32; 5] =[5.0, 4.1, 1.2, 3.4, 2.5];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(v, [1.2, 2.5, 3.4, 4.1, 5.0]);
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert_eq!(v, [5.0, 4.1, 3.4, 2.5, 1.2]);

}

/// 回顾与展望
/// 
/// 本机虽然重点介绍的是Vector，但是里面涉及的方法同样适用于array,因为在这些方法实际上是为[T]
/// 类型实现的。
/// 
/// pub trait SliceExt {
///     type Item;
///     ...
///     fn split<P>(&self, pred: P) -> Split<Seld::Item, P>
///         where P: FnMut(&Self::Item) -> bool;
///     ...
///     fn binary_search(&self, x: &Self::Item) -> Result<usize, usize>
///         where Self::Item: Ord;
///     fn len(&self) -> usize;
///     fn is_empty(&self) -> bool { self.len() == 0} 
///     fn iter_mut(&mut self) -> IterMut<Self::Item>;
///     fn swap(&mut self, a: usize, b: usize);
///     fn contains(&self, x: &Self::Item) -> bool
///         where Self::Item: PartialEq;
///     fn clone_from_slice(&mut self, src: &[Self::Item])
///         where Self::Item : Clone;
///     fn copy_from_slice(&mut self, src: &[Self::Item])
///         where Self::Item : Copy;
///     fn sort_unstable(&mut self) where Self::Item : Ord;
/// }
/// 
/// impl <T> SliceExt for [T] {
///     type Item = T;
///     ...
/// }
/// 
/// 在Rust2018中，加入了针对array数组和切片进行match匹配的新语法
/// 
/// match 匹配array数组示例
/// 
#[test]
fn test_match () {
    fn pick(arr: [i32; 3]) {
        match arr {
            [_, _, 3] => println!("ends with 3"),
            [a, 2, c] => println!("{:?}, 2, {:?}", a, c),
            [_, _, _] => println!("pass!"),
        }
    }

    let arr = [1, 2, 3];
    pick(arr);
    let arr = [1, 2, 5];
    pick(arr);
    let arr = [1, 3, 5];
    pick(arr);
}

/// 当前Rust只是用array数组局限性比较大，不过该语法
/// 还支持数组切片，所以利用数组切片就可以模拟变长参数的函数
/// 
#[test]
fn test_match_array_slice() {
    fn sum(num: &[i32]) {
        match num {
            [_one] => println!("at least two"),
            [first, second] => {
                println!("{:?} + {:?}  = {:?}", first, second , first +second );
            },
             _ => println!(
                "sum is {:?}", num.iter().fold(0, |sum, i| sum + i)
             ),
        }
    }
    sum(&[1]);
    sum(&[1, 2]);
    sum(&[1, 2, 3]);
    sum(&[1, 2, 3, 5]);
}

/// 8.2.2 映射集
/// 
/// 在日常编程中，另一个常用的数据结构非 映射集(Map)莫属。
/// Map是依照键值对(Key-Value)形式存储的数据结构，每个键值对
/// 都被称为一个Entry， 在Map中不能存在重复的Key，并且每个Key必须有一个一一对应的值。
/// Map提供的查找，插入和删除操作的时间复杂度基本都是O(1), 最坏情况下是O(n), 
/// 虽然需要消耗额外的空间，但是随着当下可利用的内存越来越多，这种空间
/// 换时间的做法呢也是值得的。
/// 
/// Rust提供了两种类型的Map： 基于哈希表(HashTable)的HashMap和基于
/// 多路平衡查找树(B-Tree)的BTreeMap,
/// 
/// HashMap的增、删、改、查
/// 
/// 通过keys和values方法可以分别单独获取HashMap中的键和值
/// ，注意的是这两个方法是迭代器，因为HashMap是无序的映射表，所以在迭代键和值的时候，
/// 输出的顺序并不一定和插入的顺序相同。
/// 
/// 通过index语法可以按指定的键来获取对应的值，这里需要注意的是
/// 目前Rust支持inde, 而不支持indexMut， 也就是说，只可以通过
/// hash[key]方法来取值，而不能通过Hash[key] = value方法来插入键值对，
/// 这是因为针对该特性正在准备一个更好的设计方案，并在不远的将来得到支持。
/// 
#[test]
fn test_hash_crud() {
    use std::collections::HashMap;
    let mut book_reviews = HashMap::with_capacity(10);
    book_reviews.insert("Rust Book", "good");
    book_reviews.insert("Porgramming Rust", "nice");
    book_reviews.insert("The Tao of Rust", "deep");
    for key in book_reviews.keys() {
        println!("{}", key);
    }
    for val in book_reviews.values() {
        println!("{}", val);
    }

    if !book_reviews.contains_key("Rust book") {
        println!("find {} times", book_reviews.len());
    }

    book_reviews.remove("Rust Book");
    let to_find = ["Rust Book", "The Tao of Rust"];
    for book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{} : {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }

    for (book, review) in &book_reviews {
        println!("{} : \" {}\"", book, review);
    }
    assert_eq!(book_reviews["The Tao of Rust"], "deep");
    
}


/// Entry 模式
/// 
/// 对于HashMap中的单个桶(Bucket)来说，其状态无非是”空“和”满“
/// 所以Rust对此做了一层抽象，使用Entry枚举体来表示每个键值对。
/// 
/// pub enum Entry<'a, K: 'a, V: 'a> {
///     Occupied(OccupiedEntry<'a, K, V>),
///     Vacant(VacantEntry<'a, K, V>),
/// }
/// 
/// 其中包含了两个变体: Occupied(OccupiedEntry<'a, K, V>)和
/// Vacant(Vancant<'a, K, V>). 
/// 
/// OccupiedEntry<'a, K, V>和VacantEntry<'a, K, V>是内部定义的两个结构体，
/// 分别对应HahsMap底层桶的存储信息，其中Occupied代表占用，
/// Vacant代表留空。
/// 
/// Entry一共实现了三个方法，通过这三个方法可以方便地对HashMap中的键值对进行操作
/// 
/// 将键，传入entry方法之后，首先会去判断哈希表是否有足够的空间，
/// 如果没有，则会进行自动扩容。
/// 接下来调用内部的hash函数生成此键的hash值，然后通过这个
/// hash值在底层的哈希表中搜索，如果能找到此键呢，则返回相应的桶(Occupied)
/// 如果找不到，则返回空桶(Vacant), 最后，将得到的桶转换为Entry<K, V>
/// 并返回。
/// 
/// 在得到Entry之后，就可以调用其实现的or_insert方法，该方法的参数就是
/// 要插入的值，并返回该值的可变借用
/// 
/// // pub fn or_insert(self, default: V) -> &'a mut V
/// 
/// 使用or_insert_with方法可以传递一个可计算的闭包作为要插入的值
/// 注意的，其只允许FnOnce() -> V的闭包，也就说，闭包不能包含参数
///  
/// or_insert方法的源码
/// 
/// pub fn or_insert(self, default: V) -> &'a mut V {
///     match self {
///         Occupied(entry) => entry.into_mut(),
///         Vacant(entry) => entry.insert(default),
///     }
/// }
/// 
/// 通过entry方法从底层找到相应的桶之后，再通过match方法
/// 分别处理不同类型的桶。如果是占用的桶
/// Occupied(entry), 则通过into_mut方法将其变为可变借用，
/// 这样就可以被新插入的键值对覆盖。
/// 如果是空桶(Vancant(entry))， 则使用相应的insert方法直接插入，
/// 注意的是此时的insert方法为VancantEntry定义的insert方法
/// 
#[test]
fn test_entry() {
    use std::collections::HashMap;
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("current_year").or_insert(2017);
    
    assert_eq!(map["current_year"], 2017);
    *map.entry("current_year").or_insert(2017) += 10;
    assert_eq!(map["current_year"], 2027);
    let last_leap_year = 2016;
    map.entry("next_leap_year")
        .or_insert_with(|| last_leap_year + 4);
    assert_eq!(map["next_leap_year"], 2020);
    assert_eq!(map.entry("current_year").key(), &"current_year");
}