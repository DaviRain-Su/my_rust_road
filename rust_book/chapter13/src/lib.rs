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