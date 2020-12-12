

# 必看 Substrate 入门课程导读

# 必看 常见问题回答

# 第1课 【1- 1 什么是区块链】

# 第1课 【1-2 如何参与开源项目】

# 第2课 【2-1 波卡介绍】

# 第2课 【2-2 substrate 介绍】

# 第2课 【2-3 node templaet】

第3课 [web3 大会： Energy Web 概览]

第3课 [Web3大会 : ChainSafe CEO@Aidan Hyman 主题演讲]

第3课【Web3 大会 对话佳士得：区块链文化、NFT与艺术市场】

第3课【Web3 大会：Gavin Wood 主题演讲】

第3课【Web3 大会：Gavin Wood 社区AMA】

第3课【 Web3大会 Parity公共事务主管 Peter：Web3.0 与媒体】

第3课【Web3 大会：Chainlink主题演讲】

第3课【Web3 大会：Behindthecoder】

# 第4课【4-1：Rust 入门】

## 课程内容

- 数据类型
- 流程控制
- 异常处理
- Cargo 项目管理工具
- Node Template 代码导读

##  整数类型

u8, u16, u32, u64, u128
i8 , i16, i32, i64, i128

## 浮点， 布尔， 字符

f32, f64, true, false, Char

## Tuple, Array, Vector, Map, Set

(1, 2, 3)

[1, 2, 3]

Vec![1, 2, 3]

HashMap, BTreeMap, HashSet, BTreeSet

## Struct

```rust
struct TreeNode {
  data: i32, 
  left: Box<TreeNode>,
  right: Box<TreeNode>,
}
```

## Enum 

```rust
enum Person {
  Unknown, 
  Anonymous(bool),
  Name(String),
}
```



## String , str

```rust
// pub strcut String {
// 	vec: Vec<u8>,
// }
let s = String::from("hello world");
// string slice , str is [u8]
let s = "hello, world!";
```

## 流程控制

if /else , match 

## 循环

loop, while/ for in 

## 异常处理 Option 

option, Result, panic

## Rust 包管理工具 Cargo 

常用命令

Cargo new project-name --lib 

Cargo build --release // debug / release 模式

Cargo run 

Cargo check 

Cargo test

## Node Template 代码导读

1. Node Template代码分析
2. Code base https://github.com/substrate-developer-hub/substrate-node-template
3. 知乎文章关于Node Template https:://zhuanlan.zhihu.com/123167097



# # 第4课【4-2：Rust 入门】

# 第4课【4-3：Rust 入门】

# 技术分享会：Ink 智能合约入门

# 第5课【5-1：Rust 进阶】

## 枚举 

Option, Result,

表示多个变体（同一类型的多种可能性）。变体可以包含任意类型的数据，字符串，数值，结构体，其他的枚举等

也可以对枚举定义方法，实现triat。

Option ： 存在某种类型的值， 不存在有效的该类型的值

is_some(), map(), map_or(), unwrap()

Result<T,E> ， 表示的正确的某种类型的值， 表示错误信息的另一类型的值

is_ok(), map(), map_or(), unwrap() 

模式匹配



## 所有权

## 所有权概念和规则

任何程序的运行都需要依赖内存，典型的内存管理机制有：

- 垃圾收集器， java， Go
- 手动分配和释放， C/C++
- 编译时的所有权系统, Rust (Ownership)， 只有Rust

Ownership 是Rust区别与其他编程语言最核心的特性，它保证了代码的内存安全性，且性能卓越



 

### 所有权规则

- 任何值都有一个变量与之对应，称之为owner
- 某一时刻，只能有一个owner
- 当owner退出作用域后，值被丢弃



## 所有权转移

如果是简单类型比如数值，bool， 赋值会发生数据拷贝，而不是转移所有权

## Copy & Clone 

- 赋值时可以通过数据拷贝、克隆，不去转移现有数据所有权，

- Copy， 适用于基本类型或完全由基本类型组成的复杂类型，u32, bool , char, tuples,

- Clone，数据存储在堆上，在堆上克隆一份新的, String, HashMap, Vec



- 自动派生Copy & Clone 接口，#[derive(Copy, Clone)]
- 一般不需要显式实现Copy
- Clone更慢， Clone()不可缺省





## 函数与所有权

- 和赋值类似，将值传递给函数也会转移所有权或copy
- 返回值可以把函数内变量对应值的所有权转移至函数外



## Reference & Borrowing

当需要使用某个值，但又不希望获取所有权时，可以通过引用

- 在变量名前放置&符号，获取值的引用
- Borrowing : 函数参数为引用
- 默认是不可变的(immutable), 可变引用为&mut 
- 引用的作用域，在最后使用的地方结束，而不是大括号的末尾



## Slice类型

和引用类似，slice也不拥有值的所有权，用于引用集合内的部分连续数据

- 与值绑定，当退出作用域，需要清空时，slice也同时失效
- 定义slice: &name[start..end], 不包含end
- 类型签名： &str为string slice, &[T] 为Vector / array slice 



所有权和引用如何保证内存安全性？

- 拥有所有权的变量退出作用域时，自动清空值的内存空间
- 同一时间，最多有一个可变引用，或者多个不可变引用
- 编译时不允许空指针
- 通过slice引用值的一部分

## Rust 泛型 Trait， 生命周期



## 泛型

为什么使用trait

- 减少相似代码
- 通过抽象，增加扩展性
- 常用语结构体，枚举和函数签名

使用泛型的好处

- 编译时使用具体类型代替，不影响执行效率
- 过多的泛型，可读性降低



## triat

- 抽象了某种功能或行为

- 对泛型添加了triat bound 表示泛型参数满足某种约束
- 多个参数时，trait bound 保证类型一致
- 多个类型约束时，使用加号 + 
- where关键字



## 生命周期



- 每个变量都有生命周期 lifetime
- 生命周期确保引用的有效性，防止出现空指针



引用的生命周期

- 多数情况，可由编译器推断出来
- 推断不出时，使用泛型指定多个引用之间生命周期的关系

- 返回值的引用生命周期必须来自参数
- 如果来自函数内，会造成空指针，编译不通过



引用生命周期的缺省规则：

- 为每一个引用参数类型，添加生命周期泛型
- 生命周期只有一个时，所有引用类型的返回值使用此生命周期
- 生命周期泛型有多个时，且其中一个为&self或&mut self，所有引用类型的返回值使用它对应的生命周期

- 此类结构体的作用域不能在引用的值之外



静态生命周期

- 此类引用的有效性是程序的整个执行周期
- string literal 默认是static
- 谨慎使用static，修复代码可能存在的空指针或者引用生命周期不匹配

## Rust项目管理



## package, crate, module 介绍

## 功能模块引入

## Workspace



为什么要做项目管理

- 组织大量代码
- 封装无需外部关心的实现细节
- 代码复用



Rust项目管理的组件

- package ： cargo 工具用来构建 编译 测试的空间
- crate ： 工具库(src/lib.rs)或可执行程序(src/main.rs)
  - rand
  - serde
  - diesel
- moudle： 在crate里组织代码，控制是否对其他模块可见

模块引入

- use 
- pub use 
- crate
- self
- super
- as 

Workspace 

- 管理多个library， binary 
- 共享cargo.lock和输出目录
- 依赖隔离



# Runtime 宏介绍

## rust宏

宏是一种元编程的方式，常见的还有java的反射，Rust提供了两种宏：

- 声明宏
- 过程宏



substrate 为什么使用宏

为了简化Runtime的开发，substrate使用宏建立了一套DSL（Domain Specific Language) 设计合理的DSL 可以：

- 很好的被用户理解
- 代码更加简洁，提升效率
- 解放应用开发者，只需实现业务组件



数据库

点对点网络

密码学 

WASM 执行环境

## substrate runtime 定义

State A --> 交易---> Runtime ----> State B 



## Substrate Module 定义

## Substrate Runtime 定义

内置的模块也被称为Pallet 调色板

## Runtime常用的宏



使用substrate 进行Runtime模块开发的过程中，常用到的宏有;

- decl_storage 定义存储单元
- decl_module 包含可调用函数
- decl_event 事件
- decl_error 错误信息
- Constrcut_runtime 添加模块到 runtime 

## decl_stroage 

不管是web2,0 传统的互联网应用，还是采用区块链技术的web3.0 应用，关键数据都需要存起来

Decl_stroage宏，就是用来定义runtime模块的存储单元

```rust
/// The pallet's configuration trait 
pub triat Trait: system::Trait {
  /// The overaching event type
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
  triat Store for Module<T: Trait> as TemplateMoudle {
    Something get(fn something) : Option<u32>;
  }
}
```

数据类型 ： 

- 单值
- 映射
- 双键映射

## decl_module 

区块链的链上状态变化由交易触发，substrate 不仅支持自定义的存储数据结构，还支持自定义的交易，例如转账，注册身份，投票等等，也叫做extrinsic 外部交易



decl_module 用来定义模块里可调用函数，每一个外部交易都会触发一个可调用函数，并根据交易体信息也就是函数参数，更新链上状态。



```rust
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    type Error = Error<T>;
    fn deposit_event() = default;
  	
    #[weight = 10_000]
    pub fn do_something(origin, something: u32)-> dispatch::DispatchResult {
      // -- snip -- 
      Something::put(something);
      Self::deposit_event(RawEvent::SomethingStored(something, who));
      Ok(())
    }
  }
}

// -- snip -- 
#[weight = 10_000]
pub fn cause_error(origin) -> dispatch::DispatchResult {
  // -- snip -- 
  match Something::get() {
    None => Err(Error::<T>::NoneValue)?,
    Some(old) => {
      let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
      Something::put(new);
      Ok(())
    }
  }
}
```



Runtime模块里存在保留函数，除了deposit_event之外，还有

- on_initialize, 在每个区块的开头执行
- on_finalize， 在每个区块结束时执行
- Offchain_worker： 开头且是链外执行，不占用链上的资源
- On_runtime_upgrade: 当有runtime升级时才会执行，用来迁移数据

## decl_event 

区块链是一个异步系统， runtime通过触发事件通知交易执行结果。

```rust
decl_event! {
  pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
    SomethingStored(u32, AccountId),
  }
}
// -- snip -- 
Self::deposit_event(RawEvent::SomethingStored(something, who));
```

## decl_error 

```rust
decl_error! {
  pub enum Erro for Module<T: Triat> {
    /// Value was None
    NoneValue,
    /// Value reached maximum and cannot be incremented further
    StorageOverflow,
  }
}
```



可调用函数里的错误类型

- 不能给他们添加数据
- 通过metadata暴露给客户端
- 错误发生时触发system.ExtrinsicFailed事件，包含了对应错误的信息

```rust
impl template::Trait for Runtime {
  type Event = Event;
}

construct_runtime! (
	pub enum Runtime where 
  	Block = Block,
  	NodeBlock = opaque::Block,
  	UncheckedExtrinsic = UncheckedExtrinsic
  {
    // -- snip -- 
    TemplateModule: template::{ Module, Call, Storage, Event<T> };
  }
);
```



## cargo expand

将宏里面的代码展开，得到Rust的标准语法

Https://github.com/dtolnay/cargo-expand

https:://github.com/kaichaosun/play-substrate/blob/master/pallets/template/expanded.rs



## 其他宏

Decl_runtime_apis & imp_runtime_apis , 定义runtime api:

Https://substrate.dev/recipes/3-entrees/runtime-api.html

https://substrate.dev/rustdocs/master/sp_api/macro.decl_runtime_apis.html

Https:://substrate.dev/rustdocs/master/sp_api/macro.impl_runtime_apis.html

Runtime_interface， 定义在runtime里可以调用host提供的函数

Https:://substrate.dev/rustdocs/v2.0.0-alpha.8/sp_runtime_inferface/attr.runtime_interface.html







# Runtime 数据存储的设计

## 内容

- 区块链存储的不同点和约束
- Substrate存储单元的类型
- 存储的初始化
- 最佳实践



## 区块链存储的不同点

区块链应用通常几个特点：

- 开源可审查，对等节点，引入延迟和随机来达到共识
- 链式，增量地存储数据

区块链应用的节点软件依赖高效的键值对数据库

- LevelDB
- RockDB

## 区块链存储的约束

区块链作为业务的载体， 存储相关的限制有：

- 大文件直接存储在链上的成本很高
- 链式的区块存储结构不利于对历史数据的索引
- 另一个约束是，在进行数值运算时不能使用浮点数



## substrate 存储单元的类型

开发链上存储单元的特点

- Rust原生数据类型的子集，定义在核心库和alloc库中
- 原生类型构成的映射类型
- 满足一定的编解码条件

单值类型 

简单映射

双键映射



单值类型

存储某种单一类型的值，如布尔，数值，枚举，结构体等

- 数值， u8, i8, u32, i32, u64, i64, u128, i128
- 大整数， u128, u256, u512
- 布尔， bool
- 集合， Vec < T >, BTreeMap, BTreeSet, 
- 定点小数,  Percent,  Permill, Perbill, 
- 定长哈希， H128, H256, H512
- 其他复杂类型， Option < T > ，tuple,  enum, struct 
- 内置自定义类型， Monment,  AccountId



```rust
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    // store unsigned integer, init to zero if not set 
    MyUnsignedNumber get(fn unsigned_number）: u8 = 10;
    // also init to zero, can store negative number 
    MysignedNumber get(fn signed_number) : i8;
  }
}
```



数值类型u8, i8, u32, i32, u64, i64, u128, i128的使用

- 增: MyUnsignedNumber::put(number);
- 查: MyUnsignedNumber::get();
- 改: MyUnsignedNumber::mutae(| v|  v + 1 );
- 删: MyUnsignedNumber::kill();

更多API， 请参考文档Triat frame_suppor::storage::StorageValue

数值类型u8, i8, u32, i32, u64, i64, u128, i128的安全操作

- 返回Result类型: checked_add, checked_sub, checked_mul, checked_div

  // fail to transaction if error

  My_unsigned_num.checked_add(10)?;

- 溢出返回饱和值： saturating_add, saturating_sub, saturating_mul

  // result is 255 for u8

  My_unsigned_num.saturating_add(10000);



大整数u256, u512类型定义：



```rust
use sp_core::U256;
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    // init to 0
    MyBigInteger get(fn my_big_integer): u256;
  }
}
```

 操作: checked_add, overflowing_mul .. 

更多API, 参考文档 sp_core::U256

Bool 类型定义

```rust
decl_storage! {
  triat Store for Module<T: Trait> as DataTypeModule {
    // init to false, store boolean value 
    MyBool get(fn my_bool): bool;
  }
}
```



Vec < T > 类型定义：

```rust
use sp_std::prelude::*;

decl_storage! {
  triat Store for Mudule<T: Trait> as DataTypeModule {
    // default to 0x00
    MyString get(fn my_string) : Vec<u8>;
  }
}
```



操作： Push, pop, iter ... 



Percent, Permill, Perbill 类型定义

```rust
use sr_primitives::Permill;
decl_storage! {
  trait Store for Mudule<T: Trait> as DataTypeModule {
    // fix point number, default to 0
    MyPermill get(fn my_permill) : Permill;
  }
}
```

Percent, Permill, Perbill类型操作：

- 构造
  - Permill::from_percent(value);
  - Permill::from_parts(value);
  - Permill::from_rational_approxmation(p, q);
- 计算
  - Permill_one.saturating_mul(permill_two);
  - My_permill * 20000 as u32

API 文档 sp_arithmetic::Permill



Moment 时间类型定义;

```rust
pub triat Trait : System::Trait + timestamp::Trait {}
decl_storage! {
  trait Store for Module<T: Triat> as DataTypeModule {
    // Moment is type alias of u64
    MyTime get(fn my_time) : T::Moment;
  }
}
```



获取链上时间 <timestamp::Module<T>>>::get();



AccountId 账户类型定义： 

```rust
decl_storage! {
  triat Store for Module<T: Triat> as DataTypeModule {
    MyAccountId get(fn my_account_id) : T::AccountId;
  }
}
```



获取: AccountId: let sender = ensure_signed(origin)?;

struct类型

```rust
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default)]
pub strcut People {
  name: Vec<u8>,
  age: u8,
}

decl_storage! {
  triat Store for Module<T: Triat> as DataTypeModule {
    MyAccountId get(fn my_struct) : People;
  }
}
```

enum 类型：

enum类似， 还需要实现Default接口

## 简单映射类型

map类型, 用来保存键值对，单值类型都可以用作key 或者value, 定义

```rust

```



# 第 5课【5-2：Rust 进阶】

# 第 5课【5-3：Rust 进阶】

# 第5课【5-4：Rust 进阶】

# 第6课【6-1：开发存证区块链】

# 第6课【6-2：开发存证区块链】

# 第6课【6-3：开发存证区块链】

# Substrate 建桥原理探究

# 第 7课【7-1：Polkadot-js-app/api 如何使用】

# 第7课【7-2：Polkadot-js-app/api 如何使用】

# Substrate Noe template 

## Polkadot Js Apps 

- Generalized and hosted UI
- Quickly test new functionality 
- Loaded with general tools like: 
  - Creating transactions
  - Read storage 
  - See events
  - and way more ...
- Great for development

## 模块定义概览

```rust
use support::{decl_module, decl_storage, decl_event, ...};

// triat Rust 中的关键字
pub trait Trait: system::Trait { ... }

decl_storage! { ... } // 存储
decl_event! { ... } // 事件
decl_error! { ... } // 错误
decl_module! { ... } // 用户可以调用的一些方法
 
impl <T: Trait> Mudule<T> { ... } // 用户不可以调用但是给template提供的一些方法
```

## 引入和定义关联类型

```rust
pub trait Trait: system::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// and it inherits from system::Trait: 
// From 'system' pallet 
pub triat Trait : 'static  + Eq + Clone {
  type Origin : ... // 关联类型
  type Call: ... // 关联类型
  type Index: ... // 关联类型
  type BlockNumber: ... // 关联类型
}
```

## 定义存储

```rust
decl_storage! {
  triat Store for Module<T: Trait> as TemplateModule { // TemplateModule 是目前模块
    // Here we are declaring a StorageValue, 'Something' as an Option<u32> 
    // 'get(fn something)' defines a getter function 
    // Getter called with 'Self::thing()'
    Something get(fn something) : Option<u32>;
    // Here we are declaring a StorageMap 'SomeMap' from an AccountId to a Hash. 
    // Getter called with 'Self::some_map(account_id)'
    SomeMap get(fn some_map) : map hasher(identity) T::AccountId => u32;
  }
}
```

## 定义可调用函数

```rust
decl_moudle! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    dn deposit_event<T>() = default; // The default deposit_event defintion 
    
    pub fn do_something(origin, something : u32) -> Result {
      let sender = ensure_signed(origin)?; // Check for transaction 
      <Something>::put(something); // Put a value into a StorageValue 
      Self::deposit_event(RawEvent::SomethingStored(something, who)); // Emit Event
      Ok(()) // Return Ok at the end of a function
    }
  }
}
```

## 定义公共和私有函数

```rust
impl <T: Trait> Module<T> {
  fn mint(to: T::AccountId, id: T::Hash) -> Result { ... }
  fn transfer(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Result {
    ...
  }
}
// 如果定义为'pub' , 其他模块也可以调用
```

## 定义事件

```rust
decl_event! {
  pub enum Event<T> where AccountId = <T as system::Triat>::AccountId {
    // Event 'SomethingStored' is declared with a parameter of the type 'u32' and 'AccountId' 
    SomethingStored(u32, AccountId),
  }
}
```

## Coding 

https:://github.com/SubstrateCourse/substrate-node-template

## 帮助链接

- Https:://www.substrate.io/
- Https:://www.substrate.io/tutorials
- Substrate Node Templete, Front-end Template
- Official: Substrate Github, Polkadot Github
- 官方中文, 知乎专栏， 视频教学bilibill
- Rust: Rust programming book 

## 如何参与开源项目

- Star & watch & fork 
- Check issues and PRs repo
- Join riot/discord channels
- Feel free to ask any questions,  stupid ones are always welcomed 
- Follow the instructions to contribute

