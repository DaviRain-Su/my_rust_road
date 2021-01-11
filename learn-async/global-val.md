# 全局变量

假如我们正在编写网络代码，我们希望有一个全局变量，一个计数器，每次发送一个数据包都给他递增一次：

```rust
// 服务器成功处理的数据包数
static PACKETS_SERVED : usize = 0;
```

这里可以编译通过，不过有个问题： PACKETS_SERVED是不可修改的，因此无法修改它。

Rust会尽可能阻止全局可修改状态。当然使用const声明的常量是不可修改的。静态变量默认是不可修改的，因此无法取得某个值的mut引用。static可以声明为mut，但是再访问它就是不安全的。Rust的这些规则的主要目的都是保证线程安全。（虽然可以通过unsafe代码块修改static的值但是不推荐）

```rust
static mut NUM: u8 = 0;

fn main() {
  unsafe {
    NUM += 1;
  }
  unsafe {
    println!("NUM = {:?}", NUM);
  }
}
```

支持递增PACKETS_SERVED，同时又能保证线程安全的最简单的方式，就是把它改成一个原子整数：

```rust
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);

fn main() {
  for _ in 0..100 {
    PACKETS_SERVED.fetch_add(1, Ordering::SeqCst);
  }
  
  println!("Packets served = {:?}", PACKETS_SERVED);
}
```

这里使用AtomicUsize::new(0)来初始化一个原子类型的整数。

## 一些方法

```rust
//Adds to the current value, returning the previous value.
fn fetch_add(&self, val: isize, order: Ordering) -> isize;

//Subtracts from the current value, returning the previous value.
fn fetch_sub(&self, val: isize, order: Ordering) -> isize;

// Bitwise "and" with the current value.
fn fetch_and(&self, val: isize, order: Ordering) -> isize;


// Bitwise "or" with the current value.
fn fetch_or(&self, val: isize, order: Ordering) -> isize;


// Bitwise "xor" with the current value.
fn fetch_xor(&self, val: isize, order: Ordering) -> isize


// Loads a value from the atomic integer.
fn load(&self, order: Ordering) -> isize;

//Stores a value into the atomic integer.
fn store(&self, val: isize, order: Ordering);

// Stores a value into the atomic integer if the current value is the same as the current value.
fn compare_and_swap(
    &self,
    current: isize,
    new: isize,
    order: Ordering
) -> isize;

//Stores a value into the atomic integer if the current value is the same as the current value.
fn compare_exchange(
    &self,
    current: isize,
    new: isize,
    success: Ordering,
    failure: Ordering
) -> Result<isize, isize>;

```

原子全局变量只能是简单的整数或者布尔值。要创建其他任何类型的全局变量也需要解决两个问题，

- 变量必须通过某种方式保证线程安全，因为要不然就不能是全局变量。考虑到安全，静态变量必须即是Sync, 又是非mut

  Rust有针对性的安全共享可变化值的类型：Mutex, RwLock和原子类型。这些类型即使在被声明为非mut的情况下也是可修改的，这就是他们的目的所在。

- 静态初始化不能调用函数，这意味着声明静态Mutex的显式方式行不通。

```rust
error: expected identifier, found keyword `ref`
  --> src/main.rs:13:8
   |
13 | static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
   |        ^^^ expected identifier, found keyword
```

可以看到这样是有错误的。

解决方法：

可以使用lazy_static包来解决这个问题。

通过lazy_static宏定义的变量可以使用任何表达式来初始化。这个表达式会在变量第一次被解引用时运行，而值会保存下来供后续操作使用。

可以像下面这样使用lazy_static来声明一个全局Mutex

```rust
#[macro_use] extern crate lazy_static;
lazy_static! {
	static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}
fn main() {
  fn i in 0..10 {
    HOSRNAME.lock().unwrap().push_str(&format!("{}", i));
  }
  println!("HOSTNAME = {:?}", HOSTNAME.lock());
}
```

同样的技术也可以用于RwLock和AtomicPtr变量。

使用lazy_static!会在每次访问静态数据时造成微微小的性能损失，因为其实现是用了为一次性初始化而设计的一个低级同步原语std::sync::Once，在后台每次访问懒静态数据，程序都要执行一次原子加载指令已检查初始化是否完成。