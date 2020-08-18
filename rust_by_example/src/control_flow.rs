/// # if/else
///
/// Rust语言中的布尔判断条件不必用小括号包住，且每个条件后面都跟着一个代码块。
///
/// if-else条件选择是一个表达式，并且所有分支都必须返回相同的类型
///
///
pub fn if_else_example() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if -10 < n && n < 10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is big number, half the number");
            n / 2
        };

    println!("{} - > {}", n, big_n);
}

/// # loop
///
///
/// loop关键字来实现一个无限循环
///
/// 可以使用break语句在任何时候退出一个循环，
/// 另外可以用continue跳过循环体的剩余部分并开始下一轮循环
///
///
pub fn loop_flow_example() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }
}

/// # 嵌套循环和标签
///
/// 在处理嵌套循环的时候可以break或continue外层循环。在这类情形中，循环必须用一些'lable 标签来标注，
/// 并且标签必须传递给break/continue语句
///
pub fn loop_lable_example() {
    #![allow(unreachable_code)]

    'outer: loop {
        println!("Entered the outer loop");

        // 'inner: loop {
        loop {
            println!("Entered the inner loop");

            // break; //这只是中断内部的循环

            break 'outer; //中断外层循环
        }

        println!("This point will never be reached");

    }

    println!("Exited the outer loop");
}


/// # 从loop 循环中返回
///
/// loop 有个用途是尝试一个操作直到成功为止。若操作返回一个值，则可能需要将其传递给代码的其余
/// 部分： 将该值放在break之后，他就会被loop表达式返回
///
///
///
pub fn from_loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("result is {}",result);
}

/// # while 循环
///
///
/// while关键字可以用作当条件满足时循环
///
///
pub fn while_flow_example() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

/// # for循环
///
///
///
/// # for 与区间
///
/// for in 结构可以遍历一个Iterator。创建迭代器的一个简单的方法是使用区间标记
/// a..b ,生葱从a包含此值，到b不含此值，步长为1
/// a..=b 表示两端都包含在内的范围
///
pub fn for_flow_example() {

    //a..b
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // a..=b
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

/// # for与迭代器
///
/// for i结构体能以几种方式与iterator互动。在迭代器triat一节中将会谈到。如果没有特别
/// 指定，for循环会对给出的集合应用into_iter函数，把它转换成一个迭代器，
///
/// 三个函数会以不同的方式返回集合中的数据
/// - iter， 在每次迭代中借用集合中的一个元素，这样集合本身不会被改变，循环之后仍可以使用
/// - into_iter, 会消耗集合，在每次迭代中，集合中的数据本身会被提供，一旦集合被消耗了，之后就无法再使用了，因为他
/// 已经在循环中被移除了
/// - iter_mut， 可变的借用集合中的每个元素，从而允许集合被就地修改
///
pub fn for_in_iterator_three_way() {

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name  {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name  {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

/// # match 匹配
///
///
/// Rust通过match关键字来提供模式匹配，用法和C语言的swit类型
///
///
pub fn match_control_flow() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This ia a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

/// # 解构
///
///
/// # 元组
///
///
pub fn tuple_match_example() {
    let pair = (0, -2);

    println!("Tell me about: {:?}", pair);

    match pair {
        (0, y) => println!("First is 0, and y is {:?}", y),
        (x, 0) => println!("x is {:?} and last is 0", x),
        _ => println!("It doesn't matter what they are"),
    }
}
/// # 枚举
pub fn enum_match_example() {
    #[allow(dead_code)]
    enum  Color {
        Red,
        Blue,
        Green,

        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(127, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green:{}, and blue : {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c,m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}， key (black) : {}!", c, m, y, k),
    }
}

/// # 指针和引用
///
/// 对指针来说，结构和解引用要区分开，因为这两者的概念是不同的，和C的用法是不一样的
///
/// - 解引用使用*
/// - 解构使用&, ref, ref mut
///
pub fn pointer_reference_example() {
    let reference = &4;

    match reference {
        // 如果用&val这个模式去匹配reference，就相当于做这样的比较&i32 -- &val
        //可以看到，如果去掉匹配&， i32应当赋给 val
        // 因此可用val 表示被reference 引用的值4
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果不想用&， 需要在匹配前解引用
    match *reference {
        val => println!("Got a value via dereferencing : {:?}", val),
    }

    // 如果一开始就不用引用，会怎么样呢？ reference是一个&类型，因为赋值语句
    // 的右边已经是一个引用，但下面这个不是引用，因为右边不是
    let _not_a_reference = 3;

    // Rust对这种情况提供了 ref, 它更改了赋值行为，从而可以对具体值创建引用
    //下面得到一个引用
    let ref _is_a_reference = 3;

    // 定义的两个非引用的变量，通过ref, ref mut, 仍可取得其引用
    let value = 5;
    let mut mut_value = 6;

    // 使用ref关键字来创建引用
    // 下面的r是&i32,类型，他像i32一样可以直接打印，因此似乎用法上似乎看不出什么区别，但是
    // 可以把println!中的r 改成*r，仍然可以正常运行，
    match value {
        ref r => println!("Got a reference to a value: {}", r),
    }


    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. mut_value : {:?}", m);
        },
    }
}

/// #结构体
///
///
pub fn destruct_struct_example() {
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo{ x: (1, 2), y: 3};
    let Foo { x: (a, b), y} = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);


    let Foo { y: i, x: j} = foo;
    println!("i = {:?}, j = {:?}", i, j);

    let Foo { y, ..} = foo;
    println!("y = {}", y);

    // let Foo { y } = foo;
}


/// # 守卫
///
/// match守卫
///
///
pub fn match_guard_example() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is old"),
        _ => println!("No correlation..."),
    }

}

/// # 绑定
///
/// 在match 中，若间接地访问一个变量，则不经过重新板顶就无法再分支中在使用它，
/// match中提供了@ 符号来绑定变量到名称
pub fn match_bounding_example() {
    fn age() -> u32 { 15 }

    println!("Tell me type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1 ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 .. 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    // 可以使用绑定解构 enum 变体
    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        // got some variant, match if its value , bound to n
        Some(n @ 42) => println!("The Answer: {}!", n),
        // match any other number
        Some(n) => println!("Not interesting ... {}", n),
        // match anything else (none)
        _ => (),
    }
}

/// # if let
///
/// 在一些场合下，用match匹配枚举类型并不优雅
///
///
pub fn lf_let_example() {
    let option = Some(7);
    match option {
        Some(i) => {
            println!("This is a really long string and {:?}", i);

        },
        _ => {},
    };

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :) !");
    };

    enum  Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    } else if Foo::Bar = b {
        println!("b is foobar");
    } else if Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // if let 允许匹配枚举非参数化的变量，即枚举未注明
    // #[derive(PartialEq)],我们也没有为其实现PartialEq,
    // 在这种情况下，通常 if Foo::Bar == a会出错，因为此枚举的实例不具有可比性。
    // 但是if let是可行的

    enum Boo { Bar }

    let a = Boo::Bar;

    if let Boo::Bar = a {
        println!("a is BooBar");
    }
}

/// # while let
///
///
pub fn while_let_example() {
    let mut optional = Some(0);

    loop {
        match  optional {
            Some(i) => {
                if 9 < i {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!(" i is {:?}, Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => {
                break;
            }
        }
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if 9 < i {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!(" i is {:?}, Try again.", i);
            optional = Some(i + 1);
        }
    }// if let 有可选的， else / else if 分句
    // whil let 没有
}