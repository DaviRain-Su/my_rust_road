trait Max<T> {
    fn max(&self) -> T;
}

struct ThreeTuple<T> {
    first: T,
    second: T,
    third: T,
}

impl<T> ThreeTuple<T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Self {
            first,
            second,
            third,
        }
    }
}

impl<T: PartialOrd + Copy> Max<T> for ThreeTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second && self.first >= self.third {
            self.first
        } else if self.second >= self.first && self.second >= self.third {
            self.second
        } else {
            self.third
        }
    }
}

struct TwoTuple<T> {
    first: T,
    second: T,
}

impl<T> TwoTuple<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}
impl<T: PartialOrd + Copy> Max<T> for TwoTuple<T> {
    fn max(&self) -> T {
        if self.first >= self.second {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let two_tuple = TwoTuple::new(4, 2);
    let three_tuple = ThreeTuple::new(6, 5, 10);

    println!("{}", two_tuple.max());
    println!("{}", three_tuple.max());
}
