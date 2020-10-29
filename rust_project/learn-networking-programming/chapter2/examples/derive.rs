use std::fmt;

use std::fmt::Display;

#[derive(Debug, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> fmt::Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point::new(1, 2);
    //using Display
    println!("{}", p);
    //  using Debug
    println!("{:?}", p);
}
