pub mod  davirain;

/// # 所有权系统 
/// 
/// Rust中的智能指针， 在堆上开辟内存空间，并拥有其所有权，通过存储于栈中的指针来管理堆内存。
/// 智能指针的RAII机制利用了栈的特点，在栈元素被自动清空时自动调用析构函数，来释放
/// 智能指针所管理的堆内存空间。
/// 
/// 现在C++的RAII机制解决了无GC自动管理的基本问题，但没有解决全部问题。
/// ```
/// #include <iostream>
/// #include <memory>
/// 
/// using namesapce std;
/// 
/// int main() {
///     unique_ptr<int> orig(new int(5));
///     cout << *orig << endl;
///     auto stolen = move(orig); // 将原来的unique_ptr指针赋予了stolen
///     //并转让了所有权，原来的orig则变成了空指针，而对空指针解引用是不安全的，
///     // 所以C++代码运行时就会抛出段错误segmentation fault, 
///     cout <<  *orig << endl; // 段错误
/// }
/// ```
/// 
/// ```
/// fn main() {
///     let orig = Box::new(5);
///     println!("{}", *orig);
///     let stolen = orig; 
///     println!("{}", *orig); // error, use of moved value;
/// }
/// ```
/// 
/// 现代C++中的RAII机制虽然也有所有权的概念，但其作用范围非常有限，仅智能指针有
/// 所有权，并且现代C++编译器也并没有依据所有权进行严格检查，所以才会出现解引用
/// 空指针的运行时错误。而Rust中，所有权是系统的概念，是Rust语言中的基础设施。
/// Rust中的每个值都必定有一个唯一控制者，即，所有者。所有权的转移都是按系统性的
/// 规则隐式自动完成的。
/// 
/// 
pub mod example {

}