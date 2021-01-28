pub mod davirain {

/// # 动态大小类型： str
/// 
/// ### 探索&str的组成
/// 
/// Basic usage:
/// 
/// ```
/// fn str_compose() {
///     let str = "Hello Rust";
///     let ptr = str.as_ptr();
///     let len = str.len();
///     println!("{:p}", ptr); // address pointer
///     println!("{:?}", len);
/// }
/// 
/// str_compose();
/// ```
pub fn str_compose() {
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}", ptr);
    println!("{:?}", len);
}

/// # 动态大小类型: `[T]`
/// 
/// ### 探索数组
/// 
/// Error usage:
/// 
/// ```
///  //Error: `[u32]` does not have a constant size know at compile-time
/// fn reset(mut arr: [u32; 5]){
///     arr[0] = 5;
///     arr[1] = 4;
///     arr[2] = 3;
///     arr[3] = 2;
///     arr[4] = 1;
///     println!("reset arr {:?}", arr);
/// }
/// 
/// let arr: [u32; 5] = [1, 2, 3, 4, 5];
/// reset(arr);
/// println!("origin arr {:?}", arr);
/// ```
/// 
/// Right usage 1: 指定固定长度
/// 
/// ```
/// fn reset(mut arr: [u32;5]) {
///     arr[0] = 5;
///     arr[1] = 4;
///     arr[2] = 3;
///     arr[3] = 2;
///     arr[4] = 1;
///     println!("reset arr {:?}", arr);
/// }
/// let arr : [u32; 5] = [1, 2, 3, 4, 5];
/// reset(arr);
/// println!("origin arr {:?}", arr);
/// ```
/// 
/// Right usege 2: 使用胖指针
/// 
/// ```
/// fn reset(arr: &mut [u32]) {
///     arr[0] = 5;
///     arr[1] = 4;
///     arr[2] = 3;
///     arr[3] = 2;
///     arr[4] = 1;
///     println!("array length {:?}", arr.len());
///     println!("reset arr {:?}", arr);
/// }
/// let mut arr = [1, 2, 3, 4, 5];
/// println!("reset before: origin array {:?}", arr);
/// {
///     let mut_arr : &mut[u32] = &mut arr;
///     reset(mut_arr);
/// }
/// println!("reset after: origin array {:?}", arr);
/// ```
pub fn reset(arr: &mut [u32]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;
    println!("array length {:?}", arr.len());
    println!("reset array {:?}", arr);
}

/// # 动态大小类型：比较`&[u32;5]`和`&mut [u32]`的空间占用
/// 
/// Base usage:
/// 
/// ```
/// fn compare_size() {
///     assert_eq!(std::mem::size_of::<&[u32;5]>(), 8);
///     assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);
/// }
/// compare_size();
/// ```
pub fn compare_size(){
    use std::mem::size_of;
    assert_eq!(size_of::<&[u32;5]>(), 8);
    assert_eq!(size_of::<&mut [u32]>(), 16);
}

/// # 零大小类型：
/// 
/// Base usage: 
/// 
/// ```
/// fn zero_size() {
///     enum Void{}
///     struct Foo;
///     struct Baz {
///         foo: Foo,
///         qux: (),
///         baz: [u8; 0],
///     }
///     assert_eq!(std::mem::size_of::<()>(), 0);
///     assert_eq!(std::mem::size_of::<Foo>(), 0);
///     assert_eq!(std::mem::size_of::<Baz>(), 0);
///     assert_eq!(std::mem::size_of::<Void>(), 0);
///     assert_eq!(std::mem::size_of::<[(); 10]>(), 0);
/// }
/// 
/// zero_size();
/// ```

pub fn zero_size() {
    use std::mem::size_of;
    enum Void {}
    
    struct Foo;
    struct Baz {     
        foo: Foo,
        qux: (),
        baz: [u8; 0],
    }
    assert_eq!(size_of::<()>(), 0);
    assert_eq!(size_of::<Foo>(), 0);
    assert_eq!(size_of::<Baz>(), 0);
    assert_eq!(size_of::<Void>(), 0);
    assert_eq!(size_of::<[(); 10]>(), 0);
}

/// # 零大小类型： 应用
/// 
/// Base  usage: 利用单元类型查看值的类型
/// 
/// ```
/// let v: Vec<()> = vec![(); 10];
/// // Error: expected(), found struct 'std::vec::Vec'
/// ```
/// 
/// Base usage: 零大小循环
/// 
/// ```
/// fn zero_size_loop() {
///     let v : Vec<()>  = vec![(); 10];
///     for i in v {
///         println!("{:?}", i);
///     }
/// }
/// zero_size_loop();
/// ```
pub fn zero_size_loop() {
    let v : Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }
}

/// # 底类型： 应用
/// 
/// Base usage
/// 
/// ```
/// #![feature(never_type)]
/// fn foo() -> ! {
///     // do something
///     loop { println!("jh"); }
/// }
/// let i = if false {
///     foo();
/// }else {
///     100
/// };
/// assert_eq!(i, 100);
/// ```
pub fn bottom_type() {
    let i = if false {
        foo();
    } else {
        100
    };
    assert_eq!(i, 100);

    fn foo() -> ! {
        // do something
        loop { println!("jh"); }
    }
}


/// # 底类型： 空Enum
/// 
/// Base usage : 仅仅是为了演示， 编译并不能通过
/// 
/// ```
/// //使用enum Void {} 就可以避免处理Err的情况，但当前Rust并支持该用法
/// fn void_enum() {
///     enum Void {} 
///     let res: Result<u32, Void> = Ok(0);
///     let Ok(num) = res;
/// }
/// ```
pub fn void_enum() {
    // Just for show

    // enum Void {}
    // let res: Result<u32, Void> = Ok(0);
    // let Ok(num) = res;
}

/// # 类型推导
/// 
/// Base uage: 正常推导
/// 
/// ```
/// fn sum(a: u32, b: i32) -> u32 {
///     a + (b as u32)
/// }
/// 
/// fn infer_demo() {
///     let a = 1;
///     let b = 2;
///     assert_eq!(sum(a, b), 3); //a推导为u32, b推导为i32
///     let elem = 5u8;
///     let mut vec = Vec::new();
///     vec.push(elem);// vec 类型会自动推断为Vec<u8>
///     assert_eq!(vec, [5]);
/// }
/// infer_demo();
/// ```
/// Base usage: 无法推导的情况
/// 
/// ```
/// let x = "1";
/// println!("{:?}", x.parse().unwarp());
/// ```
/// 
/// Base usage: 解决无法推导的情况
/// 
/// ```
/// let x = "1";
/// println!("{:?}", x.parse::<i32>().unwarp());
/// ```
/// 
/// Base usage: 另一种类型无法推导的情况
///
/// ```
/// let a = 0;
/// let a_pos = a.is_positive(); 
/// // error : no mathod named 'is_positived' found for type '{integer}'
/// // in the current scope
/// ```
/// 
pub fn infer_demo() {
    let a= 1;
    let b = 2;
    assert_eq!(sum(a, b), 3);
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    assert_eq!(vec, [5]);

}
fn sum(a: u32, b: i32) -> u32 {
    a + (b as u32)
}

/// # 泛型
/// 
/// Base usage: 自定义泛型函数
/// 
/// ```
/// fn foo_generic<T>(x: T) -> T {
///     x
/// }
/// 
/// assert_eq!(foo_generic(1), 1);
/// assert_eq!(foo_generic("hello"), "hello");
/// ```
/// 
/// Base usage: 自定义泛型结构体
/// 
/// ```
/// struct Point<T> { x: T, y: T }
/// ```
/// 
/// Base usage: 'foo_generic<T>'静态分发等价于下面代码
///
/// ```
/// fn foo_1(x: i32) -> i32 { 
///     x
/// }
/// 
/// fn foo_2(x: &'static str) -> &'static str {
///     x
/// }
/// foo_1(1);
/// foo_2("2");
/// ```
pub fn foo_generic<T>(x: T) -> T {
    x
}

/// # 泛型： 为泛型结构体实现方法
/// 
/// Base usage: 
/// 
/// ```
/// fn impl_method() {
///     #[derive(Debug, PartialEq)]
///     struct Point<T> { x: T, y: T}
///     impl <T> Point<T> {
///         fn new(x: T, y: T) -> Self {
///             Point{ x: x, y: y}
///         }    
///     }
///     let point1 = Point::new(1, 2);
///     let point2 = Point::new("1", "2");
///     assert_eq!(point1, Point{x: 1, y: 2});
///     assert_eq!(point2, Point{x: "1", y: "2"});
/// }
/// impl_method();
/// ```
pub fn impl_method() {
    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T, 
        y: T,
    }
    impl <T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point{ x, y }
        }
    }

    let point1 = Point::new(1, 2);
    let point2 = Point::new("1", "2");
    assert_eq!(point1, Point{x: 1, y: 2});
    assert_eq!(point2, Point{x: "1", y: "2"});
}

/// # 泛型： 返回值自动推导
/// 
/// Base usage: 
/// 
/// ```
/// 
/// fn infer_generics() {
///     #[derive(Debug, PartialEq)]
///     struct Foo(i32);
///     #[derive(Debug, PartialEq)]
///     struct Bar(i32, i32);
///     trait Inst {
///         fn new(i: i32) -> Self;
///     }
///     impl Inst for Foo {
///         fn new(i: i32) -> Foo {
///             Foo(i)
///         }    
///     }
///     impl Inst for Bar {
///         fn new(i: i32) -> Bar {
///             Bar(i, i + 10)
///         }
///     }
///     fn foobar<T: Inst>(i: i32) -> T {
///         T::new(i)
///     }
///     
///     let f: Foo = foobar(10);
///     assert_eq!(f, Foo(10));
///     let b: Bar = foobar(20);
///     assert_eq!(b, Bar(20, 30));
/// }
/// infer_generics();
/// ```
pub fn infer_generics() {
    #[derive(Debug, PartialEq)]
    struct Foo(i32);

    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);

    trait Inst {
        fn new(i: i32) -> Self;
    }
    impl Inst for Foo {
        fn new (i: i32) -> Foo {
            Foo(i)
        }
    }

    impl Inst for Bar {
        fn new (i: i32) -> Bar {
            Bar(i, i + 10)
        }
    }

    fn foobar<T: Inst>(i: i32) -> T {
        T::new(i)
    }
    let f : Foo = foobar(10);
    assert_eq!(f, Foo(10));
    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));
    // let s = foobar(40);
}

/// # triat: 关联类型
/// 
/// Base usage: 
/// 
/// ```
/// pub fn associated_type() {
///     #[derive(Debug, PartialEq)]
///     struct Foo(i32);
///     #[derive(Debug, PartialEq)]
///     struct Bar(i32, i32);
///     trait Inst {
///         type F;
///         type B;
///         fn new_foo(i: i32) -> Self::F;
///         fn new_bar(i: i32) -> Self::B;
///     }
/// 
///     struct FooBar;
///     impl Inst for FooBar {
///         type F = Foo;
///         type B = Bar;
///         fn new_foo(i: i32) -> Foo {
///             Foo(i)
///         }
///         fn new_bar(i: i32) -> Bar {
///             Bar(i, i + 10)
///         }
///     }
///     let f: Foo = FooBar::new_foo(10);
///     assert_eq!(f, Foo(10));
///     let b : Bar = FooBar::new_bar(20);
///     assert_eq!(b, Bar(20, 30));
/// }
/// associated_type();
/// ```
pub fn associated_type() {
    #[derive(Debug, PartialEq)]
    struct Foo(i32);
    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);
    trait Inst {
        type F;
        type B;
        fn new_foo(i: i32) -> Self::F;
        fn new_bar(i: i32) -> Self::B;
    }
    struct FooBar;
    impl Inst for FooBar {
        type F = Foo;
        type B = Bar;
        fn new_foo(i: i32) -> Foo {
            Foo(i)
        }
        fn new_bar(i: i32) -> Bar {
            Bar(i, i + 10)
        }
    }
    let f : Foo = FooBar::new_foo(10);
    assert_eq!(f, Foo(10));
    let b: Bar = FooBar::new_bar(20);
    assert_eq!(b, Bar(20, 30));
}


/// # trait: 泛型triat
/// 
/// Base usage: 
/// 
/// ```
/// fn generics_trait() {
///     pub trait Add<RHS, Output> {
///         fn my_add(self, rhs: RHS) -> Output;
///     }
///     impl Add<i32, i32> for i32 {
///         fn my_add(self, rhs: i32) -> i32 {
///             self + rhs
///         }
///     }
///     impl Add<u32, i32> for u32 {
///         fn my_add(self, rhs: u32) -> i32 {
///             (self + rhs) as i32
///         }
///     }
///     let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
///     let x: i32 = a.my_add(b);
///     let y: i32 = c.my_add(d);
///     assert_eq!(x, 3i32);
///     assert_eq!(y, yi32);
///        
/// }
/// gernerics_trat();
/// ```
pub fn generics_trait() {
    trait Add<RHS, Output>  {
        fn my_add(self, rhs: RHS) -> Output;
    }
    impl Add<i32, i32> for i32 {
        fn my_add(self, rhs: i32) -> i32 {
            self + rhs
        }
    }
    impl Add<u32, i32> for u32 {
        fn my_add(self, rhs: u32) -> i32 {
            (self + rhs) as i32
        }
    }

    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x: i32 = a.my_add(b);
    let y: i32 = c.my_add(d);
    assert_eq!(x, 3i32);
    assert_eq!(y, 7i32); 
}

/// # trait : 关联类型String用法
/// 说明：
/// 在rust标准库中，‘String‘类型实现'Add trait', 使用了关联类型
/// 如下所示：
/// ```
/// impl Add<&str> for String {
///     type Output = String;
///     fn add(mut self, other: &str) -> String {
///         self.push_str(other);
///         self
///     }
/// }
/// ```
/// 
/// Base usage: 
/// 
/// ```
/// let a = "hello";
/// let b = ", world";
/// let c = a.to_string() + b;
/// println!("{:?}", c);// "hello, world"
/// ```
pub fn string_add() {
    let a = "hello";
    let b = ", world";
    let c = a.to_string() + b;
    println!("{:?}", c); //"hello, world"
}

/// # trait 一致性
/// 
/// Base usage: 错误的方式，违反孤儿规则
/// 
/// ```
/// 
/// use std::ops::Add;
/// impl Add<u64> for u32 {
///     type Output = u64;
///     fn add(self, other: u64) -> Self::Output {
///         （self as u64) + other
///     }
/// }
/// let a = 1u32;
/// let b = 2u64;
/// assert_eq!(a + b, 3);
/// ```
/// 
/// Base usage: 正确的方式之重定义trait 
/// 
/// ```
/// trait Add<RHS=Self> {
///     type Output;
///     fn add(self, rhs: RHS) -> Self::Output;
/// }
/// impl Add<u64> for u32 {
///     type Output = u64;
///     fn add(self, other: u64) -> Self::Output {
///         (self as u64) + other
///     }
/// }
/// let a = 1u32;
/// let b = 2u64;
/// assert_eq!(a.add(b), 3);
/// ```
/// Base usage: 正确的方式之重定义类型
/// 
/// ```
/// use std::ops::Add;
/// #[derive(Debug)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// impl Add for Point {
///     type Output = Point;
///     fn add(self, other: Point) -> Point {
///         Point {
///             x: self.x + other.x,
///             y: self.y + other.y,
///         }    
///     }
/// }
/// 
/// // Point{ x: 3, y: 3}
/// println!("{:?}", Point {x: 1, y: 0} + Point(x: 2: y: 3));
/// 
/// ```
pub fn override_op() {
    trait Add<RHS=Self> {
        type Output;
        fn add(self, rhs: RHS) -> Self::Output;
    }
    impl Add<u64> for u32 {
        type Output = u64;
        fn add(self, other: u64) -> Self::Output {
            (self as u64) + other
        }
    }

    let a = 1u32;
    let b  = 2u64;
    assert_eq!(a.add(b), 3);
}

/// # trait继承
/// 
/// Base usage: 
/// 
/// ```
/// 
/// trait Page {
///     fn set_page(&self, p: i32) {
///         println!("Page Default: 1");
///     }
/// }
/// 
/// trait PerPage {
///     fn set_perpage(&self, num: i32) {
///         println!("Per Page Default : 10");
///     }
/// }
/// 
/// trait Paginate: Page + PerPage {
///     fn set_skip_page(&self, num: i32) {
///         println!("Skip Page: {:?}", num);
///     }
/// }
/// impl <T: Page + PerPage> Paginate for T {} 
/// struct MyPaginate { page: i32 }
/// impl Page for MyPaginate {} 
/// impl PerPage for MyPaginate {}
/// 
/// let my_paginate = MyPaginate { page: 1};
/// my_paginate.set_page(2);
/// my_paginate.set_perpage(100);
/// my_paginate.set_skip_page(12);
/// ```
pub fn trait_inherit() {
    trait Page {
        fn set_page(&self, p: i32) {
            println!("Page Default : 1");
        }
    }

    trait PerPage {
        fn set_perpage(&self, num: i32 ) {
            println!("Per Page Default : 10");
        }
    }

    trait Paginate : Page + PerPage {
        fn set_skip_page(&self, num: i32) {
            println!("Skip Page : {:?}", num);
        }
    }

    impl <T: Page + PerPage> Paginate for T {}

    struct MyPaginate { 
        page: i32 
    }

    impl MyPaginate {
        fn new(num: i32) -> MyPaginate {
            MyPaginate{ page: num}
        }
    }

    impl Page for MyPaginate {
        fn set_page(&self, p: i32) {
            println!("Page is {}", p);
        }
    }
    impl PerPage for MyPaginate {
        fn set_perpage(&self, num: i32) {
            println!("Per Page is {}", num);
        }
    } 

    let my_paginate = MyPaginate::new(12);
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);
}

/// # trait bound 泛型约束
/// 
/// Base usage: 
/// 
/// ```
/// use std::ops::Add;
/// fn sum<T: Add<T, Output=T>> (a: T, b: T) -> T {
///     a + b
/// }
/// 
/// assert_eq!(sum(1u32, 2u32), 3);
/// assert_eq!(sum(1u64, 2u64), 3);
/// ```
pub fn trait_bound() {
    use std::ops::Add;
    fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
        a + b
    }
    
    assert_eq!(sum(1u32, 2u32), 3);
    assert_eq!(sum(1u64, 2u64), 3);
}

/// # 抽象类型:Box 装箱 抽象类型 之 trait对象
///
/// Basic usage:
///
/// ```
/// #[derive(Debug)]
/// struct Foo;
/// trait Bar {
///     fn baz(&self);
/// }
/// impl Bar for Foo {
///     fn baz(&self) {
///         println!("{:?}", self);
///     }
/// }
/// fn static_dispatch<T>(t: &T)
///     where T: Bar {
///     t.baz();
/// }
///
/// fn dynamic_dispatch(t : &Bar) {
///     t.baz();
/// }
/// let foo = Foo;
/// static_dispatch(&foo);
/// dynamic_dispatch(&foo);
/// ```
pub fn trait_object() {
    #[derive(Debug)]
    struct Foo;
    trait Bar {
        fn baz(&self);
    }

    impl Bar for Foo {
        fn baz(&self) {
            println!("{:?}", self);
        }
    }

    fn static_dispatch<T>(t: &T) 
        where T: Bar {
        t.baz();
    }

    fn dynamic_dispatch(t: &Bar) {
        t.baz();
    }

    let foo = Foo;
    static_dispatch(&foo);
    // let foo = Foo;
    dynamic_dispatch(&foo);
}

/// # 抽象类型: impl Trait unbox存在类型 (rust 2018)
///
/// Base usage: 重构第二章中trait的示例
///
/// ```
/// use std::fmt::Debug;
/// trait Fly {
///     fn fly(&self) -> bool;
/// }
/// #[derive(Debug)]
/// struct Duck;
/// #[derive(Debug)]
/// struct Pig;
/// impl FLy for Duck {
///     fn fly(&self) -> bool {
///         true
///     }
/// }
/// impl Fly for Pig {
///     fn fly(&self) -> bool {
///         false
///     }
/// }
/// fn fly_static(s: impl Fly+Debug) -> bool {
///     s.fly()
/// }
/// fn can_fly(s: impl Fly+Debug) -> impl Fly {
///     if s.fly() {
///         println!("{:?} can fly", s);
///     } else {
///         println!("{:?} can't fly" , s);
///     }
///     s
/// }
/// fn dyn_can_fly(s: impl Fly+Debug+'static) -> Box<dyn Fly> {
///     if s.fly() {
///         println!("{:?} can fly", s);
///     } else {
///         println!("{:?} can't fly", s);
///     }
///     Box::new(s)
/// }
/// let pig = Pig;
/// assert_eq!(fly_static(pig), flase); //静态分发
/// let pig = Pig;
/// can_fly(pig); // 静态分发
/// let duck = Duck;
/// assert_eq!(fly_static(duck), true); // 静态分发
/// let duck = Duck;
/// can_fly(duck); //静态分发
/// let duck = Duck;
/// dyn_can_fly(duck);//动态分发
/// ````
///
/// Base usage: 错误示范
///
/// ```
/// use std::ops::Add;
/// // 以下多个参数， 虽然同时指定了impl Add<T, Output=T> 类型。
/// // 但是它们并不是同一类型，因为这是抽象类型
/// // 所以编译会出错，: mismatched types
/// fn sum<T>(a: impl Add<T, Output=T> , b: impl Add<T, Output=T>) -> T {
///     a + b
/// }
///
/// ```
///
/// Base usage: 正确
///
/// ```
///
/// use std::ops::Add;
/// //  只能用于单个参数
/// fn hello<T>(a: impl Add<T, Output=T> -> impl Add<T, Output=T> {
///     a
/// }
///
/// ```
pub fn impl_trait() {
    use std::fmt::Debug;
    pub trait Fly {
        fn fly(&self) -> bool;
    }
    #[derive(Debug)]
    struct Duck;
    #[derive(Debug)]
    struct Pig;
    impl Fly for Duck {
        fn fly(&self) -> bool {
            true
        }
    }

    impl Fly for Pig {
        fn fly(&self) -> bool {
            false
        }
    }

    fn fly_static(s: impl Fly+Debug) -> bool {
        s.fly()
    }

    fn can_fly(s: impl Fly + Debug) -> impl Fly {
        if s.fly() {
            println!("{:?} can fly", s);
        }else {
            println!("{:?} can't fly", s);
        }
        s
    }

    fn dyn_can_fly(s: impl Fly + Debug + 'static ) -> Box<dyn Fly> {
        if s.fly() {
            println!("{:?} can fly ", s);
        } else {
            println!("{:?} can't fly", s);
        }
        Box::new(s)
    }
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);

    let pig = Pig;
    can_fly(pig);
    let duck = Duck;
    can_fly(duck);

    let duck = Duck;
    dyn_can_fly(duck);
}

/// # 用作标签trait : Copy
///
/// 实现Copy 必须也实现Clone
/// Base usage:
///
///```
/// struct MyStruct;
/// impl Copy for MyStruct {}
/// impl Clone for MyStruct {
///     fn clone(&self) -> MyStruct {
///         *self
///     }
/// }
///```
///
/// Base usage: 可以用derive属性自动生成Copy
///
/// ```
/// #[derive(Copy, Clone)]
/// struct MyStruct;
/// ```
pub fn impl_clone() {
        struct MyStruct;
        impl Copy for MyStruct {}
        impl Clone for MyStruct {
            fn clone(&self) -> MyStruct {
                *self
            }
        }
    }
/// # 用做标签triat : Copy
///
///在Rust 中一共有五种主要的标签trait:
/// - Sized, 用来标示编译期可确定大小的类型
/// - Unsize, 目前该trait为实验特性，用于标示动态大小类型(DST)
/// - Copy, 用来标示可以按位复制其值的类型
/// - Send, 用来标示可以跨线程安全通信的类型
/// - Sync, 用来标示可以在线程间安全共享引用的类型
///
/// Base usage: 检测是否实现Copy triat的函数
///
/// ```
/// fn test_copy<T: Copy> (i: T) {
///     println!("hh");
/// }
/// let a = "String".to_string();
/// test_copy(a);
/// ```
pub fn test_copy_trait() {
    fn test_copy<T: Copy> (i: T) {
        println!("hhh");
    }

    let a = "String";
    test_copy(a);
}
/// # 用作标签的trait： Sync/Send
///
/// Base usage: error
/// 利用所有权机制化解了一次并发危机
/// ```
/// use std::thread;
/// let mut x = vec![1, 2, 3, 4];
/// thread::spawn(|| { // Error: may outlive borrowed value 'x'
///     x.push(1);// sub thread modify x
/// });
/// x.push(2);// main thread modify x
/// ```
///
/// Base usage right
/// 使用move关键字
/// ```
/// use std::thread;
/// let mut x = vec![1, 2, 3 ,4];
/// thread::spawn(move || { // x 实现Sync 和 Send
///     x.push(1);
/// });
/// //x.push(2);// 父线程不允许修改x
/// ```
/// Base usage： 未实现Sync和Send的类型示范
///
/// ```
/// use std::thread;
/// use std::rc::Rc;
///
/// let x = Rc::new(vec![1, 2, 3, 4]);
/// thread::spawn( move || { // Error: std::maker::Send is not satisfied
///     x[1];
/// });
/// ```
pub fn sync_send_trait() {
    use std::thread;
    let mut x = vec![1, 2, 3, 4];
    println!("{:?}", x);
    thread::spawn(move || {
        x.push(1);
        println!("{:?}", x);
    });
    // println!("{:?}", x);
}

/// # 类型转换： 自动解引用 （ For SmartPointer)
/// 
/// Base usage: `vec<T>` 实现了 `Deref<Target=[T]>`
/// 
/// ```
/// fn foo(s: &[i32]) {
///     println!("{:?}", s[0]);
/// }
/// let v = vec![1, 2, 3];
/// foo(&v);
/// ```
/// 
/// Base usage: `String`实现了`Deref<Target=str>`
/// 
/// ```
/// let a = "hello".to_string();
/// let b = " world".to_string();
/// let c = a + &b;
/// pritln!("{:?}", c); // "hello world"
/// ```
/// 
/// Base usage: `Rc<T>`实现了`Deref<Target<T>>`
/// 
/// ```
/// use std::rc::Rc;
/// let x = Rc::new("hello");
/// println!("{:?}", c.chars());
/// ```
pub fn auto_deref() {
    fn foo(s: &[i32]) {
        println!("{:?}", s[0]);
    }
    let v = vec![1, 2, 3];
    foo(&v);
}

/// # 类型转换： 手动解引用( For SmartPointer)
/// 
/// Base usage: `Rc` 和 `&str` 都实现了clone
/// 
/// ```
/// use std::rc::Rc;
/// 
/// let x = Rc::new("hello");
/// let y = x.clone();// Rc<&str>
/// let z = (*x).clone(); // &str
/// ```
/// 
/// Base usage: match 匹配里需要手动解引用
/// 
/// 将match &x 改为以下5中形式任意一种即可：
/// - match x.deref(), 直接调用deref方法，需要use std::ops::Deref
/// - match x.as_ref(), String类型提供了as_ref方法来返回一个&str类型，该方法定义于AsRef trait中
/// - match x.borrow(),方法borrow定义于Borrow trait中，行为和AsRef，需要uae std::borrow::Borrow
/// - match &*x, 使用“解引用”操作符，将String转换为str，然后再用“引用”操作符转为&str
/// - match &x[..], 这是因为String类型的index操作可以返回&str类型
/// 
/// ```
/// let x = "hello".to_string();
/// match &x {
///     "hello" => { println!("hello")}; 
///     _ => {}
/// }
/// ```
pub fn manual_deref() {
    use std::rc::Rc;

    let x = Rc::new("hello");
    let y = x.clone(); // Rc<&str>
    println!("{:?}", y);
    let z = (*x).clone(); // &str
    println!("{:?}", z);
}

/// # 类型转换： as操作
/// 
/// Base usage: 无歧义完全限定语法Fully Qualified Syntax for Disambiguation
/// 也称通用函数调用语法(UFCS)
/// 
/// ```
/// struct S(i32);
/// triat A {
///     fn test(&self, i: i32);
/// }
/// trait B {
///     fn test(&self, i: i32);
/// }
/// impl A for S {
///     fn test(&self, i: i32) {
///         println!("From A: {:?}", i);
///     }
/// }
/// impl B for S {
///     fn test(&self, i: i32) {
///         println!("From B: {:?}", i + 1);
///     }
/// }
/// let s = S(1);
/// A::test(&s, 1);
/// B::test(&s, 1);
/// <S as A>::test(&s, 1);
/// <S as B>::test(&s, 1);
/// ```
/// 
/// Base usage: 父类型子类型相互转换
/// 
/// ```
/// let a: &'static str = "hello"; //&'static str
/// let b: &str = a as &str; // &str
/// let c: &'static str = b as &'static str; // &'static str
/// ```
pub fn fqsfd() {
    struct S(i32);
    trait A {
        fn test(&self, i: i32);
    }
    trait B {
        fn test(&self, i: i32);
    }
    impl A for S {
        fn test(&self, i: i32){
            println!("From A: {:?}", i);
        }
    }

    impl B for S {
        fn test(&self, i: i32) {
            println!("From B: {:?}", i + 1);
        }
    }

    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 1);
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);
}

/// #类型转换 : From 和 Into 
/// 
/// Base usage: String 实现了From
/// 
/// ```
/// let string = "hello".to_string();
/// let other_string = String::from("hello");
/// assert_eq!(string, other_string);
/// ```
/// 
/// Base usage： 使用into简化代码
/// 
/// ```
/// #[derive(Debug)]
/// struct Person{ name: String }
/// impl Person {
///     fn new<T: Into<String>>(name: T) -> Person {
///         Person { name: name.into() }   
///     }
/// }
/// let person = Person::new("Alex");
/// let person = Person::new("Alex".to_string());
/// println!("{:?}", person);
/// ```
pub fn from_into(){
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
}

/// # trait :  局限
/// 
/// Base usage: 作为父crate在考虑设计trait时，不得不考虑要不要给全体的T或&'a T实现trait
///     才能不影响到子crate
/// 
/// ```
/// impl <T: Foo> for T {}
/// impl <'a, T: Bar> Bar for &'a T {}
/// ```
/// 
/// Base usage: 
/// 
/// ```
/// use std::ops::Add;
/// #[derive(PartialEq)]
/// struct Int(i32);
/// impl Add<i32> for Int {
///     type Output = i32;
///     fn add(self, other: i32) -> Self::Output {
///         (self.0) + ohter
///     }
/// }
/// 
/// // impl Add<i32> for Option<Int> {
/// //    //TODO
/// //}
/// 
/// impl Add<i32> for Box<Int> {
///     type Output= i32;
///     fn add(self, other: i32) -> Self::Output {
///         (self.0) + other
///     }
/// }
/// 
/// assert_eq!(Int(3)+3, 6);
/// assert_eq!(Box::new(Int(3)) + 3, 6);
/// ```
pub fn trait_limit() {
    use std::ops::Add;
    #[derive(PartialEq)]
    struct Int(i32);
    impl Add<i32> for Int {
        type Output = i32;
        fn add(self, other: i32) -> Self::Output {
            (self.0) + other
        }
    }
    // impl Add<i32> for Option<Int> {
    //        //TODO
    //}
    impl Add<i32> for Box<Int> {
        type Output = i32;
        fn add(self, other: i32) -> Self::Output {
            (self.0) + other
        }
    }
    assert_eq!(Int(3) + 3, 6);
    println!("Int(3) + 3 = {}", Int(3) + 3);
    assert_eq!(Box::new(Int(3)) + 3, 6);
    println!("Box::new(Int(3))  + 3 = {}", Box::new(Int(3)) + 3);
}

/// # trait: 特化 (specialization)
/// 
/// 重叠规则： 不能为重叠的类型实现同一个trait
/// 以下代码会编译失败。这种实现方式也被称为覆盖式实现(blanket impl)
/// 
/// ```
/// impl<T> AnyTrait for T
/// impl<T> AnyTrait for where T: Copy
/// impl<T> AnyTrait for String
/// ```
/// 
/// 第一个问题：性能问题
/// 
/// 以下代码为所有类型T实现了AddAssign,也就是 += 操作，
/// 这样实现虽然好，但是会带来性能问题，因为强制使用了clone方法，
/// 但实际上有的类型并不需要clone。
/// 而因为有重叠规则的限制，并不能为某些不需要clone的具体类型重新实现add_assign方法
/// 所以，在标准库中，就基本上为某个具体的类型都各自实现了一遍AddAssign.
/// 
/// 第二个问题： 代码重用
/// 
/// 如果没有重叠规则，则可以默认使用上面对泛型T的实现。
/// 然后对不需要clone的类型重新实现AddAssign,那么就完全没必要为每个具体类型都
/// 都实现一遍add_assign方法，可以省掉很多重复代码
/// 
/// ```
/// impl<R, T: Add<R> + Clone> AddAssign<R> for T {
///     fn add_assign(&mut self, rhs: R) {
///         let tmp = self.clone() + rhs;
///         *self = tmp;
///     }
/// }
/// ```
/// 
/// Basic usage: 使用特化之trait默认实现
/// 
/// ```
/// #![fetaure(specialization)]
/// struct Diver<T> {
///     inner: T,
/// }
/// trait Swimmer {
///     fn swim(&self) {
///         println!("swimming")
///     }
/// }
/// 
/// impl <T> Swimmer for Diver<T> {}
/// 
/// impl Swimmer for Diver<&'static str> {
///     fn swim(&self) {
///         println!("drowning, help!")
///     }
/// }
/// 
/// let x = Diver:<&'staic str> { inner : "bob"};
/// x.swim();
/// let y = Diver::<String> { inner: String::from("Alice") };
/// y.swim();
/// }
/// ```
/// 
/// Basic usage: 使用特化之default关键字
/// 
/// ```
/// #![feature(sepcialization)]
/// struct Diver<T> {
///     inner: T,
/// }
/// trait Swimmer {
///     fn swim(&self);
/// }
/// impl<T> Swimmer for Diver<T> {
///     default fn swim(&self) {
///         println!("swimming")
///     }
/// }
/// 
/// impl Swimmer for Diver<&'static str> {
///     fn swim(&self) {
///         println!("drowning, help!")
///     }
/// }
/// 
/// let x = Diver::<&'static str> { inner: "Bob" };
/// x.swim();
/// let y = Diver::<String> { inner: String::from("Alice") };
/// y.swim();
/// ``` 
pub fn trait_special() {
    // #![feature(specialization)]
    struct Diver<T> {
        inner: T,
    }
    trait Swimmer {
        fn swim(&self);
            // println!("swimming")
        // }
    }

    // impl<T> Swimmer for Diver<T> {
        // fn swim(&self) {
            // println!("swimming")
        // }
    // }

    impl Swimmer for Diver<&'static str> {
        fn swim(&self) {
           println!("drowning, help!") 
        }
    }

    let x = Diver::<&'static str> { inner: "Bob" };
    x.swim();
    // let y = Diver::<String> { inner: String::from("Alice")};
    // y.swim();
}

/// # trait局限： GAT (Generic Associated Type) 或 ATC ( Associated type constructor)
/// 
/// Rust标准库中的迭代器API不允许生成从迭代器本身借用的元素
/// 比如std::io::Lines, 其实现迭代器只能读一行数据分配一个新的string,而不能重用内部缓存区
/// 而流式迭代器可以允许通过引用迭代器，而不是值，这样就可以实现重用。
/// 想要支持这种StreamIterator，就必须实现一种更高级别的类型多态性，
/// 所以 GAT 将会把Rust类型系统扩展到支持特点形式的高级多态性，称为泛型关联类型，
/// 也叫做关联类型构造器
/// 
/// 不过，当前此功能还未实现，
/// 相关[RFC1598](https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md)
/// Base usage: 
/// 
/// ```
/// trait StreamingIterator {
///     type Item<'a>;
///     fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
/// }
/// ``
/// 
pub fn gat() {
    unimplemented!();
}

}