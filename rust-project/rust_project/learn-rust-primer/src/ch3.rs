//! 类型 运算符 字符串

#[test]
fn test_bool_type() {
    let is_she_love_me = false;
    let mut is_she_love_me_too: bool = true;
    println!("is she love me = {:?}", is_she_love_me);
    println!("is he love me too = {:?}", is_she_love_me_too);
    is_she_love_me_too = false;
    println!("is he love me too = {:?}", is_she_love_me_too);
}

#[test]
fn test_char_type() {
    let c = '中';
    println!("c = {:?}", c);
    let cc = '国';
    println!("cc = {:?}", cc);
}

#[test]
fn test_array_type() {
    let a = [8, 9, 10];
    let b: [u8; 3] = [8, 6, 5];
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    // #[warn(dead_code)]
    fn show(arr: [u8; 3]) {
        for i in &arr {
            print!("{} ", i);
        }
    }
    fn show_ref(arr: &[u8]) {
        for i in arr {
            print!("{} ", i);
        }
    }

    let a : [u8; 3] = [1, 2, 3];
    // show(a);
    show_ref(&a);
    let b : [u8; 4] = [1, 2, 3, 4];
    // show(b); // expected an array with a fixed size of 3 elements, found on with 4 elements
    show_ref(&b);
}

#[test]
fn test_slice_type() {
    let arr = [1, 2, 3, 4, 5, 6];
    println!("arr = {:?}", arr);
    let slice_complete = &arr[..];
    println!("slice complete = {:?}", slice_complete);
    let slice_middle = &arr[1..4];
    println!("slice middle = {:?}", slice_middle);
    let slice_right = &arr[1..];
    println!("slice right = {:?}", slice_right);
    let slice_left = &arr[..3];
    println!("slice left = {:?}", slice_left);
}

#[test]
fn test_ver_type() {
    let mut v1 : Vec<i32> = vec![1, 2, 3];
    let v2 = vec![0; 10];
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    for i in &v1 {
        print!("{}", i);
    }
    println!();
    for i in &mut v1 {
        *i = *i + 1;
        print!("{}", i);
    }
}

#[test]
fn test_function_type() {
    fn foo(x: i32) -> i32 {
        x + 1
    }
    let x: fn(i32) -> i32  = foo;
    assert_eq!(11, x(10));
}

#[test]
fn test_tuple_type() {
    let y = (2, "hello, world");
    println!("y = {:?}", y);
    let x: (i32, &str) = (3, "hello, world");
    println!("x = {:?}", x);

    let (w, z) = y;
    println!("w = {:?}, z = {:?}", w, z);


    let f = x.0;
    println!("f = {:?}", f);
    let e = x.1;
    println!("e = {:?}", e);
}

#[test]
// #[warn(dead_code)]
fn test_struct_type() {

    struct A {
        attr1: i32,
        attr2: String,
    }

    // #[warn(dead_code)]
    struct B(i32, u16, bool);

    // #[warn(dead_code)]
    struct  D;

    struct Person {
        name: String,
    }

    impl Person {
        fn new(n: &str) -> Self {
            Self {
                name: n.to_string(),
            }
        }

        fn greeting(&self) {
            println!("{} say hello .", self.name);
        }
    }

    let peter = Person::new("Peter");
    peter.greeting();

    // struct  RefBody {
    //     loc: &i32, // expected named lifetime parameter
    // }

    struct RefBody<'a> {
        loc: &'a i32,
    }

    struct  E {
        e: i32,
    }

    impl E {
        pub fn show(&self) {
            println!("{}", self.e);
            // self.add_one();
            // 'self' is '&' reference,so the data it refers to cannot be borrowed as mutable
        }
        pub fn add_one(&mut self) {
            self.e += 1;
        }
        pub fn add_two(&mut self) {
            self.add_one();
            self.add_one();
            self.show();
        }
    }

    let ast = E { e: 12i32 };

    ast.show(); // ast moved due to this method call
    println!("{}", ast.e); // value borrowed here after move

}


#[test]
fn test_enum_type() {
    enum Direction{
        West,
        North,
        South,
        East,
    }

    enum SpecialPoint {
        Point(i32,i32),
        Special(String),
    }

    enum SpecialPointName {
        Point {
            x: i32,
            y: i32,
        },
        Special(String),
    }

    let sp = SpecialPoint::Point(0, 0);
    match sp {
        SpecialPoint::Point(x, y) => {
            println!("I'm SpecialPoint(x = {}, y = {})", x, y);
        },
        SpecialPoint::Special(why) => {
            println!("I am Special because I am {}", why);
        }
    }
}
