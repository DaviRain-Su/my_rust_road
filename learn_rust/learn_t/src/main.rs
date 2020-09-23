//1. 泛型是具体类型或者其他属性的抽象替代，用于减少代码重复
//2. 在函数定义中使用泛型
//3. 在结构体体中使用泛型
//4. 在枚举中使用泛型
//5. 在方法中使用泛型
//6 总结： 使用泛型并不会造成程序性能上的损失，rust通过在编译时进行的泛型代码的单态化

// ----------没有使用泛型--------------
// fn largest_i32(list: &[i32]) -> i32  {
//     let mut largest =  list[0];
//     for item in list {
//         if *item > largest {
//             largest = *item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> char  {
//     let mut largest =  list[0];
//     for item in list {
//         if *item > largest {
//             largest = *item;
//         }
//     }
//     largest
// }
// ---------------使用泛型--------------------
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut larger = list[0];
//     for &item in list {
//         if item > larger {
//             larger = item;
//         }
//     }
//     larger
// }
// fn main() {
//     let number_list = vec![1, 2, 4, 5,32, 67, 2, 4];
//     // let max_number  = largest_i32(&number_list);
//     let max_number = largest(&number_list);
//     println!("max_number = {}", max_number);
    
//     let number_char = vec!['a', 's', 'e', 's'];
//     // let max_char = largest_char(&number_char);
//     let max_char = largest(&number_char);
//     println!("max_char = {}", max_char);
 
//     println!("Hello, world!");
// }

// --------- 在结构体中使用泛型---------------
// -------------在方法中使用泛型--------------
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl <T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y, }
    }
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &T {
        &self.y
    }
}

#[derive(Debug)]
struct Point2 <T, U> {
    x: T, 
    y: U,
}
impl <T, U> Point2<T, U>{
    pub fn new(x: T, y: U) -> Self {
        Self {
            x,
            y,
        }
    }
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &U {
        &self.y
    }

    fn create_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y 
        }
    }
}

// ---------------在枚举中使用泛型-------
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// -------------------------------------


fn main() {
    let integer = Point::new(1, 2);
    println!("{:#?}",integer);
    println!("x = {}, y = {}", integer.get_x(), integer.get_y());
    
    let folat = Point::new(1.1, 1.2);
    println!("{:#?}", folat);
    println!("x = {}, y = {}", folat.get_x(), folat.get_y());

    let a = Point2::new(1, 3.3);
    println!("{:#?}", a);
    println!("x = {}, y = {}", a.get_x(), a.get_y());

    let b = Point2::new(1, 3);
    let c = Point2::new(2, '3');
    let d = b.create_point(c);
    println!("{:#?}", d);
}