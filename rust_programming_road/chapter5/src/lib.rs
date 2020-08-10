pub mod davirain {

pub fn ownership_demo() {
    let orig = Box::new(5);
    println!("orig is {}", orig);
    // println!("orig address is {:?}", orig);
    let stolen = orig; // value moved here
    // println!("{}", *orig); //  value borrowed here after move
    println!("stolen is {}", stolen);
}

pub fn value_semantic() {
    let x = 5;
    let y = x;
    println!("the value x is {}, the value y is {}", 
    x, y);
    assert_eq!(x, 5);
    assert_eq!(y, 5);
}

pub fn reference_semantic() {
    // use std::ops::Drop;
    #[derive(Debug, Clone)]
    struct A {
        a: i32,
        b: Box<i32>, // the trait 'Copy' cannot implement for this type
    }

    impl Drop for A {
        fn drop(&mut self){
            println!("Drop {:?}", self);
        }
    }

    let a = A { a: 2 , b: Box::new(4)};
    println!("a is {:#?}", a);
    {
        let b = a.clone();
        println!("b is {:#?}", b);
    }
    println!("a is {:#?}", a);
}

pub fn struct_demo_ownership() {
    #[derive(Debug, Copy, Clone)]
    struct A {
        a: i32,
        b: i32,
    }

    let a = A { a: 1, b: 2};
    let b = a;
    println!("b is {:#?}", b);
    println!("a is {:#?}", a);
}

pub fn tuple_demo_ownership() {
    let a = ("a".to_string(), "b".to_string());
    let b = a;
    println!("b is {:#?}", b);
    let c = (1, 2, 3);
    let d = c;
    println!("d is {:#?}", d);
    println!("c is {:#?}", c);
}

}