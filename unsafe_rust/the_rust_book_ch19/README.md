# unsafe rust


不安全rust允许执行4种在安全rust中不被允许的操作包括：

- 解引用裸指针
- 调用不安全的函数或方法
- 访问或修改可变的静态变量
- 实现不安全trait

unsafe关键字不会关闭借用检查器或禁用任何其他rust安全检查：
如果在不安全代码中使用引用，那么该引用依然会被检查。

通常做法：
为了尽可能隔离不安全代码，可以将不安全代码封装在一个安全的抽象中并提供一套安全的API。

## 解引用裸指针

裸指针：裸指针要么是可变的，要么不是可变。

分别写作：
*const T , *mut T., 这里的*是类型的一部分而不是解引用操作。

在裸指针的上下文中，不可变意味我们不能对解引用后的指针赋值。


**裸指针与引用、智能指针的区别在于**

- 允许忽略借用规则，可以同时拥有指向同一内存地址的可变和不可变指针，或者拥有指向同一地址的多个可变指针。
```
let mut num = 4;
let r1  = &num as *const i32; 
let r2 = &mut num as *const i32;
// 创建了同时指同一地址的可变指针和不可变指针，并能通过可变指针来修改数据。
// 如果创建一个指向num的可变引用和不可变引用，就会因为rust的所有权规则而导致编译失败。


//可以在安全代码内合法地创建裸指针，但是不能在安全代码块外解引用裸指针。
// 使用as分别将不可变引用和可变引用强制转换为了对应的裸指针。

unsafe {
    <!-- *r1 = 2;// error -->
    *r2 = 2;
    println!("{}, {}", *r1, *r2); // 2, 2 
}
```
- 不能保证自己总是指向了有效的内存地址。
- 允许为空。
- 没有实现任何自动清理机制。


可以创建一个无法确定其有效性的裸指针
尝试使用任意内存地址的行为是未定义的，这个地址可能有数据，也可能没有数据，编译器可能会
通过优化代码来去掉该次内存访问操作，否则程序就会在运行时出现段错误。


```
//一般不会写这样的代码
let address = 0x012345usize; 
let r = address as *const i32;
```

创建一个指针并不会产生任何危害，只有当我们试图访问它指向的值时才可能因为无效的值
而导致程序异常。


## 调用不安全函数或方法

```
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```
函数前面的unsafe关键字意味着我们需要在调用该函数时手动满足并维护一些先觉条件，因为rust无法对这些条件进行验证。


因为不安全函数的函数体也是unsafe代码块，所以可以在一个不安全函数中执行其他不安全操作而
无须添加额外的unsafe代码块。

创建不安全代码的安全抽象

将不安全代码封装在安全函数中是一个十分常见的抽象。


```
let mut v = vec![1, 2, 3, 4, 5, 6];
let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

println!("a = {:?}, b = {:?}", a, b);
```

```
#[inline]
#[stable(feature = "str_split_at", since = "1.4.0")]
pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str) {
    // is_char_boundary checks that the index is in [0, .len()]
    if self.is_char_boundary(mid) {
        let len = self.len();
        let ptr = self.as_mut_ptr();
        // SAFETY: just checked that `mid` is on a char boundary.
        unsafe {
            (
                from_utf8_unchecked_mut(slice::from_raw_parts_mut(ptr, mid)),
                from_utf8_unchecked_mut(slice::from_raw_parts_mut(ptr.add(mid), len - mid)),
            )
        }
    } else {
        slice_error_fail(self, 0, mid)
    }
}
```