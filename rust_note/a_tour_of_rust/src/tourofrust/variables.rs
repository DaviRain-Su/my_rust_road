/// # 变量
/// 
/// 变量使用let关键字来声明
/// 在赋值时，Rust能够在99%的情况下推断变量类型。
/// 如果不能，可以将类型添加到变量声明中`let a : i32 = 9;`
/// 
/// 需要注意的是，如果多次分配相同的变量名的方式，称为变量遮蔽，
/// 可以改变变量类型以实现对改变量名的后续使用
/// 
/// 变量名总是遵循蛇形命名法snake_case
pub fn variable_example() {
    //rust推断出x的类型
    let x = 13;
    println!("x = {}", x);
    
    let x: u32 = 34;
    println!("x = {}", x);

    let x = 8u8;
    println!("x = {}", x);

    //rust显式声明类型
    let x : f64 = 3.14159;
    println!("x = {}", x);

    //rust也支持先声明后初始化
    let x ;
    x = 0;
    println!("x = {}", x);

    #[derive(Debug, Clone)]
    struct S(Vec<i32>);

    trait Math {
        type Output; 
        fn max(&self) -> Self::Output;
        fn min(&self) -> Self::Output;
    }

    impl S {
        fn new(vec: &Vec<i32>) -> Self {
            S(vec.clone())
        }

        fn print(&self) {
            println!("S = {:?}", self);
            println!("S.0 = {:?}", self.0);
        }
    }

    impl Math for S {
        type Output = i32;
        fn max(&self) -> i32 {
            if self.0.is_empty() { return 0; }
            let mut largest = self.0[0];
            for &item in self.0.iter() {
                if largest < item {
                    largest = item;
                }
            }
            largest
        }
        fn min(&self) -> i32 {
            if self.0.is_empty() { return 0; }
            let mut litter = self.0[0];
            for &item in self.0.iter() {
                if item < litter {
                    litter = item;
                }
            }
            litter
        }
    }

    let vec = vec![1, 3, 9, 4, 3];
    let s = S::new(&vec);
    s.print();
    println!("s max = {}", s.max());
    println!("s min = {}", s.min());
}