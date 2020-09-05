pub mod model_pattern;
/// # 函数、闭包与迭代器
/// 
/// 
/// # 6.1 函数
/// 
/// rust中的函数参数不能指定默认值
/// 
/// 利用raw identifier将语言关键字用作函数名
/// 
/// r#match 用于FFI, 用于避免C函数和Rust的关键字或保留重名而引起冲突。
/// 
/// fn 后的函数名，通常是sanke_case 格式，否则编译器会警告。
/// 
/// 函数的参数和返回值都必须指明类型
/// 
pub fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

/// 参数传递规则， 可以按值传递，也可以按引用传递。
/// 按值传递，会转移所有权或者执行复制语义。
/// 按引用传递时，不会发生所有权的转移。（**这里按引用传递是不是实际上也是执行引用的Copy操作**）
/// 这里Copy的是引用这个值，因为引用实现了Copy trait所以不会发生所有权的转移。**）
/// 
/// 使用引用传递要标明引用的生命周期参数，可以自动推导的不需要添加，不能推导的需要手动添加。
/// 
/// 函数的参数也有可变和不可变，可变和变量的声明一样在参数名前加mut,`fn foo(mut i: i32);`
/// 
/// Rust中有这样的一个观点，Rust中的每个函数都是自治的，在每一个函数体中
/// ，相当于重新开辟了一个新的领域。
/// 
/// ```rust
/// fn modify(mut v: Vec<u32>) -> Vec<u32> {
///     v.push(42);
///     v
/// }
/// ```
/// 就像这个函数在调用的时可以这样搞， 
/// ```
/// let v = vec![1, 2, 3];
/// modify(v);
/// ```
/// 可以看到直接传入的就是一个不可变得值v，但是因为函数有这样一个规则，
/// 传入的数和参数相当于去做一次绑定，因此对于函数传递过来的v被重新绑定为了可变的参数v
/// 这里可以看出，rust中的每个函数体都是自治的。
///  

/// by value and mutable
///
pub fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
} 

/// by reference of mutable
/// 
pub fn modify_ref_mut(v: &mut [u32]) {
    v.reverse();
}

/// ## 6.1.1 函数遮蔽
/// 
/// 对于变量来说可以声明同名的变量遮蔽原来的变量，但是对于函数来说Rust中是不可以的。
/// 
/// 一种trick就是：
/// 
/// 可以通过显示地使用花括号将同名的函数分隔到不同的作用域中，这样就可以不让编译器报错了。
/// 
/// 这样也就推导出了一个规则： 
/// 
/// 在同一个作用域中不能同时定义多个同名的函数, 这是因为默认的函数定义只有在当前的作用
/// 中有效，会屏蔽作用域外的同名函数。 
/// 
/// 
/// 
pub fn function_shadow() {
    fn _f() { print!("1");} // 这个函数被屏蔽掉了不会被调用
    {
        f(); // 2
        {
            f(); // 3
            fn f() { print!("3");}
        }
        f(); // 2
        fn f() { print!("2"); }
    }
    // output: 232
    println!();
}

/// ## 6.1.2 函数参数匹配
/// 
/// 上面说过函数的参数是一个隐式地绑定（let绑定）， 既然是绑定了所以也就支持模式匹配了
/// 因为let绑定本事就是一个模式匹配
/// 
/// ```
/// let a = (1, 2, 3);
/// let (b, c, d ) = a; // 从这里看出a进行了模式匹配，b = 1, c = 2, d = 3;
/// 
/// ```
/// 
pub fn function_para_pattern() {
    let string = String::from("hello, world");

    fn foo(ref s : String){
        println!("s = {}", s);
    }
    
    foo(string);

} 