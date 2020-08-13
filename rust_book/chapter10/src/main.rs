extern crate chapter10;
fn main() {
    // chapter10::generics_type::find_list_max();
    // chapter10::generics_type::generics_strcuct();
    let mut x = String::from("hello, world!");
    let ptr = x.as_ptr();
    println!("Address is {:p}", ptr);
    println!("x is {}", x);
    let x_ptr = &x as &str as * const str;
    let y_ptr = &mut x as *mut String;
    println!("Address is {:p}", x_ptr);
    println!("Address is {:p}", y_ptr);
    chapter10::generics_type::generics_strcuct();
}
