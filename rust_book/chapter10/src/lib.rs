pub mod generics_type {
/// # 找到一个数字列表中的最大值 v1
/// 
/// 
/// ```
/// fn find_list_nax() {
///     let number_list = vec![34, 50, 25, 100, 65];
///     
///     let mut largest = number_list[0];
///     
///     for number in number_list {
///         if largest < number {
///             largest = number;
///         }
///     }
///     println!("The largest number is {}", largest);
/// }
/// 
/// fn largest_list_max(list: &[i32]) -> i32 {
///     let mut largest = list[0];
///     
///     for &item in list.iter() {
///         if largest < item  {
///             largest = item;
///         }
///     }
///     largest
/// }
/// ```
pub fn find_list_max() {
    pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if largest < item {
                largest = item;
            }
        }
        largest
    }
    pub fn largest_list_i32_max(list : &[i32]) -> i32 {
        println!("The list size is --> {}", std::mem::size_of_val(list));
        let mut largest = list[0];

        for &item in list.iter() { // list.iter() 迭代每一个元素返回的都是每一个元素的引用
            //^所以这里使用&item 来模式匹配每一个元素拆解出来，因此 item 就是其中每个元素
            if largest < item  {
                largest = item;
            }
        }
        largest
    }
    pub fn largest_list_char_max(list : &[char]) -> char {
        println!("The list size is --> {}", std::mem::size_of_val(list));
        let mut largest = list[0];

        for &item in list.iter() { // list.iter() 迭代每一个元素返回的都是每一个元素的引用
            //^所以这里使用&item 来模式匹配每一个元素拆解出来，因此 item 就是其中每个元素
            if largest < item  {
                largest = item;
            }
        }
        largest
    }
    //find vec1 
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_list_i32_max(&number_list);
    let result = largest(&number_list);
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if largest < number {
    //         largest = number;
    //     }
    // }

    println!("The largest number is {}", result);

    // find vec2
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest_list_i32_max(&number_list);
    let result = largest(&number_list);
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if largest < number {
    //         largest = number;
    //     }
    // }

    println!("The largest number is {}", result);

    let char_list = vec!['u', 'i', 'a', 'o', 'e'];
    // let result = largest_list_char_max(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}


/// # 在结构体中使用泛型
/// 
/// 
pub fn generics_strcuct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    impl <T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let x = Point{ x: 1u8, y: 2u8};
    println!("x is {:?}", x);
    println!("x.x is {}", x.x());
    let m = 12u8;
    println!("the value is {}. the i32 is {}", std::mem::size_of_val(x.x()), std::mem::size_of_val(&m));
    let y = Point{x: 2.2, y: 3.3};
    println!("y is {:?}", y);
    #[derive(Debug)]
    struct PointM<T,U> {
        x: T, 
        y: U,
    }
    let z = PointM{x: 1.0, y: 13};
    println!("z is {:?}", z);
    let z = PointM{x: 3, y: 4};
    println!("z is {:?}", z);
}

}