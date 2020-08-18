/// # 变量绑定
///
/// Rust通过静态类型确保类型安全，变量绑定可以在声明时说明类型，
/// 不过在多数情况下，编译器能够从上下文推导出变量的类型，从而大大减少类型说明的工作
/// 使用let 绑定操作可以将值绑定到变量
///
///
pub fn variable_bounding_example() {
    let an_integer = 1u32;
    let a_boolean = true;

    let uint = ();
    let copied_integer  = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", uint);

    let _unused_variable = 3u32;
    let _noisy_unused_variable = 2u32;
}


/// 可变绑定
///
/// 变量绑定默认是不可变的, 但是加上mut修饰后变量就可以改变
///
///
pub fn mutable_example() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation : {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1;
}

/// # 作用域和遮蔽
///
///
/// 变量绑定有一个作用域，它被限定只在一个代码块中生存，代码块是一个被{}包围的语句集合。
/// 也允许变量遮蔽
///
///
pub fn variable_scope_example () {
    let _long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short : {}", short_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

/// # 变量先声明
///
/// 可以先声明变量绑定，后面才将他们初始化，
/// 这种方法可能导致未初始化变量
///
/// 编译器禁止使用未进行初始化的变量，因为这会产生未定义行为
///
pub fn declear_variable() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;

    }
    println!("a binding : {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}

