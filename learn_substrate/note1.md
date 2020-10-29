# 第二课 课程大纲

Substrate & Polkadot 简介

- 为什么学习Substrate 
- Polkadot特点和生态
- 搭建开发环境
    - 安装依赖
    - 使用IDE
- 启动单节点开发网络
    - 命令行参数
- 启动多节点本地测试网络
- 作业


作业： 
1. 
- Substrate的官网地址是什么？ https:://substrate.io 
- Substrate的recipes网址是什么？ https:://substrate.io/recipes 
- Substrate的rust docs的网址是什么？ https:://substrate.io/docs
- Substrate的tutorial的网址是什么？https:://substrate.io/tutorial 
- Substrate Seminar在每周几进行？ 每周二


2. github的基本操作，养成良好的开源社区开发习惯
star 和 Watch substrate和polkadot的repo, 并截屏

3.编译第一节课中的node-template并截图编译成功的界面

4. 请运行node-template节点并截图


# Polkadot介绍

Maggie Dong 

Softeware developer @ParityTech 

Bachground: 
- NTF 协议
- 开发科普
- 开发以太坊Dapp开发
- Substrate开发

## 互联网的发展看区块链未来 -- 跨链

Web1.0 - 只读互联网

社交网络和在线视频出现之前，互联网就已经存在了。Web1.0甚至在20世纪90年代谷歌出现之前就已经存在。当时
的互联网是由AltaVista和网景公司主导的。网页是“只读”， 用户只能搜索信息，浏览信息。 

Web2.0 -- 可读写互联网

用户不仅仅局限于浏览，他们还可以自己创建内容并上传到网页上。在这个时代，诞生了问答式的、交互式的门户、诸如博客，
Wiki百科，最明显的变化时信息变得不再从官方到个人了，而是拓展出来的交互式的内容产生。但同时也产生了信息过载的问题。 

Web3.0  -- 价值互联网

Web3.0将创造一个新的全球数字经济，创造新的商业模式和市场。与谷歌和Facebook一起破坏平台垄断，并产生大量自下
而上的创新。打破数据隔离，让真正有价值的东西流通起来。 


## 区块链发展现状

### 公链

开发复杂
乌托邦
生态丰富
维护成本高
升级困难
开放
犯错成本高 

Defi
DAO
Staking 
Economics
ZK
Oracle 
Consensus
Crypto
TEE
跨链


### 联盟链

CA准入
开发简单
开发框架单一
功能单一
升级困难
可扩展性低 
升级困难
生态单调

业务


## 波卡是什么 

波卡要把不同的quk链连接到一起
愿景： 一个跨链协作的未来

## 为什么跨链是未来

- 区块链可以解决信任问题
- 专业化可以解决可扩展问题
- 可交互可以在可扩展的前提下，解决更广泛的信任问题，是一个更真实的世界

波卡在解决下面三个问题：
1. 跨链协作
2. 可扩展性
3. 共享安全


## 传统区块链根据场景作不同的取舍

Scalability -- IPFS --- Decenteralization

Decenteralization --- bitcoin/ethereum -- Security 

Scalability --- EOS -- Security 

## 波卡尝试打通整个区块链生态沟通协作

## 也因为能沟通协作，波卡也允许无上限扩展

## 拓扑结构

中介链 Relay Chain 

平行链 Parachain 

区块整理节点 Collators 

区块验证节点 Validators 

桥 Brige 

可交互的联盟链 -- 数据共享 全局信任

政务链
银行链
供应链
人才链
资产管理链
征信链


## 联盟链式趋势，公链是未来

联盟链 -- Substrate -- 公链 



# Substrate介绍

## Substrate -- 从Polkadot提取的开发敬仰

Polkadot  ---- > Substrate通用区块链开发框架

Substrate 是构造区块链的框架 

## 区块链包含哪些组件

- 数据库
- 点对点网络
- 共识算法
- 交易处理
- 状态转换函数Runtime
- 其他特别的函数： 零知识证明ZK, 分片


Substrate的定制化自由度高 

核心组件：  ---> 区块链的基础核心组件 
- 数据库
- 加密算法
- 点对点网络
- 序列化
- 零知识
- 共识算法
- 交易队列

链逻辑升级 ---> 治理、升级模型 
链上治理

Dapp --> 智能合约

## Substrate 特点

- 可扩展性
- 模块化
- 开源
- 自主可控

## Substrate 具体包含什么？

- 核心模块
    - 数据库
    - 交易队列
    - 命令行界面
    - 公密钥生产
    - RPC 

基本逻辑
    - 数据库
    - 结算
    - 时间戳

- P2P网络
    - 网络节点管理
    - 私讯协议集成 
    - 分布式哈希表

- 共识机制

    - 抵押
    - Bade 
    - Grandpa 
    - 区块落实追踪

- 链上治理
    - 民主
    - 投票
    - 议会
    - 国库


## Polkadot: Substrate之上建立的模块

- 平行链 Parachins 
- 区块链整理 Collators 
- 跨链通讯协定 Cross Chain Message Passing 
- 私讯协议 Gossip Protocol 
- 持续可用存储 Persistent Availability Store 
- 平行线程 Parathreads 
- 众筹模块 Crowd Funding 
- 赔偿 Claims 
- 拍卖 Auctions
- 公证 Register 

## The Substrate Runtime 

Sustrate Runtims Module Library SRML 

资产
共识
余额 
collective 
合约
治理
选举
grandpa
账户
块确认
indices 
会员
offences 
session 
抵押
超级权限
system 
时间戳
国库
and more ... 

Runtime是区块链的链上运行逻辑的集合 也就是状态转换函数 


## 一键链上升级 --- 永不分叉 (不分叉的链上升级)

Native client environment --- ENTRY APIs --- > Native runtime 和链上的runtime版本是否一致？ 

---> Native runtime (from client ) (OK)

--> Wasm runtime (from chain) WebAssemblu interpreter (Error)
 

 ---> Merklised Storage database 

## Runtime 升级的治理Governing Runtime Upgrades 

- Runtime 代码可以通过链上治理访问
- Sudo 模块
- Democracy 模块
- 自定义的模块和逻辑
- Runtime升级是可选的


## 为什么需要链上升级

- 修复重要的安全漏洞
- 改变核心规则
- 添加新功能
- 修复链上状态
- 硬分叉需要的协作成本极高，且易升级失败
- 没有明确的治理策略和升级时间点
Note : 使用WASM， 升级过程无需节点直接参与，如果不使用Wasm, 整个网络都需要执行升级操作。 

## Substrate 与企业系统无缝集成 
使区块链成为解决方案的一部分

- 使用内置的链下工作机， 提供与SAP, Oracle, SQL 服务器以及更多其他企业系统的无缝集成支持
- 支持集成Trusted Execution Environments (TEEs)
- 内置与区块链语言机的双向集成
- 使用JSON-RPC代理的中间件集成

## Substrate 是公链技术、生态和联盟链之间的桥梁 通过 Substrate 分享先进的区块链技术成果 

## 全球已有超过50个团队基于Substrate开发区块链

## Building a Hub for Developers 

https://substrate.io 

Tutorials 
videos 
recipes 
docs 
playground 
samples 
community 

## Substrate中文资料 substrate.io 

Bilibili  https://space.bilibili.com/67358318

知乎： https://zhuanlan.zhihu.com/substrate
        https://zhuanlan.zhihu.com/v2web3

微信公众号 
polkadot中文平台
Polkaworld
Polkabase 

## 如何运行Substrate节点

- 单节点（开发链dev) 

./target/release/substrate --dev 
./target/release/substrate purge-chain --dev // purget-chain 删除存储目录

- 多节点（本地链local) // 启动多节点的网络

./target/release/substrate --alice --chain local --base-path /tmp/alice // --chain指定链 指定用户alice 
./target/release/substrate --bob --chain local --base-path /temp/bob // --base-path 指定数据库的位置


# Substrate Node Template 

最小可用substrate单元


##  Polkadot JS Apps 

- Generalized and hosted UI
- Quickly test new functionality 
- Local with general tools like:
    - Creating transactions
    - Read storage
    - See events 
    - and Way more ... 
- Great for development 

## 模块定义概览

```rust 
use support::{decl_module, decl_storage, decl_event, ....};
pub triat Trait: system::Trait { ... }

decl_storage! { ...} // 存储
decl_event! { ... } // 事件 
decl_error! { ... } // 错误
decl_module! { ... } // runtime
impl <T: Trait > Module<T>  { ... }
```

## 引入和定义关联类型

```rust
pub trait Trait : system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// .... and it inherits from system::Trait:
// From 'system' pallet 
pub triat Trait : 'static + Eq + Clone {
    type Origin: ... 
    type Call : ... 
    type Index: ... 
    type BlockNumber: ... 
}

```

## 定义存储

```rust 
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        // Here we are declaring a StoreageValue 'Something' as an Option<u32> 
        // 'get(fn something)' defines a getter function 
        // Getrer called with 'Self::thing()'
        Something get(fn something) : Option<u32>;
        // Here we are declaring a StoreageMap 'SomeMap' from an AccountId to 
        // a Hash 
        // Getter called with 'Self::some_map(account_id)' 
        SomeMap get(fn some_map): map hasher(identity) T:：AccountId => u32;
    }
}
```

## 定义可调用函数

```rust 
decl_module! {

    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default; // The default deposit_event definition 

        pub fn do_something(origin, something: i32) -> Result {
            let sender = ensure_signed(origin)?;// Check for trasaction 
            Something::put(something); // Put a value into a StoreageValue
            Self::deposit_event(RawEvent::SomethingStored(something, who)); //Emit Event 
            Ok(()) // Return Ok at the end of a function
        }
    }
}
```

## 定义事件


```rust 
decl_event!{
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        //Event 'SomethingStored' is declared with a parameter of the type 'u32' 
        // and 'AccountId' 
        SomethingStored(u32, AccountId),
    }
};
```

## 定义公共和私有函数

```rust 
impl<T: Trait> Module<T> {
    fn mint(to: T::AccountId, id: T::Hash) -> Result {... }
    fn transfer(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Result { 
        ...
    }
}
// 如果定义为pub , 其他模块则可用
```
## Coding 
https://github.com/SubstrateCourse/substrate-node-template 






