///# 模式匹配
/// 
/// 模式是Rust中一种用来匹配类型结构的语法。
/// 
/// 将模式与match表达式或者其他工具配合使用可以更好地控制程序流程。
/// 
/// 一个模式统称由一下组件组合而成：
/// - 字面量
/// - 解构的数组、枚举、结构体、或元组
/// - 变量
/// - 通配符
/// - 占位符
/// 
/// 模式被用来与某个特定的值进行匹配，如果模式与值匹配成功，那么下面就可以在代码
/// 中使用这个值的某些部分。
/// 
/// 如果值与模式在形状上相符，那么我们就可以在随后的代码块中使用模式中的命名各种标识符
/// ，而如果不相符，那么模式对应的代码就会被简单地略过。
/// 
/// - 可以使用模式的场景
/// - 不可失败模式与可失败模式之间的区别
/// - 各种模式匹配的语法
/// 
/// ## 所有可能使用模式的场合
/// 
/// ### match分支
/// 
/// 模式可以被用在match表达式的分支中，match表达式在形式上由match关键字、待匹配的值、以及
/// 至少一个匹配分支组合而成，而分支则由一个模式及匹配成功后应当执行的表达式组成。
/// 
/// ```
/// let value = 4;
/// match value {
///     value => println!("{}", value), 
///     // 变量名可以被用来覆盖所有剩余的可能性，
///     //一个能够匹配任何值得变量名永远不会失败
/// }
/// ```
/// 
/// match 表达式必须**穷尽**匹配值的所有可能性，做到这一点可以在最后分支使用全匹配模式。
/// 例子，**变量名可以被用来覆盖所有剩余的可能性，一个能够匹配任何值得变量名永远不会失败**
/// 
/// 特点： **一个特殊的 _ 模式可以被用来匹配所有可能的值，且不将它绑定到任何一个变量上，
/// 这个模式常常陪用作匹配列表中最后一个分支。**
/// _ 模式经常在忽略所有未指定的值时很有用。
/// 
/// ### if let 条件分支
/// 
/// if let 可以看做指匹配单个分支的match表达式来使用。
/// 还有，if let还能添加一个可选的else分支， 这个else在if let 对应的模式没有匹配成功后，执行。
/// 
/// 还可以混用if let, else if以及else if let表达式来匹配。
/// 
/// if let 的缺点不会穷尽所有的可能，编译器也不会报错。
/// 
/// ```
/// let favorite_color : Option<&str> = None;
/// let is_tuesday = false;
/// let age: Result<u8, _> = "34".parse();
/// 
/// if let Some(color) = favorite_color {
///     println!("Using your favorite color, {}, as the background", color);
/// }else if is_tuesday {
///     println!("Tuesday is green day!");
/// }else if let Ok(age) = age {
///     // if let Ok(age) = age 这条语句中引入了新的变量age来存储Ok变体中值，而它覆盖
///     //右侧的同名变量，这意味着必须把判断条件if age > 30 放置到匹配成功后执行的代码块中，
///     //而不能把这个两个条件组合成if let Ok(age) = age && age > 30.
///     // 因为覆盖了同名变量的age只有在花括号后的新作用域中才会变得有效。
/// 
/// 
///     if age > 30 {
///         println!("Using purple as the background coloe");    
///     }else {
///         println!("Using orange as the background color");
///     }
/// }else {
///     println!("Using blue as the background color");
/// }
/// ```
/// 
/// ### while let 条件匹配
/// 
/// while let 会反复执行同一个模式匹配直到出现失败的情形
/// 
/// ```
/// let mut stack = Vec::new();
/// 
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
/// 
/// while let Some(top) = stack.pop() {
///     println!("{}", top);
/// }
/// ```
/// 
/// ### for循环
/// 
/// for语句中紧随关键字for的值就是一个模式,
/// 
/// ```
/// let v = vec!['a', 'b', 'c'];
/// 
/// for (index, value) in v.iter().enumerate() {
///     println!("{} is at index {}", value, index);
/// }
/// 
/// // enumerate方法作为迭代器的适配器，他会在每次迭代器过程中生成
/// // 一个包含值本身即值索引的元组
/// ```
/// 
/// 
pub fn example() {
    let x = 4;
    match x {
        a => println!("{}", a),
    }
} 