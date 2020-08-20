/// # 函数
///
/// 函数使用fn关键字来声明，函数的参数需要标注类型，和变量一样
/// 如果函数返回一个值，返回类型必须在箭头 -> 之后指定。
///
/// 函数最后的表达式将作为返回值。 也可以在函数内部使用return语句来提前返回值。
///
/// return甚至也可以在循环或if内部使用
///
pub fn function_example() {
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 { return false; }
        lhs % rhs == 0
    }

    //返回的是单元类型
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n,5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }


    // 当函数返回()时，函数签名可以省略返回类型
    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }

    fizzbuzz(100);
    fizzbuzz_to(10);
}

/// # 方法
///
/// 方法是依附于对象的函数， 这些方法通过关键字self来访问对象中的数据和其他.
///
/// 方法在impl代码块中定义
///
///
pub fn method_example() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl  Point {
        // static method
        // 静态方法不需要被实例调用
        //这类方法一般用作构造器 constructor
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            Point {
                x, y,
            }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // &self是 self: &Self的语法糖，
        // Self是方法调用者的类型
        fn area(&self) -> f64 {
            let Point { x: x1,y: y1} = self.p1;
            let Point { x: x2,y: y2} = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point{ x: x1, y: y1} = self.p1;
            let Point{ x: x2, y: y2} = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // &mut self == self: &mut Self 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // Pair 拥有资源，两个堆分配的整型
    struct Pair(Box<i32>, Box<i32>);

    impl  Pair {
        // 消耗调用者的资源
        // self --- self: Self
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);
        }
    }
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // rectangel.perimeter() == Rectangle::perimeter(&rectangle)
    println!("Rectangle perimeter : {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle 是不可变的，但这个方法需要一个可变对象
    // rectangle.translate(1.0, 0.0);

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // 前面的destroy 调用消耗了pair
    // pair.destroy();
}

/// # 闭包
///
/// Rust中的闭包，也叫做lambda表达式或者lambda，是一类能够捕获周围作用域中变量的函数
/// 一个可以捕获x变量的闭包
///
/// | val | val  + x
///
/// 闭包在临时使用时方便
/// 调用一个闭包和调用一个函数完全相同，不过调用闭包时，输入和返回类型两者都可以自动推导，而且输入变量必须指明。
///
/// - 声明时使用|| 代替（） 将输入参数括起来
/// - 函数体定界符{} 对于单个表达式是可选的，其他情况必须加上
/// - 有能力捕获外部环境的变量
pub fn closure_example() {

    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = | i : i32| -> i32 { i + 1};
    let clousre_inferred = |i| i + 1;
    // 闭包表达式产生的类型就是闭包类型

    let i = 1;
    println!("function: {}",function(i) );
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", clousre_inferred(i));


    let one = || 1;
    println!("closure returning one {}", one());
}

/// # 捕获
///
/// 允许变量捕获灵活地适应使用场合，即可以移动又可以借用变量
///
/// 闭包可以通过以下手段捕获变量：
///
/// - 通过引用： &T
/// - 通过可变引用： &mut T
/// - 通过值： T
///
/// 闭包更倾向于通过引用来捕获变量，并且只在被要求时才使用其他手段
///
///
pub fn capture_closure_example() {
    // let color = "green";
    let color = "color".to_string();

    // 这个闭包打印 color ,他会立即借用（&）， color并将该借用和闭包本身存储到print变量中。
    // color会一直保持被借用状态直到print离开作用域
    //
    // println! 只需要传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需进一步处理就可以使用
    //println！
    {
        let print =|| println!("color : {}", color); //println! 只需要传引用就能使用
        // 所以这里捕获的color是以借用的方式捕获的

        print();
        print();
    }
    println!("color : {}", color);



    let mut count = 0;

    // 这个闭包使count 值增加，要做到这点，它需要得到&mut count或者本身，但是&mut count的要求，没那么严格，所以
    //我们采用这种方式。该闭包立即借用count.
    //
    // inc 前面需要加上mut，因为闭包里存储着一个&mut变量，调用闭包时，该变量
    // 的变化就意味着闭包内部发生了变化。因此闭包需要时可变的
    {
        let mut inc = || {
            count += 1;
            println!("count : {}", count);
        };

        inc();
        inc();
    }
    let reborrow = &mut count;
    println!("reborrow = {}", reborrow);

// -----------------------------------------------------

    println!("--------&mut T------------------");
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count = {}", count);
    };

    // let reborrow = &mut count;
    // 这里打开的话这里的就是第一次的可变的借用的地方


    inc(); // 这里发生的也是可变借用
    inc();

    //是不是闭包也是也是惰性求值的特性呢
    //当没有去调用inc时，不会发生闭包中的可变借用，
    //当闭包被调用后，闭包中就会发生可变借用，这里也就能合理的解释了上面的问题。


    let reborrow = &mut count;

    // inc();

    println!("reborrow = {}", reborrow);

    println!("------------------&T----------------");
    let color = "color";

    let print = || println!("color = {}", color);

    println!("color = {}", color);
    print();
    // ---------------------------------------------------

    let movable = Box::new(4);

    // mem::drop要求T类型本身，所以闭包将会捕获变量的值，
    //这种情况下，可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    //到闭包，因而movable变量在这里立即移动到了闭包中

    let consume = || {
        println!("movable = {:?}", movable);
        std::mem::drop(movable);
    };

    // consume 消耗了该变量，所以该闭包只能调用一次
    consume();

    // ------------------------------------------------------------

    println!("--------------T---------------------");

    let movable = Box::new(5);

    // movable has move in closure
    let consume = || {
        println!("movable : {:?}", movable);
        std::mem::drop(movable);
    };

    // error used moved value
    // println!("movable = {:?}", movable);

    consume(); //use movable of move value
    // consume();


    // 在竖线前使用move会强制闭包取得被捕获变量的所有权

    let haystack = vec![1, 2,3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&2));

    // error use moved value haystack
    // 借用检查不允许在变量被移动后继续使用
    // println!("There're {} elements in vec", haystack.len());

    // 在闭包的签名中删除move会导致闭包以不可变方式借用haystack，因为之后haystack仍然可用，
    //取消上面的注释也不会导致错误
}

/// # 作为输入参数
///
/// 当以闭包作为输入参数时，必须指出闭包的完整类型，他是通过使用以下triat中的一种来指定的。
/// 其受限成都按以下顺序递减
/// - Fn；表示捕获方式通过引用的闭包 &T
/// - FnMut; 表示捕获方式通过可变引用的闭包 &mut T
/// - FnOnce；表示捕获方式通过值的闭包 T
///
/// 顺序之所以是这样的，是因为&T，只是获取了不可变的引用，&mut T则可以改变变量
/// ，T则是拿到了变量的所有权而非借用。
///
/// 对闭包所要捕获的每个变量，编译器在满足使用需求的前提下尽量以限制最多的方式捕获。
///
/// 例如用一个类型说明FnOnce的闭包作为参数，这说明闭包可能采取&T, &mut T或T中的一种捕获方式，
/// 但编译器最终是根据所捕获变量在闭包里使用的情况决定捕获方式。
///
/// 这是因为如果以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量，
/// 注意反过来就不再成立，如果参数的类型说明是Fn，那么不允许该闭包通过&mut T或T 捕获变量
///
/// 三种trait的限制范围来看
///
/// Fn(Fnmut(FnOne))) --- > Fn 包含 FnMut, FnMut 包含 FnOnce
///
///
pub fn closure_as_input_para_example() {
    // 该函数将闭包作为参数并调用它
    fn apply_fnonce<F>(f: F)
        where F : FnOnce() {
        f();
    }

    fn _apply_fn<F>(f: F)
        where F: Fn() {
        f();
    }

    // 当使用 FnMut 是函数参数也要加上mut
    fn _apply_fnmut<F>(mut f: F)
        where F: FnMut() {
        f();
    }

    // 输入闭包，返回一个i32整型的函数
    fn apply_to_3<F>(f: F) -> i32
        where
            F : Fn(i32) -> i32 {
        f(3)
    }

    let greeting = "hello";

    //不可复制的类型
    // to_owned 从借用的数据创建所有权的数据
    let mut farewell =  "goodbye".to_owned();


    //捕获两个变量，通过引用捕获greeting, 通过值捕获farewell
    // this closure implements "FnOnce"
    let diary = || {
        println!("I said {}.", greeting);

        //下文改变了farewell, 因此要求闭包通过可变引用来捕获它
        //现在需要FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        //手动调用drop有要求闭包通过值获取farewell
        //现在需要FnOnce
        std::mem::drop(farewell);

    };

    //以闭包作为参数，调用函数apply_fnonce
    apply_fnonce(diary);

    // apply_fnmut(diary); // error as fnmut to apply

    // apply_fn(diary); // error as fn to apply

    // 所以这里可以看出FnOnce的限制是最小的，
    // Fn是限制最多的， 其次是FnMut限制多

    let double = | x | 2  * x;

    println!("3 doubleed : {}", apply_to_3(double));

}

/// # 类型匿名
///
/// 闭包从周围的作用域中捕获变量是简单明了的，这样会有某些后果吗？确实有，观察一下使用闭包
/// 作为函数参数，这要求闭包是泛型的，闭包定义的方式决定了这是必要的
///
/// ```
/// use core::num::flt2dec::decoder::FullDecoded::Finite;
/// fn apply<F>(f: F)
///     where F : FnOnce() {
///     f();
/// }
/// ```
///
/// 当闭包被定义，编译器会隐式地创建一个匿名类型的结构体，用于存储闭包捕获的变量，同时为这个未知类型
/// 的结构体实现函数功能，通过Fn, FnMut, FnOnce三种trait中的一种
///
/// 若使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是泛型的，然而，使用未限定类型
/// 的参数<T>过于不明确，并且是不允许的。事实上，指明为该结构体实现的是Fn, FnMut, FnOnce中的那种
/// trait, 对于约束该结构体的类型而言就已经足够了。
///
///
pub fn inner_struct_closure_example() {
    // F必须为一个没有输入参数和返回值的闭包实现，Fn, 这和对print的要求恰好一样

    fn apply<F> (f: F)
        where F: Fn() { // FnOnce 包含了FnMut, Fn, FnOnce限制是最少的
        f();
    }

    let mut x = 7;

    // 捕获x到匿名类型结构体中，并为他实现Fn
    //将闭包存储到print中, x 是基本类型实现了Copy, 所以move的话这里的闭包类型是FnOnce，
    //此时闭包中的x是闭包外x的复制
    let print = move || println!("FnOnce x = {}", x); // FnOnce
    // let mut print = || { x += 1;println!("FnMut x = {}",x);  }; // FnMut
    // let print  = || println!("Fn x = {}", x); // Fn

    apply(print);

    println!("x = {}", x);
}

/// # 输入函数
///
///
/// 既然闭包可以作为参数，函数也是可以的，
///
/// 如果声明一个接受闭包作为参数的函数，那么任何满足该闭包的triat约束的函数都可以作为
/// 其参数
///
///
pub fn input_function_example() {

    //定义一个函数，可以接受一个由Fn限定的泛型F，参数并调用它
    fn call_me<F: Fn()> (f: F) {
        f()
    }

    //定义一个满足Fn约束的封装函数
    fn function() {
        println!("I'm a function!");
    }

    let closure = || println!("I'm a closure!!");

    call_me(closure);
    call_me(function);
}

/// # 作为输出参数
///
/// 闭包作为输入参数是可能的，所以返回闭包作为输出参数，也是可能的。
/// 然而返回闭包类型会有问题的，因为目前rust只支持返回具体地类型。
/// 按照定义，匿名的闭包的类型是未知的，所以只有使用impl Trait才能返回一个闭包
///
/// 返回值的合法trait和前面的略有不同
/// - Fn； 前面一样
/// - FnMut; 和前面一样
/// - FnOnce； 目前该类型还是不稳定的，
///
/// 必须使用move关键字，它表明所有的捕获都是通过值进行得。这是必须的，因为在函数退出时，
/// 任何通过引用的捕获都被丢弃，在闭包中留下无效的引用
///
pub fn output_para_example() {

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a :{}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();

}

/// # std中的例子
///
///
/// # Iterator::Any
///
/// Iterator::any是一个函数，若传给他一个迭代器，当其中任一个元素满足谓词时它将返回true, 否则返回false,
/// 谓词时闭包规定的，true/false是闭包作用在元素上的返回值
///
/// ```
/// pub trait Iterator {
///     //被迭代的类型
///     type Item;
///
///
///     // any接受&mut self参数，表明函数的调用者可以借用和修改，但不会被消耗
///     fn any<F>(&mut self, f: F) -> bool
///         // FnMut 表明被捕获的变量最多只能被修改，而不能消耗
///         // Self::Item 指明了被捕获变量的类型， 是迭代器的元素本身的类型
///         where F: FnMut(Self::Item) -> bool {}
///
///         // FnMut 就表示闭包只能通过引用捕获变量，把类型为T的变量作为闭包的参数不代表
///         //闭包会拿走它的值，也可能拿走它的引用
/// }
/// ```
///
///
pub fn any_iterator_example() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1 : {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2 : {}", vec2.into_iter().any(|x| x == 2));
    // println!("vec2 = {:?}", vec2);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    let b = &array2;

    println!("2 in array1: {}", &array1.iter().any(|&x| x == 2));
    // println!("2 in array2 : {}", &array2.iter().any(|&x| x == 2 ));
    println!("2 in array2 : {}", b.into_iter().any(|&x| x == 2));
}
