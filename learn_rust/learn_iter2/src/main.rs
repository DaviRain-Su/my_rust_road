struct Counter {
    count: u32,
}
impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let mut c = Counter::new();

    while let Some(i) = c.next() {
        println!("i = {}", i);
    }
    // }else {
    //     println!("end");
    // }
    println!("Hello, world!");
}
