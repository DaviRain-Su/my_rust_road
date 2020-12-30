// use std::io;


fn main() {
    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();

    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to line");

    // println!("You guessed: {}", guess);
    
    let s = String::from("hello world");

    print_type_of(&s);

    let a = 23;

    print_type_of(&a);

    let b = ();

    print_type_of(&b);

    let a = vec![1, 2, 3, 4];
    print_type_of(&a);

    let b = &a;
    print_type_of(&b);

    let c = String::from("hello");
    print_type_of(&c);
    // println!("c = {:?}", c);
    let c = c.as_bytes();
    print_type_of(&c);
    // println!("c = {:?}", c);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
// &mut i32;
// &i32; 

/// # Doc Test
/// 
/// ```rust
/// 
///     println!("hello world");
/// 
/// ```
fn test_result() {
    let a : Result<i32, i32> = Ok(23);
    // let a : Result<i32, i32> = Err(34);
    // if let Ok(temp) = a { // let Ok(temp) = Ok(23); temp == 23;
    //     println!("temp = {}", temp);
    // }else if let Err(temp) = a {
    //     println!("temp = {}", temp);
    // }

    match a {
        Ok(suc) => println!("suc = {}", suc),
        Err(err) => println!("err = {}", err),
    }
}

// Vec<u8> 
// &Vec<u8> ---> &[u8]

#[test]
fn test() {
    let a = vec![1, 2, 3, 4];
    println!("a = {:?}", a);
    let b = &a;
    println!("b = {:?}", b);

    let c = String::from("hello");
    println!("c = {:?}", c);
    let c = c.as_bytes();
    println!("c = {:?}", c);
}