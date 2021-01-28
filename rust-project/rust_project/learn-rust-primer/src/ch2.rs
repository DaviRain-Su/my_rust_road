//! 基本程序结构

/// 条件分支
/// if
/// if let
/// match
/// ```no_run
/// // 形式1
/// if expr1 {
///
/// }
/// //形式2
/// if expr2 {
///
/// } else {
///
/// }
///
/// // 形式3
/// if expr1 {
///
/// } else if expr2 {
///
/// } else {
///
/// }
///
/// ```
///
/// Rust 表达式的特点
/// 1. 判断条件不用小括号括起来
/// 2. 他是表达式，而不是语句
///
#[test]
fn test_if() {
    let x = 5;
    println!("x = {}", x);
    let y = if x == 5 { 10 } else { 15 };
    println!("y = {}", y);

    // if let
    let x = Some(5);
    if let Some(y) = x {
        println!("y = {}", y); // 5
    }
    match x {
        Some(y) => println!("y = {}", y),
        None => (),
    }


    let z = if let Some(y) = x { y } else { 0 };
    println!("z = {}", z);
    let z = match x {
        Some(y) => y,
        None => 0,
    };
    println!("z = {}", z);
}

#[test]
fn test_match() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("Something else"),
    }
}

#[test]
fn test_for_while_loop() {
    // for
    for x in 0..10 {
        print!("{} ", x);
    }
    println!();
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
    let lines = "Content of line one\
    Content of line two\
    Content of line three\
    Content of line four".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    // while
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // break

    let mut x = 5;
    loop {
        x +=  x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            break;
        }
    }

    // continue
    for x in 0..10 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }

    // label
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // continue the loop x
            if y % 2 == 0 {
                continue 'inner;
            } // continue the loop y
            println!("x: {}, y: {}", x, y);
        }
    }

}
