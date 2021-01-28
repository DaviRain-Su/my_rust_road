#![feature(unboxed_closures, fn_traits)]
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

mod closure {
    
    /// 返回闭包
    #[test]
    fn return_closure(){
        // counter 函数返回的是一个闭包，放在Box<T>中，是因为闭包的大小在编译期是未知的。
        // 在Rust 2018 中，返回的闭包也可以使用impl Trait语法，impl Fn(i32) -> i32
        // fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
        
        // 这里的i是复制语义，所以是以按引用捕获。
        // 此引用会妨碍闭包做为函数返回值，编译器会报错，
        // 所以这里使用move 关键字来把自由变量i的所有权转移到闭包中，
        // 因为变量i是复制语义，所以这里只会进行按位复制。

        // 这里的Fn(i32) -> i32,并不是函数指针，而是一个triat
        fn counter(i: i32) -> impl Fn(i32) -> i32 {
            Box::new( move |n : i32| n + i)
        }
// error[E0373]: closure may outlive the current function, but it borrows `i`, which is owned by the current function
//    --> src/lib.rs:378:22
//     |
// 378 |             Box::new(|n : i32| n + i)
//     |                      ^^^^^^^^^     - `i` is borrowed here
//     |                      |
//     |                      may outlive borrowed value `i`
//     |
// note: closure is returned here
//    --> src/lib.rs:377:31
//     |
// 377 |         fn counter(i: i32) -> impl Fn(i32) -> i32 {
//     |                               ^^^^^^^^^^^^^^^^^^^
// help: to force the closure to take ownership of `i` (and any other referenced variables), use the `move` keyword
//     |
// 378 |             Box::new(move |n : i32| n + i)

        let f = counter(3);
        assert_eq!(4, f(1));
    }

    // 闭包函数可以使任意类型
    #[test]
    fn closure_any_para(){
        fn val() -> i32 { 5 }
        // 这里定义了闭包有两个函数
        // 第一个是函数指针类型，第二个是元组类型，虽然元组类型中没有显式的标注类型，但是Rust
        // 编译器会通过函数指针类型的信息来推断其为i32类型
        let add =  | a: fn() -> i32, (b, c) | (a)() + b + c;
        let  r = add(val, (2, 3));
        assert_eq!(r, 10);
    }
    #[test]
    fn different_closure(){
        let c1 = || { println!("c1");};
        let c2 = || { println!("c2"); };
        let v = [c1, c2];
        for item in v.iter() {
            item();
        }
    }

    #[test]
    fn closure_type() {
        // let v1 : () = || {};
//error[E0308]: mismatched types
//    --> src/lib.rs:407:23
//     |
// 407 |         let v1 : () = || {};
//     |                  --   ^^^^^ expected `()`, found closure
//     |                  |
//     |                  expected due to this
//     |
//     = note: expected unit type `()`
//                  found closure `[closure@src/lib.rs:407:23: 407:28]`
    }

    #[test]
    fn impl_closure() {
        
        struct Closure {
            env_var: u32,
        }

        impl FnOnce<()> for Closure {
            type Output = u32;
            extern "rust-call" fn call_once(self, args: ()) -> u32 {
                println!("call it FnOnce()");
                self.env_var + 2
            }
        }

        impl FnMut<()> for Closure {
            extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
                println!("call it FnMut()");
                self.env_var + 2
            }
        }
        
        impl Fn<()> for Closure {
            extern "rust-call" fn call(&self, args: ()) -> u32 {
                println!("call it Fn()");
                self.env_var + 2
            }
        }

        fn call_it<F: Fn() -> u32>(f: &F) -> u32 {
            f()
        }

        fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 {
            f()
        }

        fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 {
            f()
        }
        
        let env_var = 1;
        let mut c = Closure{ env_var : env_var };

        c(); // 默认调用的是Fn triat 实现的 call()方法，
        // 此处结构体实例可以像函数那样被调用，这是因为。
        // extern "rust-call" fn call(&self, args: ()) -> u32;
        // extern 关键字用于fn前，表示使用指定的ABI (Application Binary Interface）
        // 程序二进制接口，此处代表指定使用Rust语言的rust-call ABI， 它的作用是将
        // 函数参数中的元组类型做动态扩展，以便支持可变长参数。
        // 因为在Fn, FnMut, FnOnce这三个trait里方法要接受闭包参数，而编译器本身并不可能知道
        // 开发者给闭包设定的参数个数，所以这里只能传元组，然后由rust-call ABI 在底层做动态扩展。

        c.call(());         // 这里必须显式的指定一个单元值为参数
        c.call_mut(());     // 同上
        c.call_once(());    // 同上

        let mut c = Closure{ env_var: env_var};
        {
            assert_eq!(3, call_it(&c));
        }
        {
            assert_eq!(3, call_it_mut(&mut c));
        }
        {
            assert_eq!(3, call_it_once(c));
        }

        //模拟的闭包等价于这个， 闭包这里默认是用不可变引用的方式去捕获的
        let env_var = 1;
        let c = || env_var + 2;
        assert_eq!(3, c());
    }

    #[test]
    fn copy_closure_example() {
        let s = "hello"; // 字符串字面量是复制语义
        let mut c = || { println!("{:?}" ,s); }; //因为这里的println!是对s的不可变借用，所以这里的闭包类型是Fn,
                                    //因为闭包对于复制语义是按照不可变引用的方式捕获， 所以这里捕获的变量s是不可变引用的形式
        // 所以，根据闭包内使用变量的方式推断出这里的闭包时Fn, 捕获变量的方式是以不可变引用的方式去捕获
        c();
        c();// 闭包C可以两次调用，说明编译器自动为闭包表达式实现的结构体实例并未失去所有权。
        println!("{:?}", s); // 这里也说明了只有不可变借用才可以被借用多次。
        c;
        c.call_mut(()); // 这里因为调用的是call_mut所以需要c是mut的
        c.call_once(()); // 因为s是复制语义，所以这里默认实现的FnOnce也会自动实现Copy去捕获s
        // 所以此处调用call_once并不会导致闭包c所有权被转移
        // 如果闭包c的捕获变量是移动语义，那么调用call_once就会被转移所有权

        println!("{:?}", s); //这里依然可以打印s, 这也说明了在调用call_once之后闭包
        // 闭包依旧是按照不可变借用的方式捕获

        // 要实现Fn就必须实现FnMut, FnOnce, 所以被编译器翻译为匿名结构体和triat， 那么Fn, FnMut, FnOnce
        // 都会被实现。

        struct Closure<'a> {
            env_var: &'a str,
        }

        impl<'a> FnOnce<()> for Closure<'a> {
            type Output = ();
            extern "rust-call" fn call_once(self, args: ()) -> (){
                println!("{:?}", self.env_var);
            }
        }

        impl<'a> FnMut<()> for Closure<'a> {
            extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
                println!("{:?}", self.env_var);
            }
        }
        
        impl<'a> Fn<()> for Closure<'a> {
            extern "rust-call" fn call(&self, args: ()) -> () {
                println!("{:?}", self.env_var);
            }
        }

        let env_var = "hello";
        let mut c = Closure{ env_var: &env_var };
        c();
        c.call_mut(());
        c.call_once(());

    }

    #[test]
    fn move_closure() {
        let s = "hello".to_string(); // 移动语义类型
        let mut c = || s; // 这里闭包返回的是s, 也是采用的移动语义类型，
                                    // 所以闭包会以FnOnce的方式来操作
        // 这里闭包添加了mut来设置闭包的可变性，这里是为了显式的调用call_mut
        
        c();
        // c(); // error, use of moved value 'c'
        // println!("{:?}", s); // error, use of moved value 'c'

        // 闭包c在第一次调用时转移了所有权，导致第二次调用失败。证明了其实现的闭包结构体
        // 实例所实现的trait方法线束必然是self, 这个说名了闭包实现的是FnOnce
        // 后面的println调用失效，也说明了闭包c，夺走了s的所有权。

        // 既然闭包的默认调用的是FnOnce, 这也说明了编译器翻译的闭包结构体中记录捕获
        // 变量的成员字段不是引用类型，并且只实现了FnOnce, 所以肯定无法显式地调用
        // call, call_mut 方法

        // c.call(()); // error 
// error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
//    --> src/lib.rs:566:17
//     |
// 566 |         let c = || s; // 这里闭包返回的是s, 也是采用的移动语义类型，
//     |                 ^^^-
//     |                 |  |
//     |                 |  closure is `FnOnce` because it moves the variable `s` out of its environment
//     |                 this closure implements `FnOnce`, not `FnMut`
// ...
// 581 |         c.call(());
//     |           ---- the requirement to implement `FnMut` derives from here


            // c.call_mut(()); //erro

//             error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
//    --> src/lib.rs:566:17
//     |
// 566 |         let c = || s; // 这里闭包返回的是s, 也是采用的移动语义类型，
//     |                 ^^^-
//     |                 |  |
//     |                 |  closure is `FnOnce` because it moves the variable `s` out of its environment
//     |                 this closure implements `FnOnce`, not `FnMut`
// ...
// 595 |             c.call_mut(());
//     |               -------- the requirement to implement `FnMut` derives from here





//         error[E0382]: use of moved value: `c`
//    --> src/lib.rs:568:9
//     |
// 567 |         c();
//     |         --- `c` moved due to this call
// 568 |         c(); // error, use of moved value 'c'
//     |         ^ value used here after move
//     |
// note: closure cannot be invoked more than once because it moves the variable `s` out of its environment
//    --> src/lib.rs:566:20
//     |
// 566 |         let c = || s;
//     |                    ^
// note: this value implements `FnOnce`, which causes it to be moved when called
//    --> src/lib.rs:567:9
//     |
// 567 |         c();


        // 闭包c默认实现了FnOnce, 所以显式调用call, call_mut方法是，编译器都报错了，
        // 并且提示闭包只实现了FnOnce.

    }

    #[test]
    fn mut_closure() {
        let mut s = "rust".to_string();
        {
            println!("{:?}", s);
            let mut c = || { 
                s += "rust"; 
                // pub trait AddAssign<Rhs = Self> {
                //     fn add_assign(&mut self, rhs: Rhs);
                // }
                println!("{:?}", s); 
            };

            c();
            c();
            c.call_once(()); // 实现了FnMut的闭包，必然实现了FnOnce,但是不会实现Fn
            println!("{}",s);
            // c.call(());

        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
        //     --> src/lib.rs:643:25
        //      |
        //  643 |             let mut c = || { 
        //      |                         ^^ this closure implements `FnMut`, not `Fn`
        //  644 |                 s += "rust"; 
        //      |                 - closure is `FnMut` because it mutates the variable `s` here
        //  ...
        //  654 |             c.call(());
        //      |               ---- the requirement to implement `Fn` derives from here
        //     println!("{:?}", s);

        }

        // let mut s1 = String::from("hello");
        // // let s2 = &mut s1;
        // s1 += "world";
        // println!("{:?}", s1);
        println!("{:?}", s);
    }
    #[test]
    fn copy_move_example() {
        let s = "hello";
        let c = move || { 
            println!("{:?}", s); // 这里的使用方式是不可引用的方式，所以闭包实现的是Fn
        };
        c();
        c();
        println!("{:?}", s);

        let s1 = String::from("hello");
        let c1 = move || { 
            println!("{}", s1);
            drop(s1); // 移动语义，所以闭包实现的是FnOnce
        };
        c1();
        // c1();
        // println!("{}", s1);

        let s2 = String::from("hello");
        let c2 = move || {
            println!("{}", s2); // 闭包中变量的使用方式是不可变借用的方式，所以闭包实现的是Fn
        }; // 所以闭包可以被多次调用
        c2(); 
        c2();
        // println!("{}", s2); // s2是移动语义类型，使用move被转移到了闭包中
        
        let mut s3 = String::from("hello");
        let mut c3 = move || {
            s3 += ", world";
            println!("{}", s3);
        };
        c3();
        c3();
        // println!("{}", s3);

    }

    #[test]
    fn test_move_ex() {
        fn call<F: FnOnce()>(f: F) { f() }
        let mut x = 0;
        let inc_x = || x += 1;
        // inc_x();
        // inc_x();
        call(inc_x);
        // call(inc_x);

        let mut x = 0;
        let inc_x = move || x += 1;
        // inc_x();
        // inc_x();
        call(inc_x);
        call(inc_x);

        let mut x = vec![];
        let append_x = move || x.push(32);
        call(append_x);
        // call(append_x);

        let c = || { println!("hh"); };
        c();
        c();
        c();
    }

    #[test]
    fn box_closure_example(){
        /// Box<Fn()>是一个trait对象，吧闭包放到Box<T>中就可以构建一个闭包的trait
        /// 对象，然后就可以当做类型来使用。
        /// 
        /// triat对象时动态分发的，在运行时通过查找虚表来确定调用哪个闭包。
        fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>) {
            let s = "second";
            c.push(Box::new(|| 
                println!("first")
            ));
            // 闭包默认是以不可变借用的方式捕获了环境变量s，但是这里需要将闭包装箱，
            // 在之后的iter_call函数中调用，所以这里必须使用move关键字将s的所有权转移到闭包中。
            // 因为变量s是复制语义类型，所以该闭包捕获的是原始变量s的副本。
            c.push(Box::new(move || 
                println!("{}", s)
            ));
            c.push(Box::new(
                || println!("third")
            ));
        }

        let mut c = vec![];
        boxed_closure(&mut c);
        for f in c {
            f();
        }

        // 像这种在函数boxed_closure调用之后才会使用的闭包，叫做逃逸闭包。
        // 因为该闭包捕获的环境变量”逃离“了boxed_closure函数的栈帧，所以在函数
        // 栈帧销毁之后依然可用。
        // 与之对应的，如果是跟随函数一起调用的闭包，则是非逃逸闭包。

    }

    #[test]
    /// 静态分发实现
    fn closure_as_para_static_dispatch_example(){
        // 这里的use语句可有可无，因为Fn并不受triat孤儿规则的限制，
        use std::ops::Fn;

        trait Any {
            // 在这个tira中声明了泛型函数any，该函数F的triat限定为
            // Fn(u32) -> bool, 这种形式更像函数指针
            // 函数指针也是默认实现了Fn, FnMut, FnOnce这三个triat。

            // 在where从句中对Self做了Sized限定，这意味着，当Any被作为trait对象使用时，
            // 该方法不能被动态调用，这属于一种优化策略。
            // any方法的作用是，会迭代传入的闭包，依次调用，如果满足闭包表达式中指定的条件，
            // 则返回true，否则返回false。
            fn any<F>(&self, f: F) -> bool 
                where Self: Sized, F: Fn(u32) -> bool;
        }

        impl Any for Vec<u32> {
            fn any<F>(&self, f: F) -> bool 
            where Self: Sized, F: Fn(u32) -> bool
            {
                for &x in self {
                    if f(x) {
                        return true;
                    }
                }
                false
            }
        }

        let v = vec![1u32, 2, 3];
        let b = v.any(|x| x == 3);
        println!("{:?}", b);
    }

    #[test] 
    fn function_impl_fns(){
        fn call<F>(closure: F) -> i32 
        where F: Fn(i32) -> i32 {
            closure(1)
        }
        fn counter(i: i32) -> i32 { i + 1}
        
        // 函数指针当作闭包参数传入call函数，代码可以正常编译运行，这是因为此函数
        // 指针counter也实现了Fn

        let result = call(counter);
        assert_eq!(2, result);
    }

    #[test]
    /// triat动态分发
    fn closure_as_para_dyn_dispatch_example(){
        trait Any {
            // f: &（Fn(u32) ->bool) 也是可以的
            fn any(&self, f: Box<dyn Fn(u32)-> bool>) -> bool;
        }
        impl Any for Vec<u32> {
            fn any(&self, f: Box<dyn Fn(u32)-> bool>) -> bool {
                for &x in self {
                    if f(x) {
                        return true;
                    }
                }
                false
            }
        }

        let v = vec![1, 2, 3];
        let b = v.any(Box::new(|x| x == 3));
        println!("{}", b);
    }

    #[test]
    fn closure_as_return_value() {
        // 这的闭包指定的是Fn，可被多次调用
        fn square() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|i| i*i)
        }
        let square = square();
        assert_eq!(4, square(2));
        assert_eq!(9, square(3));
    }
    // 如果希望只调用下一是不是可以直接指定FnOnce？
    #[test]
    fn closure_as_return_fnonce(){
        // fn square() -> Box<dyn FnOnce(i32) -> i32> {
        fn square() -> impl FnOnce(i32) -> i32 {
            Box::new(|i| i * i)
        }
        let square = square();
        assert_eq!(4, square(2));
        // assert_eq!(9, square(3)); 
    }
    #[test]
    fn advance_closure_example(){
        use std::fmt::Debug;
        trait DoSomething<T> {
            fn do_sth(&self, value: T);
        }
        impl<'a, T: Debug> DoSomething<T> for &'a usize {
            fn do_sth(&self, value: T ){
                println!("{:?}", value);
            }
        }

        fn foo(b: Box<dyn for<'f> DoSomething<&'f usize>>){
            let s : usize = 10;
            b.do_sth(&s);
        }
        let x = Box::new(&2usize);
        foo(x);
    }

    #[test]
    fn advance_closure_example1(){
        struct Pick<F> {
            data: (u32, u32),
            func: F,
        }

        impl<F> Pick<F> 
            where F: for<'f> Fn(&'f (u32, u32)) -> &'f u32 {
            fn call(&self) -> &u32{
                (self.func)(&self.data)
            }
        }

        fn max(data: &(u32, u32)) -> &u32 {
            if data.0 > data.1 {
                &data.0 
            }else {
                &data.1
            }
        }

        let elm = Pick{ data: (3, 1), func: max};
        println!("{}", elm.call());
    }

    #[test]
    fn define_inner_iterator(){
        trait InIterator<T: Copy> {
            fn each<F: Fn(T) -> T> (&mut self, f: F);
        }
        impl <T: Copy> InIterator<T> for Vec<T> {
            fn each<F: Fn(T) -> T> (&mut self, f: F) {
                let mut i = 0;
                while i < self.len() {
                    self[i] = f(self[i]);
                    i += 1;
                }
            }
        }

        let mut v = vec![1, 2, 3];
        v.each(|i| i * 3);
        println!("{:?}", v);
    }

    #[test]
    fn test_for_example() {
        let v = vec![11, 2, 3, 4, 5, 6, 7];
        for i in v.iter() {
            println!("{}", i);
        }

        {
            let mut _iterator = v.iter();
            loop {
                match _iterator.next() {
                    Some(i) => {
                        println!("{}",i);
                    }
                    None => break,
                }
            }
        }
        for i in v.iter() {
            println!("{}", i);
        }
    }


    #[test]
    fn define_iterator_example1(){
        trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        struct Counter {
            count : usize,
        }
        impl Iterator for Counter {
            type Item  = usize;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                    Some(self.count)
                }else {
                    None
                }
            }
        }

        let mut counter = Counter{ count: 0};

        while let Some(i) = counter.next(){
            println!("{}", i);
        }
    }

    #[test]
    fn test_size_hint() {
        let a = [1, 2, 3];
        let mut iter = a.iter();
        while let Some(_) = iter.next() {
            println!("{:?}", iter.size_hint())
        }
    }

    #[test]
    fn append_string() {
        let mut message = "Hello".to_string();
        message.extend(&[' ', 'R', 'u', 's', 't']);
        println!("{}", message);
    }

    #[test]
    fn test_slice() {
        // 声明了slice类型的数组，该类型的数组使用for循环时，并不能自动转换为迭代器，
        // 因为并没有为[T]类型实现IntoIterator，而只是为&'a [T]和&'a mut[T]类型实现了
        // IntoIterator，相应的into_iter方法内部实现也分别调用了iter和iter_mut方法，
        // 也就是说for循环中使用&arr可以自动转换为迭代器， 而无须显式地调用iter方法。 
        // 用iter或iter_mut方法可以将slice类型的数组转换为Iter或IterMut迭代器。
        let arr = [1, 2, 3, 4, 5];
        for i in &arr{
            println!("{}", i);
        }
        println!("{:?}", arr);

        let mut arr = [1, 2, 3, 4, 5];
        for i in arr.iter_mut() {
            *i += 1;
        }
        println!("{:?}", arr);
    }

    #[test]
    fn test_map() {
        let a = [1, 2, 3, 4];
        let mut iter = a.into_iter().map(|x| x * 2);
        while let Some(i) = iter.next() {
            println!("{}", i);
        }
    }

    #[test]
    fn test_iter_example() {
        let arr1 = [1, 2, 3, 4, 5, 6];
        let c1 = arr1.iter().map(|x| 2 * x).collect::<Vec<i32>>();
        println!("c1 = {:?}", c1);

        let arr2 = ["1", "2", "3", "4", "j"];
        let c2 = arr2.iter().filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>();
        println!("c2 = {:?}", c2);

        let arr3 = ["a", "b", "v"];
        for (index, val) in arr3.iter().enumerate() {
            println!("index : {}, val: {}", index, val);
        }
    }

    #[test]
    fn test_example(){
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        let ret = a.iter().any(|x| *x != 2);
        println!("{}", ret);
        let sum = a.iter().fold(0, |acc, x| acc + x);
        let sum1 = b.into_iter().fold(0, |acc, x| acc + x);
        println!("sum = {}", sum);
        println!("sum1 = {}", sum1);
        
    }

    #[test]
    fn test_collect() {
        use std::iter::FromIterator;
        #[derive(Debug)]
        struct MyVec(Vec<i32>);
        impl MyVec {
            pub fn new() -> Self {
                Self(Vec::new())
            }
            pub fn add(&mut self, elem: i32) {
                self.0.push(elem);
            }
        }

        impl FromIterator<i32> for MyVec {
            fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
                let mut c = MyVec::new();
                for i in iter {
                    c.add(i);
                }
                c 
            }
        }

        let iter = (0..5).into_iter();
        let c = MyVec::from_iter(iter);
        println!("{:?}", c);
        let c = (0..5).into_iter();
        let c: MyVec = c.collect();
        println!("{:?}", c);
        let c = (0..5).into_iter();
        let c = c.collect::<MyVec>();
        println!("{:?}", c);
    }
}

mod myiter {
    #[derive(Debug, Clone)]
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    pub struct Step<I> {
        iter: I,
        skip: usize,
    }

    impl<I> Iterator for Step<I> 
        where I: Iterator,
    {
        type Item = I::Item;
        fn next(&mut self) -> Option<I::Item> {
            let elt = self.iter.next();
            if self.skip > 0 {
                self.iter.nth(self.skip-1);
            }
            elt
        }
    }

    pub fn step<I>(iter: I, step: usize) -> Step<I> 
        where I: Iterator,
    {
        assert!(step != 0);
        Step{
            iter: iter,
            skip: step - 1,
        }
    }
    
    pub trait IterExt: Iterator {
        fn step(self, n: usize) -> Step<Self> 
        where Self: Sized 
        {
            step(self, n)
        }
    }
    impl<T: ?Sized> IterExt for T where  T: Iterator {}

    #[test]
    fn test_ex1() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
        println!("sum = {}", sum);
    }
}

mod myPosition{
    #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
    #[derive(Debug)]
    pub struct Positions<I, F> {
        iter: I,
        f: F,
        count: usize,
    }
    pub fn positions<I, F>(iter: I, f: F) -> Positions<I, F> 
        where I: Iterator,
        F : FnMut(I::Item) -> bool,
    {
        Positions{
            iter,
            f,
            count: 0,
        }
    }

    impl<I, F> Iterator for Positions<I, F> 
        where I: Iterator,
        F: FnMut(I::Item) -> bool,
    {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            while let Some(v) = self.iter.next() {
                let i = self.count;
                self.count = i + 1;
                if (self.f)(v) {
                    return Some(i);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, self.iter.size_hint().1)
        }
    }
    impl<I, F> DoubleEndedIterator for Positions<I, F> 
        where I: DoubleEndedIterator + ExactSizeIterator,
        F: FnMut(I::Item) -> bool,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            while let Some(v) = self.iter.next_back() {
                if (self.f)(v) {
                    return Some(self.count + self.iter.len())
                }
            }
            None
        }
    }

    pub trait Itertools : Iterator {
        fn positions<P>(self, predicate: P) -> Positions<Self, P> 
            where Self: Sized,
            P: FnMut(Self::Item) -> bool,
            {
                positions(self, predicate)
            }
    }
    impl<T: ?Sized> Itertools for T where T: Iterator {}

    #[test]
    fn test_ex(){
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let r = data.iter().positions(|v| v % 3 == 0);
        // println!("r = {:?}", r);
        let rev_r = data.iter().positions(|v| v %3 == 0).rev();
        for i in r { 
            println!("{:?}", i); // 2 5 8
        }
        println!();
        for i in rev_r {
            println!("{:?}", i); // 8 5 2
        }
    }
    

}

mod programming_rust {
    #[test]
    fn test_array(){
        let mut arr = [3, 2, 1];
        println!("{:?}", arr); 
        arr.sort();
        println!("{:?}", arr);
    }
    #[test]
    fn test_fnonce () {
        let s = "hello".to_string();
        let c = || s ;
        c();
        // c();
    }
}