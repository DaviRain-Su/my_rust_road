fn main() {
    // let a = "hello";
    // let c = &a[1..];
    // println!("a = {}",a);
    // println!("c = {}", c);
    // println!("a pointer = {:p}", a);
    // println!("c pointer = {:p}", c);
    // println!("a as_ptr = {:p}", a.as_ptr());
    // println!("c as_ptr = {:p}", c.as_ptr());
    // // println!("Hello, world!");
    let a = [0i32; 20];
    let b = &a[1..3];
    let c  = "hello".to_owned();
    let d = &c;
    println!("size a = {}, size b = {}, size c = {}, size d = {}",
        std::mem::size_of_val(&a),
        std::mem::size_of_val(&b),
        std::mem::size_of_val(&c),
        std::mem::size_of_val(&d),
    );
    // println!("a len = {}, b len = {}, c len = {}, d len = {}", a.len(), b.len(), c.len(), d.len());
    // let a = "hello world";

    // let ref_a = a.as_ptr();
    // println!("a.as_ptr = {:p}", ref_a);
    // println!("(&a).as_ptr = {:p}", (&a).as_ptr());
    // // println!("as_ptr(&a) = {:?}", as_ptr(&a));
    // println!("&a = {:p}", &a);
    let len = "foo".len();
    assert_eq!(3, len);

    assert_eq!("foo".len(), 3); 
    assert_eq!("foo".chars().count(), 3);
}
