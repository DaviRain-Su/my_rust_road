# Rust 整体概念梳理


## 变量和类型

Rust中合法的标识符（包括变量名、函数名、triat名等）必须由数字、字母、下划线组成，而且不能以数字开头。这个和很多语言都是一样的。Rust将来也会允许其他Unicode字符作为标识符，还有raw identifier功能，这样可以使关键字作为标识符，比如r#self,这个用途在FFI中最多。


变量的声明: `let variable : i32 = 100; `, 在rust中采用的变量的声明方式不同于以往语言的声明方式，这里先看变量的声明， 变量的声明时，是变量的名字先在前，而变量的类型在后面，`let variable: i32;`就是这个样子。

这样变量声明的好处是，对于语法分析来说分析更为方便，并且在变量声明语句中最为重要的是变量的名字，将变量名提前突出显式变量名的重要性，对于类型是变量名的附加说明，可以通过上下文推导出变量的类型，当让rust的自动类型推导具有局限性，对于不能推导出来的类型，需要手动添加类型说明。

变量声明中的let的使用也是借鉴了函数式语言的思想，let表明的是绑定的含义，表示的是将变量名和内存作了一层绑定关系。在Rust中，一般把声明的局部变量并初始化的语句称为”变量绑定“， 这里强调的是”绑定“的含义，这里和C++/C中的”赋值初始化语句有所不同。


### 变量定义的一些问题

Rust中，每个变量必须被合理初始化之后才能被使用，使用未初始化变量这样的错误，在rust中是不可能出现的。

#### 检查是否声明初始化变量

刚刚上面的`let variable : i32;`这个是声明，而没有给变量赋值，这个在别的语言中可能是行的通的，但是在rust中，编译器直接报错（如果在后面使用这个为赋值（定义）的变量， Rust编译器会对代码作基本的静态分支流程分析，确保变量在使用之前一定被初始化，variable没有绑定任何值，这样的代码会引起很多内存不安全的问题，比如计算结果非预期、程序崩溃，所以Rust编译器必须报错。


```
let variable: i32;
println!("variable  = {}", variable); // error[E0381]: use of possibly unintialized 'variable'
```

#### 检测分支流程是否产生为初始化变量

Rust编译器的静态分支流程分析比较严格。

```
fn main() {
    let x: i32;
    if true {
        x = 1;
    } else {
        x = 2;
    }
    println!("x = {}", x);
}
```

这里的if分支的所有情况都给变量x绑定了值，所以它可以运行。但是如果去掉else分支，编译器就会报错：
```
error: use of possibly unintialized variable : 'x'
println!("x = {}", x);
```

从这里可以看到编译器已经检查出来变量x没有被正确的初始化。去掉else分支后，编译器的静态分支流程分析判断出在if表达式之外的println!也用到了变量x，但并未有绑定任何值得行为。编译器的静态分支流程分析并不能识别if表达式中的条件是true， 所以他要检查所有分支的情况。（这个在程序设计语言领域中有专门去做研究的，比如软件的静态分析，一些参考材料：南京大学的软件分析课程）

要是在把println!语句也去掉的话，则可以正常编译运行，这是因为if表达式之外再也没有任何地方使用变量x， 在唯一使用到x的if表达式中已经绑定了值，所以编译正常。


```
// 有一个例子
fn test(condition: bool ){
    let x: i32; //声明x
    if condition {
        x = 1; //初始化x,这里是初始化
        println!("{}", x); 
    }
    // 如果条件不满足，x没有被初始化

    //但是没有关系，只要这里不使用x就没有事
}
```

#### 检测循环中是否产生未初始化变量


当循环中使用break关键字的时候，break会将分支中的变量值返回。

```
fn main() {
    let x : i32;
    loop {
        if true {
            x = 2;
            break;
        }
    }
    println!("{}", x);// 2
}
```

Rust编译器的静态分支流程分析知道，break会将x的值返回,这样loop循环之外println!可以正常打印x的值

#### 空数组或向量可以初始化变量

当变量绑定空的数组或向量时，需要显式制定类型，否则编译器无法推断其类型。

```
fn main() {
    let a: Vec<i32> = vec![];
    let b: [i32; 0] = [];
}
```
要是不加显式类型标注的话，编译器就会报错：
`error[e0282]: type annotation needed`
空数组或向量可以用来初始化变量，但目前暂时无法用于初始化常量或静态变量。

#### 转移了所有权产生了未初始化变量

当将一个已经初始化的变量y绑定给另外一个变量y2时，Rust会把y看做逻辑上的未初始化变量。
这里的y和y2都是移动语义的变量，移动语义的变量会发生所有权的交接，而值语义，就像其他C++语言默认的都是传值。

```
fn main() {
    let x = 42; //原生类型都是值语义，默认存储在栈上，
    let y = Box::new(4); //变量是由Box装箱到堆上的， Box::new方法在堆上分配内存返回指针
    //并与y绑定，而指针y存储在栈上，
    println!("{}", y);
    let x2 = x;
    let y2 = y;
    //println!("{}", y);//发生了所有权的转移，所以这里的变量y可以看做没有未被初始化的变量
    //但是如果重新给变量绑定上一个值，变量y依然是可用的，这个过程叫做重新初始化
}
```
