fn main() {
    let s = String::from("Test");
    heap_example(&s);
}

fn heap_example(input: &str) {
    let mystr = input;
    let _otherstr = &mystr;
    println!("{}", mystr);
}
