/// # 前台
/// 
/// 服务客户，处理订单，结账及调酒
/// 
/// ```
/// mod front_of_house { //mod 定义模块， front_of_house是模块的名字， 花括号中是包裹的模块体模块中可以继续定义模块，
///     mod hosting { //模块hosting
///         fn add_to_waitlist() {}
///         fn seat_at_table() {}
///     }
///
///     mod serving { //模块serving
///         fn take_order() {}
///         fn serve_order() {}
///         fn take_payment() {}
///     }
/// }
/// ```
/// 模块中可以包含其他条目的定义，结构体、枚举体、常量、triat、函数
/// 使用模块将相关的定义分到一组，并根据他们的关系指定有意义的名称。
/// 
/// crate 
/// |----front_of_house 
/// |--------hosting 
/// |------------add_to_waitlist
/// |------------seat_at_table
/// |--------serving
/// |------------take_order
/// |------------serve_order
/// |------------take_payment
/// 
/// 整个模块树都被放置在一个名为crate的隐式根模块下。
pub mod hosting;
pub mod serving;