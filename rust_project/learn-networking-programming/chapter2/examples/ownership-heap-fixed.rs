fn main() {
    let s = String::from("Test");
    heap_example(s);
}

fn heap_example(input: String) {
    let mystr = input; // value moved here
    let _otherstr = mystr.clone();
    println!("{}", mystr); // value used here after move
}
