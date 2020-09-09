use chapter6;

fn main() {
    //----------------test match----------------------
    assert!(chapter6::r#match("foo", "foobar"));
    //------------------------------------------------

    //------------test by value modify -----------------
    let v = vec![1, 2, 3];
    // 这里的v传入到modify中相当于函数中的参数和v重新做了一次绑定，
    // modify中的参数是可变的，所以传入的v是不可变的重新绑定变为了可变的。
    // modify 函数参数前有mut修饰，表明这个变量是可变的参数。
    assert_eq!(chapter6::modify(v), vec![1, 2, 3, 42]);
    //------------------------------------------------
    
    //------test by reference modify_ref_mut--------------
    let mut v = vec![1,2, 3];
    // 按引用传递，因为函数的参数需要的是一个可变的引用,
    // 所以&引用修饰的变量也一定要是一个可变的变量。
    chapter6::modify_ref_mut(&mut v);
    assert_eq!(v, vec![3, 2, 1]);
    //------------------------------------------------

    //---------- test function shadow ----------------  
    chapter6::function_shadow();
    //------------------------------------------------

    //----------test fcuntion para pattern ------------
    // chapter6::function_para_pattern();

    let a = A{
        a: "hello".to_string(),
        b: "world".to_string(),
    };
    
    // 这个模式匹配没有转移所有权
    let A {a: ref c, b: ref d} = a; // 这就相当这里有这个a了，不会再去发生绑定

    println!("{}, {}", c, d);
    println!("a = {:?}", a);
    
    // 而这里在去做传入参数的时候，要符合，传值还是传引用来做。
    // foo, 到了这里转移了所有权？？下面的print不能打印了！！！
    foo(a); // 这个foo函数参数的绑定不是和上面那句let声明的绑定一样吗？？？
    // println!("a = {:?}", a); // error

    chapter6::model_pattern::example();
}


#[derive(Debug)]
struct A {
    a: String,
    b: String,
}

fn foo(A {a: ref c, b: ref d} : A){
    println!("{}, {}", c, d);
}
