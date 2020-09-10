# 函数、闭包与迭代器

Rust 是一门混合范式的编程语言，有机的融合了面向对象、函数式和泛型编程范式，而这些并非将这些特性进行简单的堆叠，而是通过高度一致的类型系统融合了这三种范式的思想。（**这里类型系统是如何去做的呢？能融合这些东西。**）

* 面向对象

通过impl关键字配合结构体和trait来实现面向对象中的多态和封装。(**struct类型系统中的积类型，enum类型系统中的和类型。 impl对应在类型系统是什么呢？还有trait对应在类型系统是什么呢？**， 结构体能够实现面向对象的封装特性，triat能够实现面向对象中的多态特性。）

* 函数范式

函数、高阶函数、闭包、模式匹配来实现函数范式中的一些工具。

* 泛型编程

Rust支持零成本静态分发的泛型编程

## 函数

Q1 : 函数的定义及其规范
A1 : fn 关键字后面是函数名称，通常以蛇形命名法snake case 命名函数名。
函数参数必须明确地指定类型，如果有返回值也必须指定返回值的类型。
Rust中的函数不能指定默认值。

Q2: 原生标识操作符(Raw Identifier) r#
A2: 一般来说，在函数定义时不允许直接使用语言中的保留字和关键字等作为函数名。
但是在Rust 2018, 引入了r#,作为关键字的前缀，就可以使用关键字作为函数名，
该语法的主要用于一般用于FFI, 用于避免C函数名和Rust的关键字和保留字重名而引起
冲突。
```
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

Q3: 函数参数传递的规则 
A3: 函数参数的传递有，按值传递，按引用传递。 
- 参数按值传递时，会转移所有权或者执行复制语义。
- 当参数按引用传递时，所有权不会发生变化，但是需要声明周期参数。
**这里按引用传递是不是实际上也是执行引用的值语义操作,这里Copy的是引用这个值，因为引用实现了Copy trait所以不会发生所有权的转移。** 
对于需要声明生命周期的问题；当符合生命周期参数省略规则时，编译器可以通过自动推断补齐函数参数的生命周期参数，否则，需要显式地为参数标明生命周期参数。


Q4: 函数参数的可变与不可变
A4: 函数参数也分为可变和不可变。这里和声明变量时的使用规则是一样的。Rust的函数参数默认不可变，当需要可变操作的时候，需要使用mut关键字来修饰。

Q5: 一个函数的观点
A5: Rust中的每个函数都是自治的，在每一个函数体中,相当于重新开辟了一个新的领域。
将参数传递给函数参数，与let声明一个绑定是一样的规则。

```
// 所有权语义
fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
}
let v = vec![1,2 ,3];
let v = modify(v); // 这里参数传递到modify执行的绑操作是， let mut v = v;是以所有权的方式转移的
println!("{:?}", v); // 1， 2， 3， 42

// 同上面进行对比
let v = vec![1, 2, 3];
let mut v = v; // 这里可以看出很多的问题了吧，使用到了变量遮蔽，将变量重新绑定的位可变的变量，这里是符合所有权的，因为还是只有一个值指向管理的内存，原来的v的所有权给了新的v，只不过这里的两个v是相同的名字而已。
v.push(42);
println!("{:?}", v);


// 借用
pub fn modify_ref_mut(v: &mut [u32]) {
     v.reverse();
}
let mut v = vec![1,2,3];
modify(&mut v); // 这里参数传递到modify执行的绑定是，let v = &mut v; 是以可变引用的方式借用的
println!("{:?}", v); // 3, 2, 1

// 同上面对比
let mut v = vec![1, 2, 3];
let v = &mut v; // 声明了一个可变借用
v.reverse(); // 对可变借用进行操作
println!("{:?}", v);

// 可变借用和不可变借用同时存在
// Case 1 ok 
let mut v = vec![1, 2, 3];

let v1 = &mut v; // 可变借用
v1.reverse();
println!("{:?}", v1); // 不可变借用 

// 此时的 v 已经被v1改变了

let v2 = &v; // 不可变借用， 此时v2做出了一个不可变绑定
println!("{:?}", v2); // 不可变借用， 从上一句到这一句之间不会不发生v的任何变化ok 

// Case 2 ok 
let mut v = vec![1, 2, 3];

let v2 = &v; // 不可变借用
println!("{:?}", v2); // 不可变借用, 从上一句到这一句之间不会发生v的任何变化ok 

// v1 可变借用了v 

let v1 = &mut v; // 可变借用
v1.reverse();
println!("{:?}", v1);// 不可变借用

// Case 3 error 
let mut v = vec![1, 2, 3];

let v2 = &v; // 不可变借用


// v1 可变借用了v
let v1 = &mut v; // 可变借用
v1.reverse();
println!("{:?}", v1);
println!("{:?}", v2); // 不可变借用, 这里的v2和上面声明的v2之间指向的v发生了变化，
及时没有任何的变化，在let v2 = &v; 和println!("{:?}", v2);有任何的可变借用都不可以。所以发生了借用规则的冲突问题。

<!-- error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/lib.rs:75:14
   |
73 |     let v2 = &v;
   |              -- immutable borrow occurs here
74 | 
75 |     let v1 = &mut v;
   |              ^^^^^^ mutable borrow occurs here
...
78 |     println!("{:?}", v2);    
   |                      -- immutable borrow later used here -->
```
从这个例子可以看到的是， modify中参数的是被mut修饰是一个可变的，而传入的v是没有被mut
修饰的，这里参数传递进来进行了一次绑定操作，let mut v = v;
在函数modify的内部v与函数外部的v没有任何的关系，v被转移到了modify的内部。从这里可以
看出每一个函数是一个自治的。

Q6: 变量遮蔽和函数遮蔽
A6: 变量遮蔽：在声明变量绑定之后，如果再次声明同名的变量绑定，则之前的变量绑定会被屏蔽，这叫做变量遮蔽。
函数遮蔽，Rust中在同一个作用域中不能定义多个同名的函数。
可以通过显式地使用花括号将同名的函数分割到不同的作用域中，这样编译器就不会报错。
也就是说，在同一个作用域中不能定义多个同名函数，因为**默认的函数定义只在当前作用域内有效**， 会屏蔽作用域外的同名函数。
```
fn f() { print!(“1); }
fn main() {
    f(); // 
    {
        f(); //3 
        fn f() { print!("3"); } // 屏蔽了外部定义的输出2的f()
    }
    f();
    fn fn() { print!("2"); } // 屏蔽了外部定义的输出1的f(),
}
// output = 232
```

Q7: 函数参数的模式匹配问题
A7: 因为函数中参数等价于一个隐式地let绑定，而let绑定本身就是一个模式匹配的行为。
```
// 变量中的绑定
// 在Rust中，一般把声明的局部变量并初始化的语句称为变量绑定。
struct Point {
    x: i32,
    y: i32,
}
let p = Point{ x: 1, y: 2};
let (mut a, mut b) = (1, 2);
let Point { x: ref a, y: ref b} = p;

// 函数中的绑定

#[derive(Debug)]
struct S {i : i32 }
fn f(ref _s: S) { // 参数使用关键字ref来修饰，这意味着使用模式匹配来获取参数的不可变引用，ref mut 用来匹配可变引用
    println!("{:p}", _s); // print  address
}
let s = S { i: 32 };
f(s); // let ref s = s;
//println!("{:?}", s); // error s have been moved

#[derive(Debug)]
pub struct S { i : i32 }
fn fOwnership(ref mut s : S) {// let ref s = s;, 这里是所有权语义
    s.i = 22;
}
fn fRefmut(ref mut s: &mut S) { // let ref mut s = &mut s;, 这里是可变借用
    s.i = 22;
}
fn fRef(ref s : &S) { // let ref s = &s;, 这里是不可变借用
    println!("{:p}", s);
}

let mut s = S { i : 23};
// let s1 = s;
println!("{:?}", s);
fRef(&s);
fRefmut(&mut s);
println!("{:?}", s);// error s have been moved
```
函数的参数可以使用通配符来忽略参数`fn foo(_: i32) {}`
Rust中的let语句通过模式匹配解构元组， 函数参数也是可以的。
```
fn foo2( _ :i32) {
    println!("foo");
}
foo2(3); 

fn swap((x, y): (&str, i32)) -> (i32, &str) {
    (y, x)
}
let t= ("alex", 18);
println!("{:?}",t);
let t = swap(t); // let (x, y) = t;
println!("{:?}",t);
```

Q8: 函数的返回值
A8: Rust中的函数只能有唯一得到返回值，即便是没有显式返回值的函数，其实也相当于返回一个单元值(), unit. 如果需要返回多个值的话，可以使用元组。
```
fn addsub(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y)
}
let (a, b) = addsub(5,8);
println!("a: {:?}, b: {:?}", a, b);
```

Rust语言提供了return关键字来返回函数中的值。对于只需要返回函数体最后一行表达式所求值的函数，return可以省略。`fn foo() -> i32 { 34 }` 
在某些控制结构中， 比如循环或条件分支，如果需要提前退出函数并返回某些值，则需要显式地使用return关键字来返回，
```
fn gcd(a: u32, b: u32 ) -> u32 {
    if b == 0 { return a; }
    return gcd(b, a % b);
}
let g = gcd(60,40);
assert_eq!(20, g);
```


Q9: 方法和函数的区别
A9:  方法来自面向对象编程范式，在语义上，他代表某个实例对象的行为。方法是通过名字调用
，但是它必须关联一个方法的接受者。
函数只是一段简单的代码，他可以通过名字来进行调用。

Q10: 什么是高阶函数
A10: 在数组中，高阶函数也叫做算子或泛函，比如微积分中的导数就是一个函数到另一个函数的
映射。
在计算机科学中，高阶函数是指以函数作为参数或返回值的函数，它也是函数式编程语言中
最基础的特性。Rust中也支持高阶函数。因为函数在Rust中是一等公民。

Q11: 什么是函数指针？ 
A11: 指向函数的指针，其值为函数的地址. 对于函数指针，可以使用type关键字为其定义别名，
便于提升代码可读性。
```
type MathOp = fn(i32, i32) -> i32;
fn math(op: MathOp, a: i32, b: i32) -> i32 {
    println!("{:p"}, op);
    op(a, b)
}
```

Q12: 
A12: