/// # 泛型、trait与生命周期
/// 
/// 所有的编程语言都会致力于高效地处理重复概念，在Rust中泛型是这样的一个工具。
/// 
/// 泛型的作用：
/// 泛型是具体类型或其他属性的抽象替代。在编写代码时，我们可以直接描述泛型的行为，或者
/// 他与其他泛型产生联系，而无须知晓它在编译和运行时采用的具体类型。
/// 
/// 对于函数可以使用参数中未知的具体值来执行相同的代码，类似的函数也可以使用泛型参数而不是
/// 具体地i32，String这样具体地类型。
/// 
/// 
/// - 使用trait来定义通用行为，在定义泛型时，使用trait可以将其限制为拥有某些特性行为
/// 的类型，而不是任意类型。
/// - 生命周期，这类泛型可以像编译器提供引用之间的相互关系，它允许我们在借用值时通过
/// 编译器来确保这些引用的有效性。
/// 
/// 
/// ## 通过代码提取为函数来减少重复工作
/// 
///  
pub mod example {
    #[test]
    fn find_a_max_value_in_number_list() { 
        // 在一个数字列表中找到最大值
        // 
        let number_list = vec![34, 50, 100, 65];

        let mut largest = number_list[0];

        for &number in number_list.iter() {
            if largest < number {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        //-----------------------------------------------
        //---------------比较第二个列表--------------------
        
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = number_list[0];

        for &number in number_list.iter() {
            if largest < number {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    // define a functon to find a max value

    pub fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
        
        for &item in list.iter() {
            if largest < item {
                largest = item;
            }
        }
        largest
    }
    #[test]
    fn test_largest_function() {
        // 提取在两个列表中搜索最大值的代码

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
        // --------------------------------------------------------

        let  number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }
}

/// # 泛型数据类型
/// 
/// 可以在那声明函数签名或结构体等元素时使用泛型，并在随后搭配不同的具体类型来使用这些元素。
/// 
/// 定义泛型函数， 结构体，枚举，方法
/// 
/// 泛型对代码性能产生的影响
/// 
/// # 在函数定义中
/// 
/// 当使用泛型来定义一个函数时，我们需要将泛型放置在函数签名中通常用于指定参数和返回值类型的地方。
/// 好处：
/// 以这种方式编写的代码更加灵活，并可以在不引入重复代码的同时向函数调用者提供更多的功能。
///  
/// 当我们需要在函数体重使用参数时，我们必须要在签名中声明对应的参数名称， 以便编译器知道
/// 这个名称的含义，类似的，当我们需要在函数签名中使用类型参数时，也需要在使用前声明这个类型
/// 参数的名称。
/// 
/// 类型名称的声明必须被放置在函数名与参数列表之间的一对尖括号<>中。
/// 
/// `fn largest<T>(list: &[T]) -> T`
/// 
/// 函数largest拥有泛型参数T, 它接受一个名为list的T值切片作为参数，并返回一个同样拥有类型T的值作为结果。
///
/// 
/// # 在结构体定义中
/// 
/// 使用<> 语法来定义一个或多个字段中使用泛型的结构体
///  
/// 在结构体名后的一对尖括号中声明泛型参数后，我们就可以在结构体定义中
/// 那些通常用于指定具体数据类型的位置使用泛型了。
/// 
/// 注意，在定义Point<T>时仅使用了一个泛型，这个定义表明Point<T>结构体对某个类型T
/// 是通用的。而无论具体地类型是什么，字段x与y都同时属于这个类型。
/// 
/// 为了保证泛型状态的前提下，让Point结构体中的x和y能够被实例化为不同的类型，
/// 我们可以使用不同多个泛型参数。
/// 
/// ## 在枚举定义中
/// 
/// 枚举定义也可以在他们的变体中存放泛型函数
/// 
/// ```
/// enum Option<T> { // Option<T>用来表示一个值可能存在的抽象性
///     Some(T), // 持有T类型值的Some变体
///     None, // 持有任何值的None变体
/// }
/// ```
/// 
/// 枚举同样可以使用多个泛型参数
/// 
/// ```
/// enum Result <T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```
/// 
/// Result 枚举拥有两个泛型： T 和 E。 同样拥有两个变体:持有了T类型值的OK， 以及一个持有E类型值的Err
/// 这个定义使得Result枚举可以很方便地被用在操作可能成功（返回某个T类型的值）， 也可能失败（返回某个E类型的错误）的场景. 
/// 
/// 
/// ## 在方法定义中
/// 
/// 方法也可以在自己的定义中使用泛型。
/// 
/// 
/// 结构体定义中的泛型参数并不总是与我们在方法签名上使用的类型参数一致。
/// 
/// ## 泛型代码的性能问题
/// 
/// Rust 实现泛型的方式决定了使用泛型的代码与具体类型的代码相比不会有任何速度上的差异
/// 
/// 如何实现的呢？
/// 
/// Rust 会在编译时执行泛型代码的单态化，单态化是一个在编译期将泛型代码转换为特定代码的
/// 过程，它会将所有使用过的具体类型填入泛型参数从而得到有具体类型的代码。
/// 
/// 他会寻找所有泛型代码被调用过的地方，并基于该泛型代码所使用的具体类型生成代码。
/// 
/// ```
/// let integer = Some(5);
/// let float = Some(5.0);
/// 
/// 
/// // 执行单态化之后是
/// 
/// enum Option_i32 {
///     Some(i32),
///     None,
/// }
/// 
/// enum Option_f64 {
///     Some(f64),
///     None,
/// }
/// 
/// let integer = Option_i32::Some(5);
/// let folat = Option_f64::Some(5.0);
/// 
/// ```
mod template {
    #[test]
    fn template_function(){
        // 两个在名称和签名上有所区别的函数
        fn largest_i32(list: &[i32]) -> i32 {
            let mut largest = list[0];
            
            for &item in list.iter() {
                if largest < item {
                    largest = item;
                }
            }
            largest
        }

        fn largest_char(list: &[char]) -> char {
            let mut largest = list[0];
            
            for &item in list.iter() {
                if largest < item {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    
        fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest  = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        
        fn largest_1<T>(list: &[T]) -> &T
            where T: PartialOrd
        {
            let mut largest = &list[0];
            let mut index = 0usize;
            
            for (temp_index, item) in list.iter().enumerate() {
                if largest < item {
                    largest = item;
                    index = temp_index; 
                }
            }

            &list[index]
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_1(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_1(&char_list);
        println!("The largest char is {}", result);
    }
    #[test]
    fn template_strut(){
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }
        
        // 必须紧跟着impl 关键字声明T， 以便能够在实现方法是指定Point<T> 。
        // 通过在impl 之后将T声明为泛型，Rust能够识别出Point尖括号内的类型是泛型而不是具体类型。
        impl <T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }


        // 这段代码意味着， 类型Point<f32> 将会拥有一个distance_from_origin的方法，而
        // 其他实例则没有该方法的定义。
        // 方法本身被用于计当前点于原点坐标的距离，使用了只能被用于浮点数类型的数据操作。
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt() 
            }
        }

        let integer = Point{ x: 5, y: 10};
        let float = Point{x: 1.0, y: 4.0};
        println!("integer = {:?}, float = {:?}", integer, float);
        println!("integer.x = {}", integer.x());
        println!("float.x = {}", float.x());

        println!("float distance origin is {}", float.distance_from_origin());
        // let wont_work = Point{x: 6, y: 9.0};//error, y, x必须为同样类型的数据
        
        // 使用了两个泛型的Point2D<T, U>, x和y可以拥有不同的类型
        #[derive(Debug)]
        struct Point2D<T, U> {
            x: T,
            y: U,
        }

        // 结构体定义中的泛型参数并不总是与我们在方法签名上使用的类型参数一致。
        // 在某些情况下，可能会有一部分泛型参数声明与impl关键字之后，而另一部分
        // 则声明于方法定义中。在这里，泛型函数T与U被声明在impl之后，因为他们是结构
        // 定义的一部分。而泛型V与W则被定义在fn mixup 中，因为他们仅仅与方法本省相关。
        //
        //
        impl <T, U> Point2D<T, U> {
            fn mixup<V, W>(self, other: Point2D<V, W>) -> Point2D<T, W> {
                Point2D {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let both_integer = Point2D{ x: 5, y: 10};
        let both_float = Point2D {x: 1.0, y: 4.0};
        let integer_and_float = Point2D{ x: 5, y: 4.0};

        println!("both integer = {:?}", both_integer);
        println!("both float = {:?}", both_float);
        println!("ingeger and float = {:?}", integer_and_float);

        let p1 = Point2D {x: 5, y: 10.4}; 
        let p2 = Point2D {x: "Hello", y: 'c' };


        // use mixup
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}


/// # triat 定义共享行为
/// 
/// trait 被用来向Rust编译器描述某些特定类型拥有的且能够被其他类型共享的功能，他使我们考科一以一种抽象的方式
/// 来定义共享行为。
/// 
/// 还可以使用trait约束来将泛型参数指定为实现了某些特定行为的类型。
/// 
/// > trait与其他语言中场被称为接口的功能类似，但是也不尽相同。
/// 
/// ## 定义trait
/// 
/// 类型的行为由该类型本身可供调用的方法组成，当我们可以在不同的类型上调用相同的方法时，
/// 我们就称这些类型共享了相同的行为。trait提供了一种将特定方法签名组合起来的途径，它定义了为达成某种目的
/// 所必须的行为集合。
/// 
/// ## 为类型实现trait
/// 
/// 为类型实现trait与实现普通方法步骤十分类似。它们的区别在于我们必须在impl关键字
/// 后提供我们想要实现的trait名，并紧跟for关键字及当前的类型名。在impl代码块中，我们
/// 同样需要填入triat中方法签名。但在每个签名的结尾不再使用分号，而是使用花括号并在其中编写
/// 函数体来为这个特定类型实现该trait的方法所应具有的行为。
/// 
/// 注意，实现triat有一个限制： 只有当triat或类型定义于我们的库中时我们才能为该类型
/// 实现对应的trait。 
/// 
/// 我们不能为外部类型实现外部trait， 这个限制被称为孤儿原则。 
/// 之所以这么命名是因为它的父类型没有定义在当前库中。这一规则也是程序一致性的组成部分，
/// 他确保了其他人编写的内容不会破坏到你的代码，反之亦然，如果没有这条规则，那么两个库可以分别对相同
/// 的类型实现相同的trait，Rust将无法确定应该使用哪一个版本。
/// 
/// ## trait的默认实现
/// 
/// 为trait宏的某些或所有方法都提供默认行为非常有用，它使我们无须为每一个类型
/// 的实现都提供自定义行为、当我们在为某个特定类型实现trait时，可以选择保留或
/// 重载每个方法的默认行为。
/// 
/// 
/// 还可以在默认实现中调用相同trait中的其他方法，哪怕这些方法没有默认实现，
/// 基于这一规则，triat可以在只需要实现一小部分方法的前提下，提供许多有用的功能。
/// 
/// 注意， 我们无法再重载方法实现爱内的过程中调用该方法的默认实现。
/// 
/// 
/// ## 使用trait作为参数
/// 
/// 如何使用trait来接受不同类型的数据
/// 
/// ```
/// pub fn notify(item: impl Summary) {
///     println!("Breaking news! {}", item.summarize());
/// }
/// ```
/// 
/// trait 约束
/// 
/// 这里impl trait常被用在一些较短的示例中，但他实际**只是trait 约束的语法糖**
/// 
/// ```
/// pub fn notify<T: Summary>(item : T) {
///     println!("Breaking new! {}", item.summarize());
/// }
/// ```
/// 
/// 两种不同形式的取舍：
/// 
/// impl trait 适用于短小的示例， 而trait约束则适合用于复杂情形
/// 
/// 例如将设我们需要接受两个都实现了Summary 的参数，那么使用impl trait 的写法：
/// ```
/// pub fn notify(item1: impl Summary, item2: impl Summary) ;
/// ```
/// 
/// item1, item2可以使用不同类型，同时实现了Summary，这里是没有问题。
/// 但是如果要是两个参数都使用相同的类型， 这是只能使用triat约束了
/// ```
/// pub fn notify<T: Summary> (item1: T, item2: T); 
/// ```
/// 泛型T指定了参数item1与item2的类型，它同时也决定了函数item1， item接受的参数
/// 值必须拥有相同的类型。
/// 
/// ## 通过+语法来指定多个trait约束
/// 
/// ```
/// pub fn notify(tiem : impl Summary + Display);
/// 
/// pub fn norify<T: Summay + Display>(item : T);
/// ```
/// 
/// ## 使用where 从句来简化trait约束
/// 
/// ```
/// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
/// fn some_function<T, U>(t: T, u: U) -> i32
///     where T: Display + Clone,  U: Clne + Debug
/// {}
/// ```
/// 
/// ## 返回实现了triat类型
/// 
/// 在返回值中使用impl trai语法
/// 
/// 为什么需要这样的功能？
/// 
/// 闭包，与迭代器，这些功能会创建出只有编译器才知道的或签名长到难以想象的类型。
/// impl trait 使你可以精炼地声明函数会返回实现了Iterator trait 的类型， 而不需要
/// 写出具体地类型。 
/// 
/// 但是问题是，只能在返回一个类型是使用impl trait.  
/// 
/// 
/// ## 使用trait约束有条件地实现方法
/// 
/// 
/// 同样我们可以为实现了某个trait的类型有条件地实现另一个trait， 对满足trait约束的所有类型
/// 实现trait也被称为**覆盖实现(Blanket implementation)**
/// 
/// 例如标准库中的对所有满足Display trait约束的类型实现了ToString triat 
/// ```
/// impl <T: Display> ToString for T {
///     // ....
/// }
/// ```
/// 
/// 由于标准库中提供了上面的覆盖实现，所以我们可以为任何实现了Display trait的类型调用
/// ToString trait的to_string 方法。
/// 
/// 
/// 借助triat和triat约束，我们可以在使用泛型参数来消除重复代码的同时，向编译器指明自己希望拥有
/// 的功能。而编译器则可以利用这些trait约束信息来确保代码中使用的具体类型提供了正确的行为。
/// 在动态语言中尝试调用一个类型没有实现的方法会导致在运行时出现错误。但是在rust中，将这些错误
/// 出现的时期转移到了编译期，并迫使我们在运行代码之前修复问题，我们无须编写那些用于运行时检查
/// 行为的代码，因为这些工作已经在编译期完成了这一机制保留泛型灵活性的同时提升了代码的性能。
/// 
mod trait_test {
    #[test]
    fn basic() {
        // 任何想要实现这个trait的类型都需要为上述方法提供自定义行为。
        // 编译器会确保每一个实现了Summary trait 的类型都定义了与这个签名完全一致的
        // summarize方法
        // 一个triat可以包含多个方法： 每个方法签名占据单独一行并以分号结尾。
        pub trait Summary{
            fn summarize_author(&self) -> String;
            fn summarize(&self) -> String {
                format!("(Read more from {} ...)", self.summarize_author())
            }
        }

        // 使用了impl trait, 这一参数可以接受任何实现了指定triat的类型
        pub fn notify(item : impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }

        pub fn return_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebook"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
        // error impl 不支持这样的写法
        // pub fn return_summarizable_2(switch : bool) -> impl Summary {
        //     if switch {
        //         NewsArticle {
        //             headline: String::from("Penguins win the Stanley Cup Championship!"),
        //             location: String::from("Pittsburgh, PA, USA"),
        //             author: String::from("Iceburgh"),
        //             content: String::from("The Pittsburgh Penguins once again are the best
        //             hockey team in the NHL"),
        //         } 
        //     }else {
        //         Tweet {
        //             username: String::from("horse_ebook"),
        //             content: String::from("of course, as you probably already know, people"),
        //             reply: false,
        //             retweet: false,
        //         } 
        //     }
        // }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }


        impl Summary for NewsArticle {
            fn summarize_author(&self) -> String {
                format!("@{}", self.author )
            }
            // fn summarize(&self) -> String {
                // format!("{}, by {} ({})", self.headline, self.author, self.location)
            // }
        }

        pub struct Tweet {
            pub username: String, 
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl  Summary for Tweet {
            fn summarize_author(&self) -> String {
               format!("@{}", self.username)
            }
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebook"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL"),
        };

        println!("New atricle available ! {}", article.summarize());

        notify(article);
        notify(tweet);
    }

    #[test]
    fn advance() {
        use std::fmt::Display;

        struct Pair<T> {
            x: T,
            y: T,
        }

        impl <T> Pair<T> {
            pub fn new(x: T, y: T) -> Self {
                Self {
                    x, y,
                }
            }
        }

        impl <T: Display + PartialOrd> Pair<T> {
            pub fn cmd_display(&self) {
                if self.x > self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }

        let pair = Pair::new(1, 2);
        pair.cmd_display();
    }
} 