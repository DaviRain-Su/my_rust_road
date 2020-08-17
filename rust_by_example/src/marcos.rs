/// # 宏
///
///
/// 宏是通过macro_rules!来创建的
///
/// 宏有什么用呢？
/// - 不写重复的代码
/// - 领域专用语言DSL， 宏允许你为特定的目的创造特定的语法
/// - 可变接口， 接受不定数目参数的接口
///
pub fn define_macros() {

    //定义宏
    macro_rules! say_hello {
    // '()'表示此宏不接受任何参数
        () => (
        //此宏将展开成这里面的东西
            println!("Hello!");
        )
    }

    say_hello!();
}

/// # 宏语法
///
/// ## 模式与指示符
///
/// ### 指示符
///
/// 宏的指示符使用一个美元符号$作为前缀， 并使用一个指示符来注明类型
///
/// 全部的指示符
/// - block
/// - expr 表达式
/// - ident 用于变量名或函数名
/// - item
/// - pat 模式patten
/// - stmt 语句 statement
/// - tt 标记树 token tree
/// - ty 类型 type
///
pub fn designator_example() {

    macro_rules! create_function {
        //此宏接受一个 'ident' 指示符表示的参数，并创建一个名为 '$func_name'的函数
        // 'ident' 指示符用于变量名或函数名
        ($func_name:ident) => (
            fn $func_name() {
                // stringify！宏把 'ident' 转换成字符串
                println!("You called {:?}()", stringify!($func_name))
            }
        )
    }

    create_function!(foo);
    create_function!(bar);


    macro_rules! print_result {
        //此宏接受一个 expr 类型的表达式，并将他作为字符串，连同其结果一起打印出来
        // 'expr' 指示符表示表达式
        ($expression:expr) => (
            //stringify 把表达式原样转成一个字符串
            println!("{:?} = {:?}", stringify!($expression), $expression)
        )
    }

    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

/// ## 重载
///
/// 宏可以重载， 从而接受不同的参数组合，
///
pub fn overload_macro() {
    //根据调用它的方式，test！将以不同的方式来比较 $left 和 $right
    macro_rules! test {
        // 参数不需要使用逗号隔开,参数可以任意组合！
        ($left:expr; and $right: expr) => (
            println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right)
        );
        //每个分支都必须以分号结尾
        ($left: expr; or $right: expr) => (
            println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right)
        );
    }
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
/// ## 重复
///
/// 宏的参数列表中可以使用+来表示一个参数可能出现一次或者多次，使用*来表示该参数可能出现零次或多次
/// 把模式这样： $(...),+, 包围起来，就可以匹配一个或多个逗号隔开的表达式。
/// 另外，宏定义的最后一个分支可以不同分号作为结束
///
///
pub fn repeat_macro() {
    //min!将求出任意数量的参数的最小值
    macro_rules! find_min {
        //basic
        ($x:expr) => ($x);
        // $x, 后面跟着至少一个$y
        ($x:expr, $($y:expr),+) => (
            std::cmp::min($x, find_min!($($y), +))
        )
    }

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(3, 2, 5, 0));
    println!("{}", find_min!(1+2, 0, 4));
}

/// # DRY不写重复代码
///
/// 通过提取函数或测试集的公共部分，宏可以让你写出DRY的代码
pub fn donnot_repeat_yourself() {
    // use std::ops::{Add,  Mul, Sub};
    //
    // macro_rules! assert_equal_len {
    //     // tt (token tree)标记树，指示符表示运算符和标记
    //     ($a: ident, $b: ident, $func:ident, $op:tt) => (
    //         assert!($a.len() == $b.len(), "{:?}: dimension mismatch: {:?} {:?} {:?}",
    //          stringify!($func),
    //          ($a.len(),),
    //          stringify!($op),
    //          ($b.len(),)
    //          );
    //     )
    // }

    // macro_rules! op {
    //     ($func: ident, $bound: ident, $op: tt, $method: ident) => (
    //         fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
    //             assert_equal_len!(xs, ys, $func, $op);
    //
    //             for (x, y) in xs.iter_mut().zip(ys.iter()) {
    //                 *x = $bound::$method(*x, *y);
    //             }
    //         }
    //     )
    // }

    // op!(add_assign, Add, +=, add);
    // op!(mul_assign, Mul, *=, mul);
    // op!(sub_assign, Sub, -=, sub);

// mod test {
//     use std::iter;
//     macro_rules! test {
//         ($func: ident, $x: expr, $y: expr, $z: expr) => (
//             #[test]
//             fn $func() {
//                 for size in 0usize..10 {
//                     let mut x: Vec<_> = iter::repeat($x).take(size).collect();
//                     let y: Vec<_> = iter::repeat($y).take(size).collect();
//                     let z: Vec<_> = iter::repeat($z).take(size).collect();
//
//                     super::$func(&mut x, &y);
//                     assert_eq!(x, z);
//
//                  }
//             }
//         )
//     }
//
//     test!(add_assign, 1, 2, 3);
//     test!(mul_assign, 2, 3, 6);
//     test!(sub_assign, 3, 2, 1);
// }

}

/// # DSL 领域专用语言
/// DSL 是Rust的宏中集成的微型"语言"。这种语言完全是合法的，因为宏系统会把它转成普通的Rust语法树，他只不过看起来像是另一种语言而已。
/// 这就允许你为一些特定功能创造一套简单直观的语法（当然是有限制的）
pub fn dsl_programming() {
    macro_rules! calculate {
        (eval $e: expr) => (
            let val : usize = $e; //强制类型为整型
            println!("{} = {}", stringify!($e), val);
        );
    }

    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval ( 1 + 2) * ( 3 / 4)
    }
}

/// # 可变参数接口
///
/// 可变参数接口可以接受任意数目的参数，比如， println!就可以，其参数的数目是由格式化字符串指定的
pub fn option_change_para() {
    macro_rules! calculate {
        (eval $e: expr) => (
            let val : usize = $e; //强制类型为整型
            println!("{} = {}", stringify!($e), val);
        );

        (eval $e: expr, $(eval $es: expr),+) => (
            calculate!{ eval $e }
            calculate!{ $(eval $es),+ }
        );
    }

    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3 ) + 1
    }
}