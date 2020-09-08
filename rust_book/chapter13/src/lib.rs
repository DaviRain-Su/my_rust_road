/// # 函数式语言特性： 迭代器与闭包
/// 
/// 常见的函数式风格编程通常包括
/// 将函数当作参数、将函数作为其他函数的返回值或将函数赋给变量以备之后执行等。
/// 
/// 
/// 谈及的内容：
/// 
/// - 闭包， 一个类似于函数且可以存储在变量中的结构
/// - 迭代器，一种处理一系列元素的方法
/// - 使用闭包和迭代器来改善第12章中的IO项目
/// - 讨论闭包和迭代器的运行时性能
/// 
/// # 闭包： 能够捕获环境的匿名函数
/// 
/// 什么是闭包？
/// Rust中的闭包是一种可以存储变量或作为参数传递给其他函数的匿名函数。
/// 
/// 闭包的特点：
/// 可以在一个地方创建闭包，然后在不同的上下文环境中调用该闭包来完成运算。
/// 闭包可以从定义它的作用域中捕获值。
/// 
/// ## 使用闭包来创建抽象化的程序行为
/// 
/// 覆盖范围：闭包的语法、类型推断、一些相关的trait 
/// 
/// 问题背景：
/// 我们身处的初创公司正在开发一个为用户提供健身计划的应用。使用Rust编写的后端程序
/// 在生成计划的过程中需要考虑到年龄、身体质量指数、健身偏好、运动历史、指定强度值
/// 等因素。具体地算法究竟长什么样子，在这个例子中并不重要，重要的是这个计算过程会消耗
/// 数秒时间。我们希望只在必要的时间调用算法，并且只调用一次，以免让用户等待过久。
/// 
/// 
/// 在函数simulated_expensive_calculation中模拟这一假设的算法计算过程。
/// 
/// ## 闭包的定义，以及相关trait中都没有出现的任何类型标注
/// 
/// ### 闭包的类型推断和类型标注
/// 
/// 与fn定义的函数不同，闭包并不强制要求你标注参数和返回值的类型。Rust之所以要我们
/// 在函数定义中进行类型标注，是因为类型信息是暴露给用户的显式接口的一部分。严格定义
/// 接口有助于所有人对参数和返回值的类型取得明确共识。 
/// 但是，闭包并不会被用于这样的暴露接口：他们被存储在变量中，在使用时既不需要命名，
/// 也不会被暴露给代码库的用户
/// 
/// 也可以为了明确而接受必要的繁杂作为代价，可以为闭包手动添加类型标注
/// 
/// ```
/// let expensive_closure = |num: i32| -> u32 {
///     println!("calculation slowly...");
///     thread::sleep(Duration::from_secs(2));
///     num
/// };
/// ```
///  
/// 对比闭包的语法和函数的语法
/// ```
/// fn add_one_v1 (x: u32) -> u32 { x + 1};
/// let add_one_v2 = | x: u32| -> u32 { x + 1};
/// let add_one_v3 = |x| { x + 1};
/// let add_one_v4 = |x| x + 1;
/// ```
/// 
/// 闭包定义中的每一个参数及返回值都会被推导为对应的具体类型。
/// 
/// ```
/// let example_closure = |x | x;
/// 
/// // 试图使用两种不同的类型调用同一种需要类型推导的闭包
/// let s = example_closure(String::from("hello"));
/// let n = example_closure(5);
/// ```
/// 
/// 我们没有闭包exmaple_closure中的x提供类型标注，第一个调用x推断为String类型
/// 第二个调用x推断为u32类型，那么发生编译错误， 这是因为触发了类型不匹配导致的。
/// String != u32
/// 
/// 
/// ## 使用泛型参数和Fn trait 来存储闭包
/// 
/// 在健身计划生成应用中，代码依然不必要地多次调用了耗时的计算闭包。这个问题的一个解决方案是将
/// 耗时闭包的结果存储至变量中，并在随后需要结果的地方使用该变量而不是继续调用闭包。但需要
/// 注意的是，这种方法可能会造成大量的代码重复。
/// 
/// 另一种的解决方案: 创建一个同时存放闭包以及闭包返回值的结构体，这个结构体只会在我们需要获得
/// 结果值时运行闭包，并将首次运行闭包时的结果缓存起来，这样余下的代码就不必再负责存储结果，
/// 可以直接复用该结果。 **这一模式一般被称作记忆化(memorization)或惰性求值(lazy evaluation)**
/// 
/// 为了将闭包存储在结构体中，我们必须明确指定闭包的类型，因为结构体各个字段的类型在定义
/// 时就必须确定。
/// 
/// 但是需要注意的是，每一个闭包实例都有他自己的匿名类型。即， 即便两个闭包拥有完全相同的签名
/// ，他们的类型也被认为是不一样的。 
/// 
/// 为了在结构体、枚举或函数中使用闭包， 需要使用泛型和triat约束
/// 
/// 标准库中提供了一系列Fn triat。而所有的闭包都至少实现了Fn, FnMut, FnOnce中的一个trait。
/// 
/// 
/// 
pub mod cacher;
pub mod example_closure {
    use std::thread;
    use std::time::Duration;

    // fn simulated_expensive_calculation(intensity: u32) -> u32 {
    //     println!("Calculation slowly ...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    // 根据输入数据打印健身计划的业务逻辑，他多次调用了simulated_expensive_calculation
    pub fn generate_workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!(
                "Today, do {} pushups!", // 今天做「」俯卧撑
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps", // 接下来做「」 个仰卧起坐
                simulated_expensive_calculation(intensity)
            );
        }else {
            if random_number == 3 {
                println!("Take a break today! Remember to study hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!", // 今天跑步「」分钟
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }
    
    // 优化目标， 通过代码重构让simulated_expensive_calculation函数调用一次
    // 希望在不必要时避免调用这个耗时的计算函数，在必要时也最多调用一次
    // fn simulated_expensive_calculation(intensity: u32) -> u32 {
    //     println!("Calculation slowly ...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    // 根据输入数据打印健身计划的业务逻辑，他多次调用了simulated_expensive_calculation
    
    // 通过函数进行重构
    // 缺点，任何调用都要先去执行这个函数的调用
    pub fn generate_workout_exp1(intensity: u32, random_number: u32) {
        let expensive_result = 
            simulated_expensive_calculation(intensity);
        
        if intensity < 25 {
            println!(
                "Today, do {} pushups!", // 今天做「」俯卧撑
                expensive_result
            );
            println!(
                "Next, do {} situps", // 接下来做「」 个仰卧起坐
                expensive_result
            );
        }else {
            if random_number == 3 {
                println!("Take a break today! Remember to study hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!", // 今天跑步「」分钟
                    expensive_result
                );
            }
        }
    }

    // 我们希望在程序中将代码定义在一处，但只有真正需要结果时才执行相关代码。
    
    // 使用闭包存储代码来进行重构
    // 相对于每次在if块之前调用simulated_expensive_calculation函数，我们可以
    // 定义一个闭包，并将闭包而不是函数的计算结果存储在变量中。
    pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("Calculation slowly ...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    // 根据输入数据打印健身计划的业务逻辑，他多次调用了simulated_expensive_calculation
    pub fn generate_workout_exp2(intensity: u32, random_number: u32) {
       // 这条let语句意味着expensive_closure变量存储了一个匿名函数的定义，而不是
       // 调用该匿名函数而产生的返回值。我们之所以使用闭包时因为想要在一个地方定义要
       // 调用的代码，将其存储起来，并在稍后的地方调用它。
        let expensive_closure = |num : u32| {
            println!("Calculating slowly ...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!(
                "Today, do {} pushups!", // 今天做「」俯卧撑
               expensive_closure(intensity)
            );
            println!(
                "Next, do {} situps", // 接下来做「」 个仰卧起坐
               expensive_closure(intensity)
            );
        }else {
            if random_number == 3 {
                println!("Take a break today! Remember to study hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!", // 今天跑步「」分钟
                    expensive_closure(intensity)
                );
            }
        }
    }
    use crate::cacher::Cacher;
    pub fn generate_workout_exp3(intensity: u32, random_number: u32) {
         let mut expensive_result = Cacher::new(
             |num | {
                 println!("calculating slowly ...");
                 thread::sleep(Duration::from_secs(2));
                 num
             }
         );
         
         if intensity < 25 {
             println!(
                "Today, do {} pushups!", // 今天做「」俯卧撑
                expensive_result.value(intensity)
             );
             println!(
                "Next, do {} situps", // 接下来做「」 个仰卧起坐
                expensive_result.value(intensity)
             );
         }else {
             if random_number == 3 {
                 println!("Take a break today! Remember to study hydrated!");
             } else {
                 println!(
                    "Today, run for {} minutes!", // 今天跑步「」分钟
                    expensive_result.value(intensity)
                 );
             }
         }
     } 

    #[test]
    fn basic(){
        // 一个来自用户的强度值，它会在用户请求健身计划时被要求制定，以便确定用户
        // 想要低强度训练还是高强度训练
        let simulated_user_specified_value = 25;
        // 一个随机数，他会让输入的健身计划产生些许变化
        let simulated_random_number = 7;

        generate_workout_exp3(
            simulated_user_specified_value,
            simulated_random_number
        );
    }
}

/// # 使用闭包捕获上下文
/// 
/// 
/// 闭包可以捕获自己所在的环境并访问自己被定义的作用域中的变量
/// 
/// ```
/// let x = 4;
/// 
/// let equal_to_x =  | z |  z == x;
/// 
/// let y  = 4;
/// assert!(equal_to_x(y));
/// ```
/// 
/// ```
/// let x = 4;
/// 
/// //fn equal_to_x(z: i32) -> bool { z == x }; //error 
/// 
/// let y = 4;
/// //因为函数不被允许从环境中捕获变量，所以定义和使用函数永远不会
/// //产生这类开销 
/// //assert(equal_to_x(y));
/// ```
/// 
/// 当闭包从环境中捕获值时，他会使用额外的空间来存储这些值以便在闭包体内使用。
/// 在大多数场景下，我们都不需要在执行代码时捕获环境，也不想要为这些场景产生
/// 额外的内存开销。因为函数不被允许从环境中捕获变量，所以定义和使用函数永远不会
/// 产生这类开销.
/// 
/// 闭包捕获环境变量的三种方式，这和函数接收参数的三种方式是完全一致的：
/// 1.获取所有权
/// 2.可变借用
/// 3.不可变借用
/// 
/// 
/// 这三种方式被分别编码在如下所示3三种Fn 系列的trait中。
/// 
/// - FnOnce. 意味着闭包可以从他的封闭作用域中，也就是闭包所处的环境中，消耗
/// 捕获的变量。为了实现这一功能，闭包必须在定义是取得这些变量的所有权big将
/// 他们移动到闭包中，这也是FnOnce中Once一词的含义：因为闭包不能多次获取并消耗
/// 同一变量的所有权，所以它只能被调用一次。
/// - FnMut, 可以从环境中可变的借用值并对他们进行修改。
/// - Fn 可以从环境中不可变地借用值。
/// 
/// - 当你创建闭包时，Rust会基于闭包从环境中使用值的方式来自动推导出他需要使用的trait。
/// 所有闭包都会自动实现FnOnce, 因为他们至少都可以被调用一次，那些不需要移动被捕获变量的
/// 闭包还会实现FnMut, 而那些不需要对捕获变量进行可变访问的闭包则同时实现了Fn. 
/// 
/// 假如希望强制闭包获取环境中值的所有权，那么可以在参数列表前添加move关键字，这个特性在把
/// 闭包传入新线程时相当有用，他可以将捕获的变量一并移动到新线程中去。
/// 
/// ```
/// let x = vec![1, 2, 3];
/// 
/// let equal_to_x = move | z | z == x;
/// 
/// // println!("can't use x heree: {:?}", x); // error 
/// 
/// let y = vec![1, 2, 3];
/// 
/// assert!(equal_to_z(y));
/// ```
///  
/// 因为添加了move关键字，所以x的值会在定义闭包时移动到闭包中，由于闭包拥有了
/// x的所有权，所以main函数就无法再printn!语句中使用x。
/// 
/// 
/// 在大部分情况下，当你需要指定某一个Fn系列的trait时，可以先尝试使用Fn Trait,
/// 编译器会根据闭包体中具体情况来告诉你是否需要FnMut, FnOnce.  
/// 
/// 
/// 
/// 
/// 
pub mod capture_value_in_scope {

    #[test]
    fn test_capture_value_in_scope(){
        let x = 4;
        
        let equal_to_x = | z |  z == x;        
        let y  = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn test_function_captuer_in_scope(){
        let _x = 4;

        // fn equal_to_x(z: i32) -> bool { z == x }; // error
        //    --> src/lib.rs:300:46
        // |
        // 300 |         fn equal_to_x(z: i32) -> bool { z == x };
        // |                                              ^
        // |

        let _y = 4;
        // assert!(equal_to_x(y));
    }
    #[test]
    fn test_move_use () {
        let x = vec![1, 2, 3];

        let equal_to_x = move | z : Vec<i32> | z == x;

        // println!("can't  use x here : {:?}", x); //error 

//         error[E0382]: borrow of moved value: `x`
//    --> src/lib.rs:357:46
//     |
// 353 |         let x = vec![1, 2, 3];
//     |             - move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
// 354 | 
// 355 |         let equal_to_x = move | z : Vec<i32> | z == x;
//     |                          ---------------------      - variable moved due to use in closure
//     |                          |
//     |                          value moved into closure here
// 356 | 
// 357 |         println!("can't  use x here : {:?}", x); //error 
//     |                                              ^ value borrowed here after move

        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}


/// # 使用迭代器处理元素序列
/// 
/// 迭代器允许依次为序列中的每一个元素执行某些任务。 迭代器会在这个过程中负责遍历每一个
/// 元素并决定序列何时结束。
/// 
/// 在Rust中，迭代器是惰性的，这意味着创建迭代器后，除非你主动调用方法来消耗并使用迭代器，
/// 否则他们不会产生任何实际效果。
/// 
/// 
/// 迭代器用统一逻辑来灵活处理各种不同种类的序列，而不仅仅只是像动态数组一样可以进行
/// 索引的数据结构。例如还有String， HashMap, 等等
/// 
/// 
/// ## Iterator triat 和next方法
/// 
/// 所有的迭代器都实现了定义于标准库中的Iterator trait, 
/// 
/// ```
/// pub trait Iterator {
///     type Item;
///     
///     fn next(&mut self) -> Option<Self::Item>;
/// }
/// ```
/// 
/// type Item; Self::Item, 是定义在Iterator triat 中的关联类型
/// Item类型是迭代器返回元素的类型。
/// 
/// Iterator trait 只要求实现者手动定义一个方法, next方法，他会在每次调用
/// 时返回一个包括Some中的迭代器元素，并在迭代器结束时返回None
///
/// 
/// iter方法生成的是一个不可变引用的迭代器，我们通过next取得的值实际上是指向
/// 动态数组中各个元素的不可变引用。如果需要创建一个取得所有权并返回元素本身的迭代器，
/// 可以使用into_iter方法，类似的，如果需要可变引用的迭代器，可以调用iter_mut方法
/// 
/// 
/// ## 消费迭代器的方法
/// 
/// 调用next的方法也被称为消费适配器， 因为他们同样消耗了迭代器本省， 
/// 
/// sum方法，这个方法会获取迭代器的所有权并反复调用next来遍历元素，进而导致迭代器被消耗。
/// 在迭代过程中，他会对所有元素进行求和并在迭代结束后将和作为结果返回。
/// 
/// 
/// ## 生成其他迭代器的方法
/// 
/// Iterator trait 中还定义了另外一些被称为迭代器适配器的方法，这些方法可以使
/// 你将已有的迭代器转换成其他不同类型的迭代器。 
/// 
/// 可以链式地调用多个迭代器适配器完成一些复杂的操作，同时保持代码易于可读性。
/// 
/// 但是所有的迭代器都是惰性的，所以你必须调用一个消耗适配器的方法才能从迭代器适配中获得结果。
/// 
/// 
/// ## 使用闭包捕获环境
/// 
/// 使用filter迭代器适配器来演示闭包捕获环境的一种常见用法，
/// 
/// 迭代器的filter方法会接受一个闭包作为参数，这个闭包会在遍历迭代器中的元素时返回
/// 一个布尔值，而每次遍历的元素只有在闭包返回true时才会被包含在filter生成的
/// 新迭代器中。
/// 
/// 
/// ## 使用Iterator triat来创建自定义迭代器
/// 
/// 只需要提供一个next方法，的定义就可以实现Iterator trait, 一旦完成该方法定义，就可以使用
/// 其他一切拥有默认实现的Iterator triat提供的方法。
/// 
/// 
pub mod test_iterator {

    #[test]
    fn basic() {
        let v1 = vec![1, 2, 3];

        // 创建一个用于遍历动态数组v1的迭代器
        let _v1_iter = v1.iter();

        // 在for循环中使用迭代器来依次遍历元素
        // 在这个for循环中, 不要求v1_iter可变，是因为循环取得了
        // v1_iter的所有权并在内部使得他可变了。
        for val in _v1_iter {
            println!("Got : {}", val);
        }
        println!("v1 = {:?}", v1);
    }
    #[test]
    fn iterator_demonstration(){
        let v1 = vec![1, 2, 3];

        // 这里的v1_iter 必须是可变的，因为调用next方法改变了迭代器内部用来
        // 记录序列位置的状态，也就是说，这段代码消耗或使用了迭代器，每次调用
        // next都吃掉了迭代器中的一个元素。
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total : i32 = v1_iter.sum();
        
        println!("total = {}", total);
    }

    #[test]
    fn iterator_map(){

        let v1 = vec![1, 2, 3];

        // v1.iter().map(|x| x+1);
//         warning: unused `std::iter::Map` that must be used
//    --> src/lib.rs:492:9
//     |
// 492 |         v1.iter().map(|x| x+1);
//     |         ^^^^^^^^^^^^^^^^^^^^^^^
//     |
//     = note: `#[warn(unused_must_use)]` on by default
//     = note: iterators are lazy and do nothing unless consumed

        let v2 : Vec<_> = v1.iter().map(|x| x + 1).collect();
        
        println!("v2 = {:?}", v2);
    }
    #[derive(Debug, PartialEq)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shopes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        
        // into_iter来创建可以获取动态数组所有权的迭代器，
        // 使用filter来将迭代器适配成一个新的迭代器，新的迭代器只会包含闭包返回值为
        // true的那些元素。
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 13, style: String::from("sandal")},
            Shoe { size: 10, style: String::from("boot")},
        ];

        let in_my_size = shopes_in_my_size(shoes, 10);

        println!("in my size = {:?}", in_my_size);
    }

    // 创建自定义类型的迭代器
    #[derive(Debug, Copy, Clone)]
    pub struct Counter {
        count: u32,
    }

    impl  Counter {
        pub fn new()-> Self {
            Self { count : 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            }else {
                None
            }
        }
    }
    #[test]
    fn calling_next_directly() {
        let counter = Counter::new();

        // assert_eq!(counter.next(), Some(1));
        // assert_eq!(counter.next(), Some(2));
        // assert_eq!(counter.next(), Some(3));
        // assert_eq!(counter.next(), Some(4));
        // assert_eq!(counter.next(), Some(5));
        // assert_eq!(counter.next(), None);

        for val in counter {
            println!("{}", val);
        }

        println!("count = {:?}", counter);
    }        

    // 使用其他的Iteratoe方法
    // 
    // 我们只需要提供next方法的定义便可以使用标准库中那些拥有默认实现Iterator triat
    // 方法，因为这些方法都依赖于next方法的功能。

    #[test]
    fn using_other_iterator_trair_methods() {
        // 将一个Counter实例产生的值与另一个Counter实例跳过首元素后的值一一配对
        // 接着讲配对的两个值相乘， 在最后再对乘积中能被3整除的那些数字求和。

        // 注意， zip方法只会产生4对值，它在两个迭代器中的任意一个返回None时结束，
        // 所以理论上第五对值(5, None)永远不会被生成出来。
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)|a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        // Count::new()        -> 1, 2, 3, 4, 5 
        // Count::new().skip() -> 2, 3, 4, 5    
        // a * b               -> 2, 6, 12, 20, 5 * None
        // x % 3 == 0          -> false, true, true, false,
        // sum                 -> 6 + 12
        println!("sum = {}", sum);
    }

}