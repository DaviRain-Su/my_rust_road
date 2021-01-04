
fn main() {
    let mut s = String::from("hello world");
    let ret2 = ref_scope(&mut s);
    println!("ret2 = {}", ret2);
    println!("s = {}", s);
}

fn ref_scope(par: &mut str) -> &str {
    par
}

