//! Rust 快速入门
// #[warn(unused_assignments)]


/// print hello world
#[test]
fn hello_world() {
    println!("Hello, World!");
    let rust = "Rust";
    println!("Hello, {}!", rust);
}


#[test]
#[allow(dead_code)]
fn variable_bind() {
    let a1 = 5;
    let a2: i32 = 5;
    assert_eq!(a1, a2);

    let _b1: u32 = 5;
    // assert_eq!(a1, b1); // mismatch types, expected 'i32', found 'u32'
}

#[test]
#[allow(dead_code)]
fn mut_variable_bind() {
    let mut a : f64 = 1.0; // Warning: value assigned to 'a' is never read
    let _b = 2.0f32;

    a = 2.0;
    println!("a = {}", a);

    let _a = a;

    // a = 3.0; // cannot assign twice to immutable variable 'a'

    // assert_eq!(a, b); // mismatched types, expected 'f64', found 'f32'
}

#[test]
fn deconstruction_variable() {
    let (a, mut b) : (bool, bool) = (true, false);
    println!("a = {}, b = {}", a, b);
    // a = false; // cannot assign twice to immutable variable

    b = true;
    assert_eq!(a, b);
}


#[test]
#[allow(dead_code)]
fn rust_type() {
    // boolean type
    let t = true;
    let f : bool = false;
    println!("t = {:?}", t);
    println!("f = {:?}", f);

    // char type
    let c = 'c';
    println!("c = {:?}", c);

    // numeric types
    let x = 42;
    println!("x = {:?}", x);
    let y: u32 = 123_456;
    println!("y = {:?}", y);
    let z: f64 = 1.23e+2;
    println!("z = {:?}", z);
    // let zero = z.abs(123.4);
    // println!("zero = {:?}", zero);
    let bin = 0b1111_0000;
    println!("bin = {:?}", bin);
    let oct = 0o7320_1546;
    println!("oct = {:?}", oct);
    let hex: i64 = 0xf23a_b049; // literal out of range for i32
    println!("hex = {:?}", hex);

    // arrays and slices
    let a = [0, 1, 2, 3, 4];
    println!("a = {:?}", a);
    let middle = &a[1..4];
    println!("middle = {:?}", middle);
    let ten_zero: [f64;10] = [0.; 10];
    println!("ten_zero = {:?}", ten_zero);


    // tuples
    let tuple: (i32, &str) = (50, "hello");
    println!("tuple = {:?}", tuple);
    let (fifty, _) = tuple;
    println!("fifty = {:?}", fifty);
    let hello = tuple.1;
    println!("hello = {:?}", hello);

    // raw pointers
    let x = 5;
    println!("x = {:?}", 5);
    let raw = &x as *const i32;
    println!("raw = {:?}", raw);
    let points_at = unsafe {
        *raw
    };
    println!("points_ar = {:?}", points_at);

    // functions
    fn foo(x: i32) -> i32 {
        x
    }
    let _bar: fn(i32) -> i32 = foo;
    // let temp = foo;

    // explicit conversion
    let decimal = 65.4321_f32;
    println!("decimal = {:?}", decimal);
    let integer = decimal as u8;
    println!("integer = {:?}", integer);
    let character = integer as char;
    println!("character = {:?}", character);

    // type aliases
    type NanoSecond = u64; // type alias is never used: 'NanoSecond'
    type Point = (u8, u8);

    let x: Point = (23, 34);
    println!("x = {:?}", x);
}

#[test]
fn array_type () {
    let mut array = [0;3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!(&[1, 2], &array[1..]);

    // This loop prints: 0, 1, 2
    print!("This loop prints: ");
    for x in &array {
        print!("{} ", x);
    }
}

#[test]
fn vec_type() {
    let v: Vec<i32> = Vec::new();
    println!("v = {:?}", v);

    let v: Vec<i32> = vec![];
    println!("v = {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);

    let v = vec![0; 10];
    println!("v = {:?}", v);

    let mut v = vec![1, 2];
    println!("v = {:?}", v);
    v.push(3);
    println!("v = {:?}", v);

    let mut v = vec![1, 2];
    println!("v = {:?}", v);
    let two = v.pop();
    println!("two = {:?}", two);

    let mut v = vec![1, 2, 3];
    println!("v = {:?}", v);
    let three = v[2];
    println!("three = {:?}", three);
    v[1] = v[1] + 5;
    println!("v[1] = {:?}", v[1]);
}

#[test]
fn str_type() {
    // &str
    let hello = "hello, world";
    println!("hello = {:?}", hello);

    let hello: &'static str = "hello, world";
    println!("hello = {:?}", hello);
}

#[test]
fn string_type() {
    let s = String::new();
    println!("s = {:?}", s);

    let mut hello = String::from("hello, ");
    println!("hello = {:?}", hello);

    hello.push('w');
    println!("hello = {:?}", hello);

    hello.push_str("orld");
    println!("hello = {:?}", hello);

    let mut s = String::from("foo");
    println!("s = {:?}", s);
    assert_eq!(s.pop(), Some('o'));
    println!("s = {:?}", s);
    assert_eq!(s.pop(), Some('o'));
    println!("s = {:?}", s);
    assert_eq!(s.pop(), Some('f'));
    println!("s = {:?}", s);
    assert_eq!(s.pop(), None);
    println!("s = {:?}", s);
}

#[test]
fn struct_type() {

    // structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 0, y: 0};
    println!("point = {:?}", point);

    // tuple structs
    #[derive(Debug)]
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    println!("android green = {:?}", android_green);
    let Color(red, green, blue) = android_green;
    println!("red = {}, green = {}, blue = {}", red, green, blue);

    // A tuple struct's constructors can be used as functions
    #[derive(Debug)]
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    println!("d = {:?}", d);

    // newtype: a tuple struct with only one element
    #[derive(Debug)]
    struct Inches(i32);
    let length = Inches(10);
    println!("length = {:?}", length);
    let Inches(integer_length) = length;
    println!("integer length = {:?}", integer_length);


    // unit-like structs
    #[derive(Debug)]
    struct EmptyStrcut;
    let empty = EmptyStrcut;
    println!("empty = {:?}", empty);

    #[derive(Debug, Default)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3d::default();
    println!("origin = {:?}", origin);
    let point = Point3d{ y: 1, ..origin };
    println!("point = {:?}", point);
    let Point3d { x: x0, y: y0, ..} = point;
    println!("x0 = {}, y0 = {}", x0, y0);

    // struct A {
    //     mut x: i32, // expected identifier, found keyword
    //     y: i32,
    // }

    use std::cell::Cell;

    #[derive(Debug)]
    struct A {
        x: i32,
        y: Cell<i32>,
    }
    let a = A {x: 5, y: Cell::new(6) };
    println!("a = {:?}", a);
    a.y.set(7);
    println!("a = {:?}", a);

}

#[allow(dead_code)]
mod graph {
    #[derive(Debug, Default)]
    pub struct Point {
        pub x: i32,
        y: i32,
    }
    pub fn inside_fn() {
        let p = Point{ x: 1, y: 2};
        println!("p.x = {}, p.y = {}", p.x, p.y);
    }
}

#[test]
fn test_outside_fn() {
    let p = graph::Point::default();
    println!("p.x = {}", p.x);
    graph::inside_fn();
    // print!("p.y = {}", p.y); // private field
    // field 'y' of struct 'ch1::graph::Point' is private
}

#[test]
fn test_if_statement() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("y = {:?}", y);

    // let y = (let x = 5); // 'let' expressions in this position are experimental
    // expected expression, found statement ('let')
    // unnecessary parentheses around assigned value

    // let z: i32 = if x == 5 { 10; } else { 15; }; // expected 'i32', found '()'
        // help: consider removing this semicolon
}

#[test]
fn test_for_statement() {
    print!("Number: ");
    for var in 0..12 {
        print!("{} ", var);
    }
}

#[test]
fn enum_type() {
    // #[derive(Default, Debug)] // Default cannot be derived for enums, only structs
    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move{x: i32, y: i32},
        Write(String),
    }

    let m: Message = Message::Move {x: 3, y: 4};
    println!("m = {:?}", m);
    let color: Message = Message::ChangeColor(23, 23, 23);
    println!("color = {:?}", color);
    let quit : Message = Message::Quit;
    println!("quit = {:?}", quit);
    let write : Message = Message::Write(String::from("hello, world"));
    println!("write = {:?}", write);
}

#[test]
fn test_loop_statement() {
    'outer: loop {
        println!("Entered the outer loop");

        // 'inner: loop { // unused label
        loop {
            println!("Entered the inner loop");
            break 'outer; // any code following this expression is unreachable
        }

        // println!("This point will never be reached"); // unreachable statement
    }
    println!("Exited the outer loop");
}

#[test]
#[allow(dead_code)]
fn test_match_statement() {
    let day = 5;
    match day {
        0 | 6 => println!("Weekend"),
        1 ..= 5 => println!("Weekday"), // '..=' for an inclusive range
        _ => println!("invalid"),
    }

    let x = 1;
    match x {
        e @ 1 ..= 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    // ref
    let x = 5;
    let mut y = 5;

    match x {
        // the 'r' inside the match has the type '&i32'
        ref r => println!("Got a reference to {}", r),
    }

    match y {
        // the 'mr'  inside the match has the type '&i32' and is mutable
        ref mut mr => {
            println!("Got a mutable reference to {}", mr);
            // mr = 3;
        },
    }

    let pair = (0, -2);
    match pair {
        (0, y) => println!("x is '0' and 'y' is '{:?}'", y),
        (x, 0) => println!("x is '{:?}' and  y is '0'", x),
        _ => println!("It doesn't matter what they are"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point{ x: 0, y: 0};
    match origin {
        Point{ x, ..} => println!("x is {}", x),
    }

    enum OptionalInt{
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    // if let && while let
    let number = Some(7);
    let mut optional = Some(0);

    // If 'let' destructures 'number' into 'Some(i)', evaluate the block.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number!");
    }

    // while 'let' destructures 'optional' into 'Some(i)', evaluate the block.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("'i' is '{:?}'. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

#[test]
#[allow(dead_code)]
fn test_function() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // fn diverges() -> ! {
    //     panic!("This function never returns!");
    // }
    //
    // let _x: i32 = diverges();
    // let _y: String = diverges();

    let num = 5;
    let plus_num = |x: i32| x + num;
    println!("3 plus num is {:?}", plus_num(3));

    let mut num = 5;
    {
        let mut add_num = move | x: i32| num += x;

        add_num(5);
        // println!("5 add num is {:?}", add_num(5));
        // println!("num = {:?}", num);
    }
    assert_eq!(5, num);

    // 高阶函数
    // fn add_one(x: i32) -> i32 { x + 1}

    fn apply<F>(f: F, y: i32) -> i32
        where F: Fn(i32) -> i32
    {
        f(y) * y
    }

    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    let transform: fn(i32) -> i32 = add_one;
    let f0 = add_one(2i32) * 2;
    let f1 = apply(add_one, 2);
    let f2 = apply(transform, 2);
    println!("f0 = {}, f1 = {}, f2 = {}", f0, f1, f2);

    let closure = |x: i32| x + 1;
    let c0 = closure(2i32) * 2;
    let c1 = apply(closure, 2);
    let c2 = apply(|x| x + 1, 2);
    println!("b0 = {}, b1 = {}, b2 = {}", c0, c1, c2);

    let box_fn = factory(1i32);
    let b0 = box_fn(2i32) * 2;
    let b1 = (*box_fn)(2i32) * 2;
    let b2 = (&box_fn)(2i32) * 2;
    println!("b0 = {}, b1 = {}, b2 = {}", b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &dyn Fn(i32) -> i32 = add_num;
    let z0 = add_num(2i32) * 2;
    let z1 = apply(add_num, 2);
    let z2 = apply(translate, 2);
    println!("z0 = {}, z1 = {}, z2 = {}", z0, z1, z2);

}


#[test]
#[allow(dead_code)]
fn test_method() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        pub fn new(x: f64, y: f64, radius: f64) -> Self {
            Self {
                x, y, radius
            }
        }
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle{ x: 0., y: 0., radius: 2.};
    println!("circle area = {}", c.area());

    // use associated function and method chaining.
    println!("circle area = {}", Circle::new(0., 0., 2.).area());
}

#[test]
#[allow(dead_code)]
fn test_trait () {
    trait  HashArea {
        fn area(&self) -> f64;
    }

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl HashArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl HashArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    fn print_area<T: HashArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }

    use std::fmt::Debug;

    fn foo<T: Clone, K : Clone + Debug>(x: T, y: K) {
        let _x = x.clone();

        let _y = y.clone();
        println!("y = {:?}", y);
    }
    fn bar<T, K>(x: T, y: K)
        where T: Clone, K: Clone + Debug
    {
        let _x = x.clone();
        let _y = y.clone();
        println!("y = {:?}", y);
    }

    trait Foo {
        fn foo(&self);

        // default method
        fn bar(&self) {
            println!("we called bar.");
        }
    }

    trait FooBar : Foo {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn foo(&self) {
            println!("foo");
        }
    }

    impl FooBar for Baz {
        fn foobar(&self) {
            println!("foobar");
        }
    }

    trait A {
        fn display(&self) {
            println!("A");
        }
    }

    trait B {
        fn display(&self) {
            println!("B");
        }
    }

    struct C;
    impl C {
        fn display(&self) {
            println!("C");
        }
    }
    impl A for C {}
    impl B for C {}

    let c = C;
    c.display(); // A / B / C
    // A::display(&c); // A
    // B::display(&c); // B
    <C as A>::display(&c);
    <C as B>::display(&c);
}


#[test]
#[allow(dead_code)]
fn test_generic() {
    fn make_pair<T, U>(a: T, b: U) -> (T, U) {
        (a, b)
    }

    let couple = make_pair("man", "female");
    println!("couple = {:?}", couple);

    // generic structs
    #[derive(Debug)]
    struct  Point<T> {
        x: T,
        y: T,
    }
    let _init_origin = Point{x: 0, y: 0};
    println!("init origin = {:?}", _init_origin);
    let _float_origin = Point{ x: 0., y: 0.};
    println!("float origin = {:?}", _float_origin);


    // use generic parameters
    // trait Graph<N, E> {
    //     fn has_edge(&self, &N, &N) -> bool;
    //     fn edges(&self, &N) -> Vec<E>;
    // }
    //
    // fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    //     unimplemented!()
    // }

    // use associated types
    trait Graph {
        type N;
        type E;

        fn has_edge(&self, node1: &Self::N, node2: &Self::N) -> bool;
        fn edge(&self, node: &Self::N)  -> Vec<Self::E>;
    }

    fn distance<G: Graph> (_graph: &G, _start: &G::N, _end: &G::N) -> u32 {
        unimplemented!()
    }

    struct Node;

    struct Edge;

    struct SimpleGraph;

    impl Graph for SimpleGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, _node1: &Node, _node2: &Node) -> bool {
            unimplemented!();
        }

        fn edge(&self, _node: &Node) -> Vec<Edge> {
            unimplemented!();
        }
    }
}


#[test]
fn test_input_output() {
    // Ex1;
    // use std::io;
    //
    // fn read_input() -> io::Result<()> {
    //     let mut input = String::new();
    //
    //     io::stdin().read_line(&mut input)?;
    //
    //     println!("You typed: {}", input.trim());
    //     Ok(())
    // }
    //
    // read_input();

    // Ex2;
    // use std::io;
    //
    // let mut input = String::new();
    //
    // io::stdin().read_line(&mut input).expect("WTF!");
    //
    // println!("You typed: {}", input.trim());
}


#[test]
fn test_output() {
    print!("This ");
    print!("will ");
    print!("be ");
    print!("on ");
    print!("the ");
    print!("same ");
    print!("line ");
    println!();

    print!("this string has a newline, why not choose println! instead?\n");

    println!("hello there!");
    println!("format {} arguments", "some");

}