
#![feature(core_intrinsics)]
unsafe fn print_type_of<T>(_: &T) {
    println!("{}", std::intrinsics::type_name::<T>() );
}
fn main() {
    let x = "learn rust";
    let a = String::from("hello world");
    let b = 8u8;
    let c = 3.24;
    unsafe {
        print_type_of(&x);
        print_type_of(&a);
        print_type_of(&b);
        print_type_of(&c);
    }

    let mut x  = 32;
    println!("Current value of x: {}", x);
    x = 64;
    println!("Current value of x: {}", x);

    // x = "rust"; // expected integer, found "&str"
    println!("Current value of x: {}", x);

    // shaowd
    let x = 1;
    let x = x + 2;
    let x = x * 2;
    println!("Value of x: {}", x);


    // scope
    let x = 5;
    if 4 < 10 {
        let x = 10;
        println!("Inside if x = {:?}", x);
    }
    println!("Outside if x = {:?}", x);
    println!("Square 5 = {:?}", square_of(-5));

    let place = "himalayas";
    let weather = if place == "himalayas" {
        "cold"
    }else {
        "hot"
    };
    println!("{:?}", weather);

    let weather = match place {
        "himalayas" => "cold",
        _ => "cold",
    };
    println!("weather = {:?}", weather);
}

fn square_of(x: i32) -> i32 {
    println!("x = {:?}", x);
    x.pow(2)
}

