use std::convert::TryInto;

/// # 类型转换
///
/// Rust使用triat解决类型之间的转换问题，一般转换会用到，From和into两个triat。
///
/// ## From和Into
///
/// 如果能把类型A转换成类型B，那么狠容易相信也能把类型B转换成类型A
///
/// ## From
///
/// From trait允许一种类型定义"怎么根据另一种类型生成自己"，因此它提供了一种类型转换的简单机制。
/// 在标准库中有无数from的实现，规定了原生类型及其他常见类型的转换功能。
///
pub fn from_type_cast() {
    let my_str = "hello";
    let _my_string = String::from(my_str);

    // use std::convert::From;

    #[derive(Debug)]
    struct  Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);
}

/// # Into
///
/// Into trait就是把From triat倒过来而已， 也就是说，如果你为你的类型实现了From，
/// 那么同时你也就免费获得了Into
///
/// 使用Into trait通常需要指明要转换到的类型，因为编译器大多数的时候不能推断它，不过
/// 考虑到我们免费获得了Into，这个代码就不值一提了。
///
///
pub fn into_type_cast() {
    // use std::convert::From;

    #[derive(Debug)]
    struct  Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    // let num = Number::from(30);
    let int = 5;
    let num : Number= int.into();
    println!("My number is {:?}", num);
}

/// # TryFrom And TryInto
///
///
/// TryFrom , TryInto trait用于易出错的转换， 返回值是Result
pub fn tryfrom_cast_type() {
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error>{
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // tryfrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // tryinto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

}

/// # ToString 和 FromStr
///
/// tostring
/// 要把任何类型转换成String， 只需要实现那个类型的ToString triat。然而不要直接这么做，
/// 你应该实现fmt::Display trait, 他会自动提供 ToString, 并且还可以用来打印类型，就像
/// print!一节中讨论的那样
///
///
pub fn tostring_cast_type() {

    struct Circle {
        radius: i32,
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

}

/// # 解析字符串
/// 我们经常需要把字符串转成数字，完成这项工作的标准手段使用parse函数，我们得
/// 提供要转换到的类型，这可以通过不使用类型推断，或者使用"turbofish<>)实现。
/// 只要对目标类型实现了FromStr triat，就可以用parse把字符串转换成目标类型。
/// 标准库中已经给如数种类实现了FromStr. 如果要转换到用户定义类型，只要手动实现
/// FromStr就行
pub fn parse_string_example() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("sum : {}", sum);
}