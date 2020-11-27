
fn main() {

}

struct  Queue<T> {
    s1: Vec<T>,
    s2: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    pub fn put(&mut self, elem: T) {
        self.s1.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        loop {
            if !self.s1.is_empty() {
                let temp = self.s1.pop().unwrap();
                self.s2.push(temp);
            } else {
                break;
            }
        }
        self.s2.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }

}

#[test]
fn test_queue() {

    let mut q = Queue::new();
    assert_eq!(q.is_empty(), true);

    q.put(1);
    q.put(2);

    assert_eq!(q.is_empty(), false);

    let temp = q.pop().unwrap();

    assert_eq!(1, temp);

    assert_eq!(q.is_empty(),  true);
}
