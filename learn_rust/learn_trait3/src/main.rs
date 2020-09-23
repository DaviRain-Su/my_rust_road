// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut larger = list[0];
//     for &item in list {
//         if item > larger {
//             larger = item;
//         }
//     }
//     larger
// }
fn largest<T>(list: &[T]) -> T 
    where T : PartialOrd + Copy
{
    let mut larger = list[0];
    for &item in list {
        if item > larger {
            larger = item;
        }
    }
    larger
}
fn main() {
    let number_list = vec![1, 2, 4, 5,32, 67, 2, 4];
    // let max_number  = largest_i32(&number_list);
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);
    
    let number_char = vec!['a', 's', 'e', 's'];
    // let max_char = largest_char(&number_char);
    let max_char = largest(&number_char);
    println!("max_char = {}", max_char);
 
    println!("Hello, world!");
}