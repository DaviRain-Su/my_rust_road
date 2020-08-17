
/// # 自定义类型
///
/// Rust自定义数据类型主要通过：
/// - struct 定义一个结构体
/// - enum: 定义一个枚举类型
///
/// 常量可以通过const 和 static 关键字来创建
///
///
/// # 结构体
///
/// 结构体有三类，使用struct来创建
/// - 元组结构体，具名元组
/// - 经典C语言风格结构体
/// - 单元结构体，不带任何字段，在泛型中很有用
///
///
pub fn struct_type() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    impl <'a> Person<'a> {
        fn new(name: &'a str, age: u8) -> Self {
            Person {
                name,
                age,
            }
        }

        fn get_name(&self) -> &str {
            self.name
        }

        fn get_age(&self) -> u8 {
            self.age
        }

        fn print(&self) {
            println!("My name is {}, and age is {}", self.name, self.age);
        }
    }

    //uint struct
    struct Nil;

    #[derive(Debug, Copy, Clone)]
    struct  Point {
        x: f32,
        y: f32,
    }

    impl Point {
        fn new(x: f32, y: f32) -> Self {
            Point{
                x,
                y,
            }
        }

        fn get_x(&self) -> f32{
            self.x
        }

        fn get_y(&self) -> f32 {
            self.y
        }

        fn add_one(&mut self) {
            self.x += 1.0;
            self.y += 1.0;
        }

        fn print(&self) {
            println!("x =  {}, y =  {}", self.x, self.y);
        }


    }
    #[derive(Debug)]
    struct Pair(i32, i32);

    trait  Area {
        type Output;
        fn area(&self) ->Self::Output;
    }

    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl  Area for Rectangle {
        type Output = f32;
        fn area(&self) -> Self::Output {
            (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
        }
    }



    let name = String::from("Davirain");
    let age: u8 = 25;
    let person = Person::new(&name, age);
    let my_name = person.get_name();
    let my_age = person.get_age();
    println!("person is {:?}, name is {}, age is {}", person, my_name, my_age);
    person.print();

    let mut point = Point::new(2.3, 3.4);
    println!("point is {:?}", point);
    point.print();
    point.add_one();
    println!("point is {:?}", point);
    point.print();

    let new_point = Point{ x: 1.0, ..point };
    println!("new point is {:?}", new_point);
    println!("new point x is {}, y is {}", new_point.x, new_point.y);
    println!("new point x is {}, y is {}", new_point.get_x(), new_point.get_y());

    let Point { x: x1, y: y1} = new_point;
    let rectange = Rectangle{
        p1: Point{ x: x1, y: y1},
        p2: point,
    };

    println!("rectange area is {}", rectange.area());

    let _nil = Nil;

    let pair = Pair(2, 3);
    println!("The pair is {:?}", pair);

    let Pair(integer, decimal) = pair;
    println!("integer is {}, decimal is {}", integer, decimal);
}