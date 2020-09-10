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
/// ```
/// pub fn modify_ref_mut(v: &mut [u32]) {
///     v.reverse();
/// }
/// let mut v = vec![1,2,3];
/// modify(&mut v); // 这里的绑定相当于是let v = &mut v;
/// println!("{:?}", v); // 3, 2, 1
/// ```
///  
pub fn modify_ref_mut(v: &mut [u32]) {
    v.reverse();
}
#[test]
fn test_ref() {
    let mut v = vec![1, 2, 3];

    let v2 = &v;

    let v1 = &mut v;
    v1.reverse();
    println!("{:?}", v1);
    // println!("{:?}", v2); // error     
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
#[test]
pub fn function_para_pattern() {
    let string = String::from("hello, world");

    fn foo(ref s : String){ // let ref s = s;
        println!("s = {}", s);
    }
    
    foo(string);
    // println!("{:?}", string);// error, stirng have been moved

    #[derive(Debug)]
    pub struct S { i : i32 }
    fn fOwnership(ref mut s : S) {// let ref s = s;
        // println!("{:p}", s);
        // s = S { i: 22};
        s.i = 22;
    }
    fn fRefmut(ref mut s: &mut S) {
        s.i = 22;
    }
    fn fRef(ref s : &S) {
        println!("{:p}", s);
    }
    let mut s = S { i : 23};
    // let s1 = s;
    println!("{:?}", s);
    fRef(&s);
    fRefmut(&mut s);
    println!("{:?}", s);// error s have been moved

    fn foo2( _ :i32) {
        println!("foo");
    }
    foo2(3); 

    fn swap((x, y): (&str, i32)) -> (i32, &str) { // let (x, y) = temp;
        (y, x)
    }
    let t= ("alex", 18);
    println!("{:?}",t);
    let t = swap(t);
    println!("{:?}",t);
}
#[test]
fn test_function_return_value () {
    fn addsub(x: isize, y: isize) -> (isize, isize) {
        (x + y, x - y)
    }
    let (a, b) = addsub(5,8);
    println!("a: {:?}, b: {:?}", a, b);

    fn gcd(a: u32, b: u32 ) -> u32 {
        if b == 0 { return a; }
        return gcd(b, a % b);
    }
    let g = gcd(60,40);
    assert_eq!(20, g);
}

/// # 泛型函数
/// 
/// 这里在调用square函数的时候并没有指定具体地类型，而是靠编译器来进行自动推断的。
/// 这里使用的都是基本原生类型，编译器推断起来很简单。
/// 但是肯定也存在编译器无法自动推断的情况，此时就需要显式地指定函数调用的类型，
/// 这里就需要用到了turbofish擦作符了::<>
#[test]
fn test_fan_function(){
    use std::ops::Mul;
    fn square<T: Mul<T,Output=T>>(x:T, y: T) -> T {
        x * y
    }
    let a = square(37, 41);
    let b = square(37.2, 42.1);
    println!("a = {}, b = {}", a, b);
    let a = square::<u32>(34, 23);
    let b = square::<f32>(37.2, 42.1);
    println!("a = {}, b = {}", a, b);
}

/// ## 方法和函数
///  
#[test]
fn function_and_way(){
    #[derive(Debug)]
    struct User {
        name: &'static str,
        avatar_url: &'static str,
    }

    impl User {
        pub fn show(&self) {
            println!("name : {:?}", self.name);
            println!("avatar : {:?}", self.avatar_url);
        }
    }

    let user = User {
        name: "alex",
        avatar_url: "hhehwewkejk",
    };

    // User::show(&user);
    user.show(); // 等价于User::show(&user);
    // 结构体实例user被隐式传递给show方法，user就是show方法的接受者。
}

/// ## 高阶函数
/// 
/// 在数学中的，高阶函数叫做算子或泛函。在计算机科学中，高阶函数是指函数作为
/// 参数或返回值的函数，它也是函数式编程语言最基本的特性。
/// Rust 语言也支持高阶函数，因为函数在Rust中是一等公民。
/// 
/// ### 函数可以作为参数传递
/// 
mod function  {
    #[test]
    fn func_as_para() {
        // math 是一个高阶函数，在调用的时候传入的只是函数名
        // 实现这一切的基础在于Rust支持类似C/C++语言中的函数指针。
        // 函数指针，是指向函数的指针，其值为函数的地址。
        fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
            op(a,b)
        }

        fn sum(a: i32, b: i32) -> i32 {
            a + b
        }
        fn product(a: i32, b: i32) -> i32 {
            a * b
        }

        let (a, b) = (3, 4);
        println!("sum = {}, product = {}", math(sum, a, b), math(product, a, b));
    }

    #[test]
    fn function_pointer(){
        fn hello() {
            println!("hello function pointer");
        }
        // let声明必须显式指定函数指针类型为fn(), 以及赋值使用的是函数名hello
        // 而非带括号的函数调用
        let fn_ptr : fn() = hello; // 这里是函数指针
        println!("{:p}", fn_ptr); 
        let other_fn = hello; // 这里是函数调用， other_fn 的类型
        // 为fn() {hello}, 这其实就是函数hello本身的类型，所以Other_fn不是函数指针类型。

        // println!("{:p}", other_fn);// erro 这不是函数指针
        hello();
        fn_ptr();
        other_fn();
        (other_fn)();
    }

    #[test]
    fn function_as_return() {
        type MathOp = fn(i32, i32) -> i32;
        fn math(op: &str) -> MathOp {
            fn sum(a: i32, b: i32) -> i32 {
                a + b
            }
            fn product(a: i32, b: i32) -> i32 {
                a * b
            }

            match op {
                "sum" => sum,
                "product" => product,
                _ => {
                    println!(
                        "Warning: Not Implemented {:?} operator, Replace with sum", 
                        op 
                    );
                    sum
                }
            }
        }


        //test
        let (a, b) = (2, 3);
        let sum = math("sum");
        let product = math("product");
        let div = math("div");
        assert_eq!(sum(a, b), 5);
        assert_eq!(product(a, b), 6);
        assert_eq!(div(a, b), 5);
    }

    #[test]
    fn function_as_return2() {
        fn sum(a: i32, b: i32) -> i32 {
            a + b
        }
        fn product(a: i32, b: i32) -> i32 {
            a * b
        }
        
        type Mathop = fn(i32, i32) -> i32;
        // fn math(op: &str, a: i32, b: i32) -> Mathop {
        //     match op {
        //         "sum" => sum(a,b),// this type is a i32
        //         _ => product(a, b), // this type is a i32
        //     }
        // }
        
        let (a, b) = (3, 2);
        // let sum = math("sum", a, b);
    }
    #[test]
    fn function_as_return3(){
        fn counter() -> fn(i32) -> i32 {
            fn inc(n: i32) -> i32 {
                n + 1
            }
            inc
        }

        let f = counter();
        assert_eq!(2, f(1));
    }

    #[test]
    fn function_as_return4(){
        fn counter() -> fn(i32) -> i32 {
            fn inc(n: i32) -> i32 {
                n + 1 // 函数不能捕获外部环境，
                // Rust中不允定义的函数捕获动态环境中的变量，因为
                // 变量i会随着栈帧的释放的而释放，解决方式是可以采用闭包来使用
            }
            inc
        }

        let f = counter();
        assert_eq!(2, f(1));
    }
}