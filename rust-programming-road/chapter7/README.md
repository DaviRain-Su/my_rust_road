Q1: 非侵入式接口？侵入式接口？

Q2: 消息传递调用的格式？
A2: 使用点操作符来调用函数, 形如，recevier.message, 称为消息传递，点操作符左边的
recevier被称为接收者，右边的部分被称为消息。在面向对象中，消息也被叫做方法。

Q3: 关联类型和关联常量？
A3: 在trait中定义的类型和常量

Q4: Rust中的构造函数？
A4: Rust标准库中使用std::default模块中的一个叫做Default trait，来实现。

Q5: 对于ANSI控制字符来说， 前景色和背景色是由相应的代码决定的，和他们的拼接顺序无关系。
A5: 所以这里最终拼接结果是43;31，先判断背景色，然后是前景色，其实反过来, 31;43也不会
影响呈现结果。

Q6: 关于命令行字体颜色问题：
A6： echo "\e[30mHello\e[0m", 这是设置的字体颜色为黑色，所以30是字体颜色黑色
30-37字体颜色(黑色， 红色， 绿色，黄色，蓝色，粉色，靛蓝色， 白色)
40-47是设置背颜色的也是从黑色到白色
echo "\e[37;40mHello\e[0m"
echo "\e[40;37mHello\e[0m",所以这两个的颜色设置是一样的，都是字体颜色为白色，背景颜色为黑色。

Q7: Newtype模式的优点
A7：
1. 隐藏实际类型，限制功能。使用Newtype模式包装的类型并不能被外界访问，，除非提供相应的方法.
2. 明确语义。比如可以将f64类型包装为Miles(f64)和Kilometers(f64), 分别代表英里和千米，这样的语义提升是零成本的，没有多余的性能开销。
3. 使复制语义的类型具有移动语义， 比如本来f64是复制语义，而包装为Miles(f64)之后，因为
结构体本身不能被自动实现为Copy， 所以Miles(f64)就成了移动语义。


Q8: 不同编程范式的特点
A8: 无论采用面向对象还是函数式的开发思想，可以代码复用和高内聚低耦合的架构就是一种美。
1.对于面向对象范式的语言，其核心的概念就是继承、多态和封装，它将对象作为程序的基本构架单元。
2.对于函数式语言， 其核心是将函数作为程序的基本构建单元，采用抽象和复用等手段来组织和复用代码。

两者之间的优点和缺点：
1. 面向对象范式在代码结构化方面的优点在于更加符合直觉，缺点是性能差，过度封装，而基于类继承的方式也会造成强耦合。
2. 函数式范式的优点在于它的核心思想是”组合优于继承“， 与面向对象范式相比，其复用的粒度更小，更自由灵活，耦合度更低，但其缺点是学习成本比较高。


Q9: Rust对面向对像编程风格的支持总结几点
A9: 
1. 封装，Rust提供了结构体strcut和枚举体Enum来封装数据，并可使用pub关键字来定义其字段可见性；提供了impl关键字来实现数据的行为。
2. 多态， 通过trait和泛型以枚举Enum来允许程序操作不同类型的值。
3. 代码复用，通过泛型单态化，triat对象、宏Marco, 语法扩展procedural macro，代码生成code generation来设计模式。