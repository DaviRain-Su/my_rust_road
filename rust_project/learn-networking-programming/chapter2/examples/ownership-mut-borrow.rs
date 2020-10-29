fn main() {
    let mut s = String::from("Test");
    heap_example(&mut s);
}

fn heap_example(input: &mut str) {
    let mystr = input; // value moved here
    let _otherstr = &mystr;
    println!("{}", mystr);
    //    println!("{}", input);
}
