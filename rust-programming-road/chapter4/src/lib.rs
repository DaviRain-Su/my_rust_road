pub mod davirain { 
/// # 栈
/// 
/// Base usage: 简单函数调用栈帧展示
/// 
/// ```
/// fn foo(x: u32) { //------+ 
///     let y = x;   //      | foo函数帧
///     let z = 100; //      |  其实就是foo函数作用域
/// }                // -----+
/// fn main() {
///     let x = 42;
///     foo(x);
/// }
/// ```
pub fn simple_stack_frame() {
    fn foo(x: u32) {
        let y = x;
        let z = 100;
    }
    fn st(){
        let x = 42;
        foo(x);
    }
}

/// # 内存对齐 :
/// 
/// Base usage: 结构体内存对齐
/// 
/// ```
/// struct A {
///     a: u8,
///     b: u32,
///     c: u16,
/// }
/// fn main() {
///     println!("{:?}", std::mem::size_of::<A>()); // 8
/// }
/// ```
/// 
/// Base usage : Union内存对齐
/// 
/// ```
/// union U {
///     f1: u32,
///     f2: f32,
///     f3: f64,
/// }
/// fn main() {
///     println!("{:?}", std::mem::size_of::<U>()); //8
/// }
/// ```
pub fn memory_align() {
    use std::mem;
    struct A {
        a: u8,
        b: u32,
        c: u16,
    }
    union U {
        f1: u32,
        f2: f32, 
        f3: f64,
    }
    println!("The struct A size is {:?}", mem::size_of::<A>());
    println!("The union U size is {:?}", mem::size_of::<U>());
}

/// #   复杂结构内存布局
/// 
/// Base usage: 结构体内存对齐
/// 
/// ```
/// struct A {
///     a: u32, 
///     b: Box<u64>,
/// }
/// struct B(i32, f64, char);
/// struct N;
/// enum E {
///     H(u32),
///     M(Box<u32>)
/// }
/// union U {
///     u: u32,
///     v: u64,
/// }
/// 
/// fn main() {
///     println!("Box<u32> : {:?}", std::mem::size_of::<Box<u32>>());
///     println!("A: {:?}", std::mem::size_of::<A>());
///     println!("B: {:?}", std::mem::size_of::<B>());
///     println!("N: {:?}", std::mem::size_of::<N>());
///     println!("E: {:?}", std::mem::size_of::<E>());
///     println!("U: {:?}", std::mem::size_of::<U>());
/// }
/// ```
pub fn memory_layout() {
    use std::mem;
    struct A {
        a: u32, 
        b: Box<u64>,
    }
    struct B(i32, f64, char);
    struct N;
    enum E {
        H(u32),
        M(Box<u32>),
    }
    union U {
        u: u32,
        v: u64,
    }
    
    println!("Box<u32> : {:?}", std::mem::size_of::<Box<u32>>());
    println!("A: {:?}", std::mem::size_of::<A>());
    println!("B: {:?}", std::mem::size_of::<B>());
    println!("N: {:?}", std::mem::size_of::<N>());
    println!("E: {:?}", std::mem::size_of::<E>());
    println!("U: {:?}", std::mem::size_of::<U>());
}

/// # 资源管理
/// 
/// Base usage: 变量与函数
///     变量默认存储在栈中
/// 
/// ```
/// fn main() {
///     let x: i32; //Rust会检查未初始化的变量，并报错
///     println!("{}", x);
/// }
/// ```
/// 
/// Base usage: if 分支检查
/// 
/// ```
/// fn main() {
///     let x: i32;
///     if true {
///         x = 1;
///     } else {  //如果去掉else，则编译器会报错
///         x = 2;
///     }
///     println!("{}", x); 
///     //如果去掉此行，再去掉else则不会报错，因为
///     //没有使用到x的地方，就算未初始化也没有关系
/// }
/// ```
/// 
/// Base usage: break会将分支中的变量返回
/// ```
/// fn main() {
///     let x : i32;
///     loop {
///         if true {
///             x = 2;
///             break;
///         }
///     }
///     println!("{}", x);//因为break会返回分支中的变量，所以该行可以正确打印2
///     
/// }
/// ```
/// 
/// Base usage: 初始化数组
/// ```
/// fn main() {
///     let a: Vec<i32> = vec![];
///     //必须指定类型，因为无法做类型推断
///     let b: [i32; 0] = [];
/// }
/// ```
/// 
/// Base usage: 当将一个已初始化变量y绑定给另外一个变量y2
/// 时，Rust会把变量y看作是逻辑上的未初始化变量
/// 
/// ```
/// fn main() {
///     let x = 42;
///     let y = Box::new(5);
///     println!("{:p}", y);
///     let x2 = x;
///     let y2 = y;
///     //println!("{:p}", y); //y实际上已经变成了未初始化变量
/// }
/// ```
pub fn binding_and_func() {
    let x: i32 = 1;
    println!("{}", x);
}

/// # 智能指针
/// 
/// Base uage: 智能指针实例
/// 
/// ```
/// fn main() {
///     let s = String::from("hello");
///     // let deref_s: str = *s;
///     let v = vec![1, 2, 3];
///     // let deref_v: [u32] = *v;
/// }
/// ```
pub fn smart_point() {
    let s = String::from("hello");
    // let deref_s : str = *s;
    let v = vec![1, 2, 3];
    // let deref_v: [u32] = *v;
}

/// # RAII : 确定性析构
/// 
/// Base: usage: 实现Drop
/// 
/// ```
/// use std::ops::Drop;
/// #[derive(Debug)]
/// struct S(i32);
/// impl Drop for S {
///     fn drop(&mut self) {
///         println!("drop {}", self.0);
///     }
/// }
/// fn main() {
///     let x = S(1);
///     println!("crate x: {:?}", x);
///     {
///         let y = S(2);
///         println!("crate y: {:?}", y);
///         println!("exit inner scope");    
///     }
///     prinln!("exit main");
/// }
/// ```
/// 
/// Base usage: 配合Valgrind 工具检查是否会内存泄漏
/// 看看Box<T>是否会自动释放
/// ```
/// fn create_box() {
///     let box3 = Box::new(3);
/// }
/// fn main() {
///     let box1 = Box::new(1);
///     {
///         let box2 = Box::new(2);
///     }
///     for _ in 0..1000 {
///         create_box();
///     }
/// }
/// ```
/// 
/// Base usage: 使用花括号主动析构
/// ```
/// fn main() {
///     let mut v = vec![1, 2, 3];
///     {
///         v
///     };
///     // v.push(4);
/// }
/// ```
/// 
/// Base usage: 变量遮蔽不等于drop
/// 
/// ```
/// use std::ops::Drop;
/// #[derive(Debug)]
/// struct S(i32);
/// impl Drop for S {
///     fn drop(&mut self) {
///         println!("drop for {}", self.0);
///     }
/// }
/// 
/// fn main() {
///     let x = S(1);
///     println!("create x: {:?}", x);
///     let x = S(2);
///     println!("create shadowing x: {:p}", x);
/// }
/// ```
pub fn drop_demo() {
    use std::ops::Drop;
    #[derive(Debug)]
    struct S(i32);
    impl Drop for S {
        fn drop(&mut self) {
            println!("drop {}", self.0);
        }
    }

    let x = S(1);
    println!("crate x: {:?}", x);
    {
        let y = S(2);
        println!("crate y: {:?}", y);
        println!("exit inner scope");
    }
    println!("exit main");
}

/// # 制造内存泄漏
/// 
/// Base usage: 内存泄漏不属于内存安全范围
///     内存泄漏是可以通过相互引用精心构造出来的
/// 
/// ```
/// fn test_list_rc() {
///     use std::rc::Rc;
///     type NodePtr<T> = Option<Rc<Node<T>>>;
///     // #[derive(Debug)]
///     struct Node<T> {
///         data: T,
///         next: NodePtr<T>,
///     }
///
///     let first = Rc::new(Node { data: 1, next: None});
///     let second = Rc::new(Node { data: 2, next: Some(first.clone())});
///     first.next = Some(second.clone());
///     second.next = Some(first.clone());
/// }
///
/// fn test_list_box () {
///     type NodePtr<T> = Option<Box<Node<T>>>;
///     // #[derive(Debug)]
///     struct Node<T> {
///         data: T,
///         next: NodePtr<T>,
///     }
///
///     let first = Box::new(Node { data: 1, next: None});
///     let second = Box::new(Node { data: 2, next: None});
///     first.next = Some(second);
///     second.next = Some(first);
///
/// }
/// ```
/// 
/// ```
/// fn main() {
///     use std::rc::Rc;
///    use std::cell::RefCell;
///    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
///    // #[derive(Debug)]
///    struct Node<T> {
///        data: T,
///        next: NodePtr<T>,
///    }
///
///    impl <T> Drop for Node<T> {
///        fn drop(&mut self) {
///            println!("Dropping!");
///        }
///    }
///
///    let first = Rc::new(RefCell::new(Node { 
///        data: 1,
///        next: None
///    }));
///    let second = Rc::new(RefCell::new(Node {
///        data: 2,
///        next: Some(first.clone())
///    }));
///    first.borrow_mut().next = Some(second.clone());
///    second.borrow_mut().next = Some(first.clone());
///```
pub fn memory_leak() {
    use std::rc::Rc;
    use std::cell::RefCell;
    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
    // #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: NodePtr<T>,
    }

    impl <T> Drop for Node<T> {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    let first = Rc::new(RefCell::new(Node { 
        data: 1,
        next: None
    }));
    let second = Rc::new(RefCell::new(Node {
        data: 2,
        next: Some(first.clone())
    }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());
}

} // end of mod davirain