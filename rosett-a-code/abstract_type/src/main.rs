/// Rust doesn't have traditional object oriented concept such as classes,
/// instead it uses a concept called trait. Traits are similar to abstract classes in the sense
/// that they define an inferface a struct must confrom to.
/// A trait can be defined as such
///
/// ```
/// trait Shape {
///     fn area(self) -> i32;
/// }
///
/// ```
/// The trait can then be implemented on a struct.
///
/// ```
/// struct Square {
///     side_length: i32,
/// }
///
/// impl Shape for Square {
///   fn area(self) -> i32 {
///     self.side_length * self.side_length
///   }  
/// }
/// ```
///
/// Note, trait can also have a default implementation:
///
/// ```
/// trait Shape {
///     fn area(self) -> i32;
///
///     fn is_shape(self) -> bool {
///         true
///     }
/// }
///
/// ```
///
trait Shape {
    fn area(&self) -> i32;

    fn is_shape(&self) -> bool {
        true
    }
}

struct Square {
    side_length: i32,
}

impl Shape for Square {
    fn area(&self) -> i32 {
        self.side_length * self.side_length
    }
}

impl Square {
    fn new(side_length: i32) -> Self {
        Self { side_length }
    }
}

fn main() {
    let square = Square::new(12);
    println!("area = {}", square.area());
    println!("{}", square.is_shape());
}
