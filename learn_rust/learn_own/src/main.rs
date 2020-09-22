// 1 rust通过所有权机制来管理内存，编译器在编译时就会根据所有权规则对内存的使用进行检查
// 2 堆和栈
//  编译时数据的类型大小是固定的，就是分配在栈上的
//  编译的时候数据类型大小不是固定的，就是分配的堆上的
//
// 3 作用域 : {}
//
// 4 String 内存回收
//
// 5 移动
//
// 6 clone
//
// 7 栈上数据拷贝
//
// 8 函数和作用域
fn main() {
    let x = 1; // x begin, x 分配在栈上

    {
        let y = 2; // y begin, y 分配在栈上 
        println!("x = {}", x);   
        println!("y = {}", y);
    }  // y end

    { 
        let mut s1 = String::from("hello, world"); // 分配在堆上, s1 begin
        s1.push_str("!!!!");
        println!("s1 = {}", s1);

        // 移动， 将s1移动给s2, s1就变为了无效的变量(防止出现二次释放的问题)
        let s2 = s1; // -- value moved here
                //  ^^ s1 moved to s2
        println!("s2 = {}", s2);
        // println!("s1 = {}", s1);

        //clone 
        let s3 = s2.clone();
        println!("s3 = {}", s3);
        println!("s2 = {}", s2);

    }// s1 end , s1.drop() 
    // String类型离开作用域的时候会调用drop方法

    println!("Hello, world!");

    // Copy triat
    let a = 1;
    let b = a;
    println!("a = {}", a);
    println!("b = {}", b);
    // 常用的具有Copy triat有： 
    // 所有的整型
    // 浮点型
    // 布尔类型
    // 字符类型
    // 元组
}// s end
