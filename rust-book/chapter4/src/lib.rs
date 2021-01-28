pub mod davirain {

    /// # 第四章 认识所有权
    ///
    /// 在所有权概念和相关工具的引入，Rust才能够在没有垃圾回收机制的前提下保障内存安全。
    ///
    /// 讨论的内容有： 借用、切片、Rust在内存中布局数据的方式
    ///
    /// ## 什么是所有权
    ///
    /// 使用包含特定规则的所有权系统来管理内存，这套规则允许编译器在编译过程中，执行检查工作，而不会
    /// 产生任何开销。
    ///
    /// ### 所有权规则
    ///
    /// - Rust中的每一个值都有一个对应的变量作为它的所有者
    /// - 在同一时间内，值有且仅有一个所有者
    /// - 当所有者离开自己的作用有时，它持有的值就会被释放掉
    ///
    /// ### 变量作用域
    ///
    /// 作用域是一个对象在程序中有效的范围
    ///
    /// ```
    /// let s = "hello";
    /// ```
    /// 这里的变量s指向一个字符串字面量，它的值被硬编码到当前的程序中，变量从声明的位置开始直到当前作用域结束都是有效的。
    ///
    /// ```
    /// {                    // 由于变量s还未被声明，所以他在这里不可用
    ///     let s = "hello"; // 从这里变量s变得可用
    ///                      // 执行与s相关的操作
    /// }                    // 作用域到这里结束，变量s再次不可用
    /// ```
    ///
    /// ### String类型
    ///
    /// 字符串字面量，被硬编码到程序中，这是因为字符串字面量是不可变的。
    /// 并不是所有字符串的值都能够在编写代码时确定：这时Rust中的String,会在堆上分配内存。
    /// 能够处理在编译时未知大小的文本。
    ///
    /// ```
    /// let s = String::from("hello"); //String::from()是关联函数
    ///
    /// let mut s = String::from("hello");
    /// s.push_str(", world!"); //push_str()函数向String空间的尾部添加一段字面量
    ///
    /// println!("{:?}", s); // "hello, world"
    /// ```
    ///
    /// ### 内存与分配
    ///
    /// 对于字符串字面量来说，在编译时就直到内存，因为这部分硬编码的文本被直接嵌入到了最终的可执行文件中。
    /// 访问字符串字面量高效的原因，完全是因为字符串字面量的不可变性。
    ///
    /// Rust中， 内存会自动地在拥有它地变量离开作用域后进行释放
    ///
    /// ```
    /// {
    ///     let s = String::from("hello"); // 从这里开始，变量s变得有效
    ///
    ///     //执行与s相关地操作
    /// }                                  // 作用域到这里结束，变量s失效
    /// ```
    ///
    /// Rust在变量离开作用域时，会调用一个叫作drop地特殊函数，在这个函数中会做一些释放内存或者相关资源地操作。
    /// Rust会在作用域结束地地方（即}处）自动调用drop函数。
    ///
    /// 在C++中，这种在对象生命周期结束时释放资源地模式也称为资源获取即初始化(Resource Acquisition Is Initialization, RAII)
    ///
    ///
    /// #### 变量和数据交互地方式： 移动
    ///
    /// Rust中的多个变量可以采用一种特殊的方式与同一数据进行交互
    ///
    /// ```
    /// let x = 5;
    /// let y = x;
    /// println!("{}", x);
    /// println!("{}", y);
    ///
    /// let s1 = String::from("hello");
    /// let s2 = s1;// s1 move to s2
    /// //println!("{}", s1); // error value have moved
    /// println!("{}", s2);
    /// ```
    ///
    /// 为了确保内存安全， 同时也避免复制分配的内存，Rust在这种场景下会简单地将s1废弃，
    /// 不再视为一个有效的变量，因此，Rust也不需要在s1离开作用域后清理任何东西。
    ///
    /// ```
    ///
    /// let s1 = String::from("hello");
    ///
    /// let s2 = s1; // value moved here
    ///
    /// println!("{}, world!", s1); // value used here after move
    /// ```
    /// Rust使第一个变量无效，所以使用了新术语移动来描述这一行为，而不再使浅拷贝。
    ///
    /// Rust中的设计原则：
    /// Rust永远不会自动地创建数据的深度拷贝，因此在Rust中，任何自动的赋值操作都可以被视为高效的。
    ///
    /// #### 变量和数据交互的方式： 克隆
    ///
    /// 当确实需要去深度拷贝String堆上的数据，而不仅仅是栈数据时，可以使用clone的方法。
    ///
    /// ```
    /// let s1 = String::from("hello");
    /// let s2 = s1.clone();
    ///
    /// println!("s1 = {}, s2 = {}", s1, s2);
    /// ```
    ///
    /// #### 栈上数据的复制
    ///
    /// ```
    /// let x = 5;
    /// let y = x;
    ///
    /// println!("x = {}, y = {}", x, y);
    /// ```
    ///
    /// 整型的类型可以编译时确定自定的大小，并且能够将自己的数据完整地存储在栈中，对于这些值的复制
    /// 操作永远都是非常快速的。
    ///
    /// Rust 提供来一个名为Copy的trait， 它可以用于整数这类完全存储在栈上的数据类型，一旦某种类型拥有了Copy 这种
    /// trait，那么它的变量就可以在赋值给其他变量复制之后保持可用性。
    ///
    /// 如果一种类型本身或者这种类型的任意成员实现了Drop trait, 那么Rust就不允许其实现Copy trait.
    /// 尝试给某个需要在离开作用域时执行特殊指令的类型实现Copy trait会导致编译时错误。
    ///
    /// 那些类型是Copy类型？
    ///
    /// 任何简单标量的组合类型都可以是Copy，任何需要分配内存或某种资源的类型都不会是Copy的，
    ///
    /// - 所有的整数类型，u32
    /// - 仅拥有两个中值的布尔类型： bool
    /// - 字符类型： char
    /// - 所有的浮点类型： f64
    /// - 如果元组包含的所有字段的类型都是Copy的，那么这个元组也是Copy的.(i32, i32)是Copy的，
    /// 但是(i32, String)不是。
    ///
    ///
    /// ### 所有权与函数
    ///
    /// 将值传递给函数在语义上类似于对变量进行赋值。将变量传递给函数将会触发移动或复制，就像赋值语句一样。
    ///
    /// ```
    /// fn main() {
    ///     let s = String::from("hello"); // 变量s进入作用域
    ///
    ///     takes_ownership(s);  // s的值被移动进了函数
    ///                          // 所以它从这里开始不再有效
    ///     //println!("{}", s); // s have moved
    ///
    ///     let x = 5;  //变量x进入作用域
    ///
    ///     make_copy(x); //变量x同样被传递进了函数
    ///     // 但由于i32是Copy的，所以依然可以在这之后使用x
    ///
    ///     println!("{}", x);
    /// } // x首先离开作用域， 随后是s
    /// // 但由于s的值已经发生了移动，所以没有什么特别的事情发生
    ///
    /// fn takes_ownership(some_string: String) { // some_string 进入作用域
    ///     println!("{}", some_string);
    /// }// some_string 在这里离开作用域，drop函数被自动调用
    /// //some_string所占用的内存也就随之被释放了
    ///
    /// fn make_copy(some_integer: i32) { // some_integer 进入作用域
    ///     println!("{}", some_integer);
    /// }// some_integer 在这里离开了作用域，没有什么特别的事情发生
    /// ```
    ///
    /// ### 返回值与作用域
    ///
    /// 函数在返回值的过程中也会发生所有权的转移
    ///
    /// ```
    /// fn main() {
    ///     let s1 = gives_ownership(); // gives_ownership将它的返回值移至s1中
    ///
    ///     let s2 = String::from("hello"); // s2进入作用域
    ///
    ///     let s3 = takes_and_gives_back(s2); // s2被移进函数
    ///     // takes_and_gives_back中，而这个函数的返回值又被移动到了变量s3上
    /// } //s3在这里离开作用域并销毁，由于s2已经移动了，所以它不会在离开作用域时发生任何事情。
    /// // s1最后离开作用域并被销毁。
    ///
    /// fn gives_ownership() -> String {
    ///     //gives_ownership会将它的返回值移至调用它的函数内
    ///     let some_string = Stirng::from("hello"); //some_string 进入作用域
    ///     some_string  //some_string 作为返回值移动至调用函数
    /// }
    /// //takes_and_gives_back将取得一个String的所有权并将它作为结果返回
    /// fn takes_and_gives_back(a_stirng: String) -> String {
    ///     // a_string 进入作用域
    ///     a_stirng // a_string 作为返回值移至调用函数
    /// }
    /// ```
    /// 变量所有权的转移总是遵循相同的模式： 将一个赋值给另一个变量时就会转移所有权。
    /// 当一个持有堆数据的变量离开作用域时， 它的数据就被drop清理回收，除非这些数据的所有权移动了另一个变量上。
    ///
    /// 在所有的函数中都要获取所有权并返回所有权显得有些烦琐。假如你希望在调用函数时保留参数的所有权，那么就不得不将传入的值
    /// 作为结果返回。除了这些需要保留所有权的值，函数还可能会返回它们本身的结果/
    ///
    /// 可以利用元祖来同时返回多个值。
    ///
    /// ```
    /// fn main() {
    ///     let s1 = String::from("hello");
    ///     let (s2, len) = calculate_length(s1);
    ///
    ///     println!("The length of '{}' is {}", s2, len);
    /// }
    /// fn calculate_length(s: String) -> (String, usize) {
    ///     let length = s.len(); // len() 会返回当前字符串的长度
    ///
    ///     (s, length)
    /// }
    /// ```
    ///
    /// ## 引用与借用
    ///
    /// ```
    /// fn main() {
    ///     let s1 = Stirng::from("hello");
    ///
    ///     let len = calculate_length(&s1);
    ///
    ///     println!("The length pf '{}' is {}", s1, len);
    /// }
    /// fn calculate_length(s: &String) -> usize { // s 是一个指向String的引用
    ///
    ///     s.len()
    /// } // 到来这里, s离开作用域，但是由于它并不持有自己所指向值的所有权，
    /// //所以没有什么特殊的事情发生
    /// // 变量s的有效作用域与其他任何函数参数一样，唯一不同的是，他不会在离开自己的作用域时
    /// //销毁其指向的数据，因为它并不拥有该数据的所有权。当一个函数使用引用而不是值作为参数时，
    /// //我们便不需要为了归还所有权而特意去返回值，毕竟在这种情况下，我们根本没有取得所有权。
    ///
    /// ```
    /// 通过引用传递参数给函数的方法也被称为借用(borrowing)
    ///
    ///
    /// &代表的就是引用语义，它们允许在不获取所有权的前提下使用值。
    ///
    /// 与使用引用& 相反的操作被称为解引用derferencing, 它使用*作为运算符。
    ///
    /// let s1 = Stirng::from("hello");
    /// let len = calculate_length(&s1);
    ///
    /// 这里的&s1语法允许我们在不转移所有权的前提下，创建一个指向s1值的引用。
    /// 由于引用不持有值的所有权，所以当引用离开当前作用域时，它指向的值也不会被丢弃。
    ///
    /// ```
    /// fn main() {
    ///     let s = String::from("hello");
    ///
    ///     change(&s);
    /// }
    ///
    /// fn change(some_string: &String) {
    ///     //some_string.push_str(", world"); // cannot borrow immutable borrowed content
    ///                                      //   "*some_string" as mutable
    /// }
    ///
    /// ```
    /// 引用是默认不可变的，Rust不允许去修改引用指向的值。
    ///
    /// ### 可变引用
    ///
    /// ```
    /// fn main() {
    ///     let mut s = String::from("hello"); //将变量s声明为mut,即可变的
    ///
    ///     change(&mut s);//使用&mut s来给函数传入一个可变引用
    /// }
    /// fn change(some_string: &mut String ) { //函数签名也是接收一个可变引用作为参数
    ///     some_string.push_str(", world");
    /// }
    /// ```
    ///
    /// 可变引用使用的一个限： 对于特定作用域中的特定数据来说，一次只能声明一个可变引用。
    ///
    /// ```
    /// let mut s = String::from("hello");
    /// // cannot borrow 's' as mutable more than once at a time
    ///
    /// let r1 = &mut s; // first mutable borrow occurs here
    /// let r2 = &mut s; // second mutable borrow occurs here
    /// ```
    ///
    /// 数据竞争， 在指令满足以下三种情况时发生：
    /// - 两个或两个以上的指针同时访问同一空间
    /// - 其中至少有一个指针会向空间中写入数据
    /// - 没有同步数据访问的机制
    ///
    /// 可以通过花括号来创建一个新的作用域范围，这样我们可以创建多个可变引用
    ///
    /// ```
    /// let mut s = String::from("hello");
    ///
    /// {
    ///     let r1 = &mut s;
    ///
    /// } // 由于r1在这里离开来作用域，所以我们可以合法地在创建一个可变引用
    /// let r2 = &mut s;
    /// ```
    ///
    /// 不能在拥有不可便引用的同时创建可变引用
    /// 同时创建多个不可变引用是合法的，对数据的只读操作不会影响到其他读取数据的用户
    ///
    /// ```
    /// let mut s = Stirng::from("hello");
    ///
    /// let r1 = &s; // immutable borrow occurs here
    /// let r2 = &s;
    /// let r3 = &mut s; // mutable borrow occurs here
    /// ```
    ///
    /// ### 悬垂指针
    ///
    /// 是指， 指针曾经存在的某处内存地址，当内存已经被释放掉甚至是重新分配另作他用来，
    /// 在Rust语言中，编译器会确保引用永远不会进入这种悬垂状态。
    ///
    /// 假如我们当前持有某个数据的引用，那么编译器可以保证这个数据不会在引用被销毁前离开自己作用域。
    ///
    /// ```
    /// fn main() {
    ///     let reference_to_nothing = dangle();
    /// }
    /// // missing lifetime specifier
    /// fn dangle() -> &String { // expected lifetime parameter
    ///     let s = String::from("hello"); // s绑定到新的String上
    ///
    ///     &s // 指向s的引用返回给调用者
    /// }//变量s在这里离开作用域并随之被销毁，它指向的内存自然也不再有效 ！！！
    /// ```
    ///
    /// ### 引用的规则
    ///
    /// - 在任何一段给定的时间，要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用
    /// - 引用总是有效的
    ///
    /// ## 切片
    ///
    /// Rust中还有一种不持有所有权的数据类型： 切片(slice)
    ///
    /// 切片允许引用结合中某一段连续的元素序列，而不是整个集合。
    ///
    /// ```
    /// fn first_word(s: &Stirng) -> usize {
    ///     let bytes = s.as_bytes();  //as_bytes()将String转换成字节数组
    ///     
    ///     for (i, &item) in bytes.iter().enumerate() {
    ///
    ///     // iter()方法创建一个可以遍历字节数组的迭代器
    ///     // enumerate() 将iter()的每个输出元素逐一封装在对应的元组中返回。
    ///     // 元组的第一个元素是索引，第二个元素是指向集合中字节的引用。
    ///     //enumerate()返回的是一个元组，可以使用模式匹配来解构它
    ///         if item == b' ' {
    ///             return i;
    ///         }   
    ///     }
    ///     
    ///     s.len()
    /// }
    ///
    /// fn main() {
    ///     let mut s = Stirng::from("hello world");
    ///
    ///     let word = first_word(&s); //索引5会绑定到变量word上
    ///     
    ///     s.clear(); // 这里clear方法会清空当前字符串，使之变为""
    ///     //虽然word依然拥有5这个值，但因为我们用于搜索的字符串发生了变化，
    ///     //所以这个索引也就没有了任何意义，word到这里便失去了有效性
    /// }
    /// ```
    ///
    /// ### 字符串切片
    ///
    /// 字符串切片是指向String 对象中某个连续部分的引用，
    ///
    /// ```
    /// let s = String::from("hello world");
    ///
    /// let hello = &s[0..5];
    /// let world = &s[6..11];
    /// ```
    /// 在一对方括号中指定切片的范围区间[starting_index..ending_index],
    /// 其中stating_index是且切片起始位置的索引值，
    /// ending_index是切片终止位置的下一个索引值。
    /// 切片数据结构在内部存储了指向起始位置的引用和一个描述切片长度的字段， 这个描述切片长度的
    /// 字段等价于ending_index - starting_index。
    ///
    ///
    /// 范围从第一个元素开始， 可以省略两个点号之前的值
    /// ```
    /// let s = String::from("hello");
    ///
    /// let slice = &s[0..2];
    /// let slice = &s[..2];
    /// ```
    ///
    /// 想要包含String中最后一个字节，可以省略双点号之后的值，
    /// ```
    /// let s = String::from("hello");
    ///
    /// let len = s.len();
    /// let slice = &s[3..len];
    /// let slice = &s[3..];
    ///
    /// let s = String::from("hello");
    ///
    /// let len = s.len();
    /// let slice = &s[0..len];
    /// let slice = &s[..];
    /// ```
    ///
    /// ```
    /// fn first_word(s: &str) -> &str {
    ///     let bytes = s.as_bytes();
    ///
    ///     for (i, &item) in bytes.iter().enumerate() {
    ///        if iten == b' ' {
    ///             return &s[0..i];
    ///        }
    ///     }
    ///     &s[..]
    /// }
    ///
    /// fn main() {
    ///     let mut s = String::from("hello world");
    ///     let word = first_word(&s); // immutable borrow occurs here
    ///     
    ///     s.clear(); // error mutable borrow here
    ///     println!("the first word is {}", word);
    ///     //当拥有某个变量的不可变引用时，就无法取得该变量的可变引用
    ///     //由于clear需要截断当前String实例，所以调用clear需要传入一个可变引用。
    /// }
    /// ```
    ///
    /// ### 字符产字面量就是切片
    ///
    /// ```
    /// let s = "hello world!";
    /// ```
    /// 变量s的类型就是&str， 它是一个指向二进制程序特定位置的切片。 正是&str是一个不可变的引用，所以
    /// 字符串字面量自然是不可变的。
    ///
    /// ### 将字符串切片作为参数
    ///
    /// ```
    /// fn main() {
    ///     let my_string = String::from("hello world");
    ///     
    ///     // first_world 可以接收String对象的切片作为参数
    ///     let word = first_word(&my_string[..]);
    ///     
    ///     let my_string_literal = "hello world";
    ///     
    ///     //first_word可以接收字符串字面量的切片作为参数
    ///     let word = first_word(&my_string_literal[..]);
    ///     
    ///     // 由于字符串字面量本身就是切片，所以我们可以在这里直接将它传入函数
    ///     //而不需要使用额外的切片语法
    ///     let word = first_word(my_string_literal);
    /// }
    /// ```
    ///
    /// ### 其他类型的切片
    ///
    /// ```
    /// let a = [1, 2, 3, 4, 5, 6, 7];
    ///
    /// let slice = &a[1..3];
    /// ```
    pub fn var_scope() {
        let _s: &str = "hello";
    }
}
