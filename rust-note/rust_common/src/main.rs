fn main() {
    let mut variable: i32 = 3;
    variable = 4;
    println!("variable  = {}", variable);

    let a = Box::new(5);
    let c = *a;
    println!("a = {}", a);

}
