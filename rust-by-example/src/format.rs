/// # 格式化输出
/// 
/// foramt! 将格式化文本写到字符串
/// print! 与format!类似， 但是将文本输出到控制台io::stdout
/// println! 与print! 类似， 但是输出结果追加一个换行符
/// eprint! 与format!类似， 但是将文本输出到标准错误io::stderr
/// eprintn! 与eprint!类似， 但是将输出结果追加一个换行符
/// 
/// 
/// 
/// 
#[test]
pub fn test_format() {

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "Alice");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    use std::fmt;

    impl fmt::Display  for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.0)
        }
    }

    println!("This struct '{:?}' won't print ... ", Structure(3));
    
    println!("This struct is '{}'...", Structure(45));
    let pi = 3.141592;

    println!("Pi is roughly {:.3}",pi);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor} name.",
        "slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 34;
    let peter = Person { name, age};

    println!("{:#?}", peter);
}

#[test]
fn test_define_struct() {
    use std::fmt;

    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})",self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minxmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minxmax);
    println!("Debug: {:?}", minxmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is  {big} and the small is {small}",
        small = small_range,
        big = big_range
    );
    

    let point = Point2D{ x: 3.3, y: 7.2};

    println!("Compare points: ");
    println!("Display: {}",point);
    println!("Debug: {:?}", point);

    
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let point = Complex { real : 3.3, imag: 7.2};

    println!("Compare complex: ");
    println!("Display: {}",point);
    println!("Debug: {:?}", point);

}