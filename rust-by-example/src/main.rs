mod format;
//！ Rust By Example为注释所属的项
/// # Rust By Example RBE
/// 
/// ## Hello World
/// ## 原生类型
/// ## 自定义类型
/// ## 变量绑定
/// ## 类型系统
/// ## 类型转换
/// ## 表达式
/// ## 流程控制 
/// ## 函数
/// ## 模块
/// ## crate
/// ## cargo
/// ## 属性
/// ## 泛型
/// ## 作用域规则
/// ## triat
/// ## 宏
/// ## 错误处理
/// ## 标准库类型
/// ## 标准库更多介绍
/// ## 测试
/// ## 不安全操作
/// ## 兼容性
/// ## 补充
/// 
/// 
fn main() {
    // 将文本打印到控制台

    /*
        这是块注释, /* This is content. */
    */
    println!("Hello, world!");
    println!("I'm a Rustacean!"); 
    
    let x = 5 + /* 90 + */ 5;
    println!("Is 'x' 10 or 100? x = {}",x);
}

