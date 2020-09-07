/// 耗时的计算操作只会在一个地方调用，而具体地代码只会在需要计算结果的地方得到执行。
/// 但是老问题还是有，依然在第一个if块中调用了两次闭包，也就是执行了两次耗时的计算操作
/// 进而导致用户的等待时间不合理地被延长了两倍。
/// 在if块中定义一个局部变量来存储闭包结果是可以解决这个问题，但是也可以利用闭包的
/// 特性提供另一种解决方案。
///
/// 使用泛型参数和Fn trait来存储闭包
///
/// Cacher结构体拥有一个泛型T的calculation字段，而trait约束规定的这个T代表一个使用
/// Fn trait的闭包。 
/// value 的类型是Option<u32>。在运行闭包之前，value会被初始化为None, 而当使用
/// Cacler的代码请求闭包执行结果时，Cacher会运行闭包并将结果粗出在value的Some变体中。
/// 之后，如果代码重复请求闭包的结果,Cacher就可以避免再次运行闭包。而降缓存Some变体中的结果
/// 返回给调用者。
use std::collections::HashMap;
pub struct Cacher<T, U, W>
    where 
    U : std::cmp::Eq + std::hash::Hash + Copy,
    W: Copy,
    T : Fn(U) -> W
{
    calculation: T,
    value : HashMap<U, W>,   
}

impl<T, U, W> Cacher<T, U, W> 
    where 
    U : std::cmp::Eq + std::hash::Hash + Copy,
    W: Copy,
    T : Fn(U) -> W 
{
    pub fn new(calculation: T) -> Self {
        Self {
            calculation,
            value : HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> W {
        if self.value.contains_key(&arg){
            *self.value.get(&arg).unwrap()                              
        }else {
            let ret_value = (self.calculation)(arg);
            self.value.insert(arg, ret_value);
            ret_value
        } 
    }
}

/// Cacher 实现的局限性
///
///
/// Cacher 实例假设value方法会为不同的args 参数返回相同的值
/// 
/// 问题在于，我们第一次使用1作为参数来执行c.value时，Cacher实例就将Some(1)
/// 存储在了Self.value 中，在这之后，无论我们在调用value方法时传入的值是什么，它都会
/// 只会返回1。
///
/// 问题的解决是， 让Cacher存储一个哈希表而不是单一的值，这个哈希表使用传入的arg值作为
/// 关键字，并将关键字调用闭包后的结果作为对应的值，
/// value 方法不再简单地判断self.value 的值是Some还是None, 
/// 而是会检查哈希映射里是否存在arg这个关键字。如果存在的话，Cacher 就直接返回对应的值。
/// 如果不存在的话，则调用闭包，使用arg关键字将结果存入哈希表之后再返回
/// 
/// Cacher实现的第二问题是他只能接受一个获取u32类型的参数并返回u32类型的值的闭包，
/// 但我们可能想要缓存的是一个获取字符串切片参数并返回usize值的闭包。
/// 
#[test]
fn call_with_differrnt_values() {

    let mut c  = Cacher::new(|a | {
        a 
    });

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
    println!("v2 = {}", v2);
    
    let mut c1  = Cacher::new(|a : &str| {
        a.len() 
    });

    let temp = String::from("hello, world");
    let v = c1.value(&temp);
    println!("v = {}", v);
}