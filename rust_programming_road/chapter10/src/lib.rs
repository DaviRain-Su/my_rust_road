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

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
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