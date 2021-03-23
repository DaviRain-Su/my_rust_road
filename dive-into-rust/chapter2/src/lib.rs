pub mod davirain {
    use std::rc::Rc;
    use std::sync::Arc;
    use std::borrow::{Cow, BorrowMut};
    use std::thread::spawn;

    ///# 变量和类型
            ///
            /// ## 变量声明
            ///
            /// ```
            /// let variable :i32 = 100;
            /// ```
            /// - 语法分析更加容易
            /// - 方便引入类型推导功能
            ///     - Rust的变量声明的一个重要特点是：要声明的变量前置，对他的类型描述后置。
            ///     - 因为在变量声明语句中，最重要的是变量本身，而类型其实是个附属的额外描述，并非必不可少的部分。
            ///     - 如果我们可以通过上下文环境由编译器自动分析出这个变量的类型，那么这个类型描述完全可以省略不写。
            /// - 模式解构 pattern destructure
            ///
            /// Rust中声明的变量缺省是"只读的"
            /// ```
            /// let x = 5;
            /// x = 10; // re-assignment of immutable variable 'x'
            /// ```
            ///
            /// 如果我们需要让变量可写的，需要使用mut关键字。
            /// let 语句在此处引入了一个模式解构，我们不能把let mut视为一个组合，而应该将mut x视为一个组合。
            /// mut x是一个模式，
            ///
            /// 在Rust中，一般把声明的局部变量并初始化的语句称为"变量绑定"
            ///
            /// ```
            /// let mut x = 5;
            /// x = 10;
            /// ```
            ///
            /// 模式解构
            ///
            /// ```
            /// #[derive(Debug)]
            /// struct Point {
            ///     x: i32,
            ///     y: i32,
            /// }
            /// let p = Point{ x: 1, y: 2}; //在Rust中，一般把声明的局部变量并初始化的语句称为"变量绑定"
            /// //强调的"绑定"
            /// let (mut a, mut b) = (1, 2); //对tuple解构
            /// println!("a = {}, b = {}", a, b);
            /// let Point {x : ref a, y: ref b} = p; //对结构体解构
            /// println!("x = {}, y = {}", x, y);
            /// ```
            ///
            /// Rust中，每个变量必须被合理的初始化之后才能被使用。使用未初始化变量，在Rust中是不可能出现的。
            /// ```
            /// let x: i32;
            /// println!("{}", x); // use of possibly uninitialized variable 'x'
            /// ```
            /// 编译器会做一个静态分析，确保变量在使用前一定被初始化
            /// ```
            /// fn test(condition: bool) {
            ///     let x: i32;  //声明x， 不需要使用mut修饰
            ///     if condition {
            ///         x = 1; //初始化x，不需要x是mut的，因为这里是初始化，不是修改
            ///         println!("{}", x);
            ///     }
            ///     //如果条件不满足，x是没有初始化
            ///     // 这里只要不使用x就没有问题
            /// }
            /// ```
            ///
            /// 类型没有"默认构造函数"， 变量没有"默认值"
            ///
            /// 对let x: i32, 如果没有显式赋值，他就是没有被初始化，不能想当然的以为初始化了。
            ///
            /// Rust里的合法标识符（变量名，函数名，triat名），必须是数字、字母、下划线组成，且不能以数字开头。
            ///
            /// RUST将来会允许其他unicode字符做标识符，只是目前这个功能的优先级不高，还没有最终确定下来。
            ///
            /// raw identifer功能， 提供一个特殊的语法，r#self，让用户以关键字作为普通标识符。
            ///
            /// Rust中的下划线是一个特殊的标识符，在编译器内部他是被特殊处理的。
            ///
            /// ```
            /// let _ = "hello";
            /// println!("{}", _); //不能在表达式中使用下划线来作为普通变量使用，下划线表达的含义是
            /// //忽略这个变量绑定， 后面再也不会用到了
            /// ```
            /// ### 变量遮蔽
            ///
            /// Rust允许在同一个代码块中声明同样名字的变量。如果这样做，后面声明的变量会将前面声明的变量"遮蔽"
            ///
            /// ```
            ///
            /// let x = "hello";
            /// println!("x = {}", x);
            ///
            /// let x = 5; //如果不使用let关键字，这条语句就是对x的重新绑定（重新赋值）
            /// //而有了let关键字，就是又声明了一个新变量，只是它的名字和前面一个变量相同而已。
            /// //这两个x代表的内存空间完全不同，类型也完全不同，它们实际上是两个不同的变量
            /// //没有办法再去访问前一个x了。
            ///
            ///
            /// println!("x = {}", x);
            ///
            /// //需要在同一个函数内部把一个变量转换为另一个类型的变量。但是又不想给它们起不同的名字。
            /// //在同一个函数内部，需要修改一个变量绑定的可变性。
            ///
            /// let mut v = Vec::new();
            /// v.push(1);
            /// v.push(2);
            /// v.push(3);
            ///
            /// let v = v;
            /// for i in &v {
            ///     println!("{}", i);
            /// }
            ///
            /// //如果一个变量是不可变的，可以通过变量遮蔽来创建一个新的、可变的同名变量。
            /// // 一个不可变绑定依然是一个变量，虽然我们没办法通过这个变量绑定修改变量的值，但是我们重新使用可变绑定之后，还是有机会修改的。
            /// // 这样做并不会产生内存安全的问题，因为我们对这块内存拥有完整的所有权，且此时没有任何其他引用指向这个变量，对这个变量的修改完全是合法的。
            ///
            /// let v = Vec::new();
            /// let mut v = v; //这里是符合内存安全的， 因为我们对这块内存有完整的所有权，且此时没有任何
            /// //其他指向这个变量，对这个变量的修改是完全合法的。
            /// v.push(1);
            /// println!("{:?}", v);
            ///
            /// ```
            ///
            ///  ### 类型推导
            ///
            /// 可以从变量声明的当前语句中获取信息进行推导，而且还能通过上下文信息进行推导。
            ///
            /// ```
            /// let elem = 5u8;
            ///
            /// let mut vec = Vec::new();
            /// vec.push(elem);  // vec: Vec<u8>
            ///
            /// println!("{:?}", vec);
            /// ```
            ///
            /// 只写一部分类型，剩下的部分让编译器去推导
            ///
            /// ```
            /// let player_scores = [
            ///     ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19),
            /// ];
            ///
            /// let players: Vec<_> = player_scores
            ///     .iter()
            ///     .map(|&(player, _score)| {
            ///         player
            ///     }).collect();
            /// println!("{:?}", players);
            /// ```
            ///
            /// 自动类型推导和"动态类型系统"是两回事，Rust依然是静态类型的。一个变量的类型必须在编译
            /// 阶段确定，且无法更改，只是某些时候不需要在源码中显式出来而已，这只是编译器提供的一个辅助
            /// 工具。
            ///
            /// Rust中只允许"局部变量/全局变量"实现类型推导，而函数签名等场景下是不允许的，这是故意
            /// 这样设计的。这是因为局部变量只有局部的影响，全局变量必须当场初始化而函数签名具有全局性
            /// 影响。函数签名如果使用自动类型推导，可能导致某个调用的地方使用方式发生变化，它的参数、
            /// 返回值类型就发生来变化，进而导致远处另一个地方的编译错误，这是设计者不希望看到的情况。
            ///
            /// ### 类型别名
            ///
            /// 使用type关键字给同一个类型起别名
            ///
            /// ```
            /// type Age = u32;
            ///
            /// fn grow(age: Age, year: u32) -> Age {
            ///     age + year
            /// }
            ///
            /// let x : Age = 20;
            /// println!("20 years later : {}", grow(x, 20));
            /// ```
            ///
            /// 类型别名还可以用于泛型场景：
            /// ```
            /// type Double<T> = (T, Vec<T>);
            /// ```
            ///
            /// ### 静态变量
            ///
            /// Rust中可以使用static关键字声明静态变量
            ///
            /// ```
            /// static GLOBAL: i32 = 0;
            /// ```
            /// static 语句也是一个模式匹配，与let语句不同的是，用static声明的变量的生命周期是
            /// 整个程序，从启动到退出。
            ///
            /// static变量的生命周期永远是'static， 它占用的内存空间也不会在执行过程中回收。这也是Rust中唯一声明全局变量的方法。
            ///
            /// 使用全局变量的限制：
            /// - 全局变量必须在声明的时候马上初始化
            /// - 全局变量的初始化必须是编译期可确定的常量，不能包括执行期才能确定的表达式、语句或函数调用
            /// - 带有mut修饰的全局变量，在使用的时候必须使用unsafe关键字
            ///
            /// ```
            /// // 局部变量声明，可以留待后面初始化，只要保证使用前已经初始化即可。
            /// let x;
            /// let y = 1_i32;
            /// x = 2_i32;
            /// println!("{} {}", x, y);
            ///
            /// // 全局变量必须声明的时候初始化，因为全局变量可以写到函数外面，被任意一个函数使用。
            /// static G1: i32 = 3;
            /// println!("{}", G1);
            ///
            /// static mut G2: i32 = 4; // 可变全局变量无论读写都必须用unsafe修饰
            /// unsafe  {
            ///     G2 = 5;
            ///     println!("{}", G2);
            /// }
            /// // 全局变量的内存不是分配在当前函数栈上，函数退出的时候，并不会销毁全局变量占用的内存空间，程序退出才会回收。
            /// ```
            ///
            /// Rust禁止在声明static变量的时候调用普通函数，或者利用语句块调用其他非const代码
            ///
            /// ```
            /// static array: [i32; 3] =  [1, 2, 3];
            /// static vec: Vec<i32> = {let mut v = Vec::new(); v.push(1); v }; // error
            ///
            /// ```
            /// 调用const fn 是允许的，const_fn 是编译期执行的
            ///
            /// Rust不允许用户在main函数之前或者之后执行自己的代码。
            /// 所以比较复杂的static 变量的初始化一般需要lazy方式，在第一次使用的时候初始化。
            /// 在Rust中，如果用户需要使用比较复杂的全局变量初始化，推荐使用lazy_static库。
            ///
            /// ```
            /// #![fetaure(const_fn)]
            /// use std::sync::atomic::AtomicBool;
            /// static FLAG: AtomicBool = AtomicBool::new(true);
            ///
            ///
            /// ```
            /// ### 常量
            ///
            /// 在Rust中可以使用const关键字做声明。
            ///
            /// ```
            /// const GLOBAL: i32 = 0;
            /// ```
            ///
            /// 使用const声明是常量，而不是变量。因此一定不允许使用mut关键字修饰这个变量绑定，这是语法错误。
            /// 常量的初始化表达式也一定要是一个编译器常量，不能是运行期的值。
            /// 它与static变量的最大区别在于：编译器并不一定会给const常量分配内存空间，在编译过程中，它很可能被内联优化。
            /// 以const声明一个常量，也不具备类似let语句的模式匹配功能

    /// ### bool
    #[test]
    fn test_bool_type() {
        let x = true;

        let y : bool = !x;
        println!("x = {}, y = {}", x, y);

        let z = x && y;
        println!("x && y = {}", z);

        let z = x || y;
        println!("x || y = {}", z);

        let z = x & y;
        println!("x & y = {}", z);

        let z = x | y;
        println!("x | y = {}", z);

        let z = x ^ y;
        println!("x ^ y  = {}", z);
    }

    /// logical op
    #[test]
    fn test_logical_op() {
        fn logical_op(x: i32, y: i32) {
            let z = x < y;
            println!("z = {}", z);
        }
        logical_op(1, 2);
    }

    /// char type
    #[test]
    fn test_char_type() {
        let love = 'å';
        println!("lover = {}", love);

        let c1 = '\n';
        let c2 = '\x7f';
        let c3 = '\u{7FFF}';
        println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);

        let x : u8 = 1;
        let y: u8 = b'A';
        let s: &[u8; 5] = b"hello";
        let r: &[u8; 14] = br#"hello \n wrold"#;
        println!("x = {}", x);
        println!("y = {}", y);
        println!("s = {:?}", s);
        println!("r = {:?}", r);
    }

    #[test]
    fn test_integer_type() {
        let var1: i32 = 32;
        let var2: i32 = 0xFF;
        let var3: i32 = 0o55;
        let var4: i32 = 0b1001;
        println!("var1 = {}, var2 = {}, var3 = {}, var4 = {}", var1, var2, var3, var4);

        let var5 = 0x_1234_ABCD;
        println!("var5 = {}", var5);
        let var6 = 123usize;
        println!("var6 = {}", var6);
        let var7 = 0xffu8;
        println!("var7 = {}", var7);
        let var8 = 32;
        println!("var8 = {}", var8);

        let _x: i32 = 9;
        println!("9 power 3 = {}", 9_i32.pow(3));

        let x = 10;
        let y = x * x;
        println!("y = {}", y);
    }

    #[test]
    fn test_int_overflow() {
        let _m: i8 = 120;
        let _n: i8 = 120;
        // println!("m + n = {}", m + n); // this arithmetic operation will overflow

        let i = 100_i8;
        println!("checked {:?}", i.checked_add(i));
        println!("saturating {:?}", i.saturating_add(i));
        println!("wrapping {:?}", i.wrapping_add(i));

        use std::num::Wrapping;
        let big = Wrapping(std::u32::MAX);
        let sum = big + Wrapping(2_u32);
        println!("sum = {}", sum.0);
    }

    #[test]
    fn test_float() {
        let f1 = 123.0f64;
        let f2 = 0.1f64;
        let f3 = 0.1f32;
        let f4 = 12E+99_f64;
        let f5 : f64 = 3.0;
        println!("f1 = {}, f2 = {}, f3 = {}, f4 = {}, f5 = {}", f1, f2, f3, f4, f5);

        let x = 1.0f32 / 0.0;
        let y = 0.0f32 / 0.0;
        println!("x = {}, y = {}", x, y);

        let inf = std::f32::INFINITY;
        println!("{}, {}, {}", inf * 0.0, 1.0 / inf, inf / inf);

        let nan = std::f32::NAN;
        println!("{}, {}, {}", nan < nan, nan > nan, nan == nan);
    }

    #[test]
    fn test_pointer_type() {
        let mut x = 12;
        let x_box = Box::new(x);
        println!("x = {}, x_box = {}", x, x_box);
        {
            let x_ref = &x;
            println!("x_ref = {}", x_ref);
            {
                let x_mut_ref = &mut x;
                println!("x mut ref = {}", x_mut_ref);
            }

        }
        {
            let raw_x = &x as *const i32;
            let raw_mut_x = &mut x as *mut i32;
            unsafe {
                println!("raw_x = {:?}, raw_mut_x = {:?}", *raw_x, *raw_mut_x);
            }
        }

        let x_rc = Rc::new(x);
        println!("x_rc = {}", x_rc);

        let x_arc = Arc::new(x);
        println!("x_arc = {}", x_arc);
        let mut handle = Vec::new();
        for _ in 0..5 {
            let clone_x = x_arc.clone();
            let ret = spawn(move || {
                println!("clone_x = {}", clone_x);
            });
            handle.push(ret);
        }

        for i in handle {
            i.join().unwrap();
        }

        let s = String::from("hello world");
        let mut x_cow = Cow::from(s);
        x_cow.borrow_mut();

    }

    #[test]
    fn type_transfer() {
        let var1: i8 = 41;
        let var2: i16 = var1 as i16;
        println!("var1 = {}", var1);
        println!("var2 = {}", var2);

        let i = 42;
        let p = &i as *const i32 as *mut i32;
        println!("{:p}", p);

        let a = 12;
        let b = a as i64;
        println!("a = {}, b = {}", a, b);

        #[derive(Debug)]
        enum A {
            C,
            D,
        }

        let c = A::C;
        let _c = A::D;
        println!("c = {:?}", c);
        let d = c as u32;
        println!("d = {:?}", d);


    }
}