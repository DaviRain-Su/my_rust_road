/// 用两个栈实现队列
struct Queue<T> {
    stack1: Vec<T>,
    stack2: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    pub fn append_tail(&mut self, elem: T) {
        self.stack1.push(elem);
    }

    pub fn delete_head(&mut self) -> Option<T> {
        loop {
            if !self.stack1.is_empty() {
                let temp_val = self.stack1.pop().unwrap();
                self.stack2.push(temp_val);
            } else {
                break;
            }
        }
        self.stack2.pop()
    }

    // 一个队列为空，要求两个栈都必须是空的才行
    pub fn is_empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

#[test]
fn test_queue_empty() {
    let q: Queue<i32> = Queue::new();
    assert_eq!(q.is_empty(), true);
}

#[test]
fn test_queue_append_elem() {
    let mut q = Queue::new();
    q.append_tail(1);
    q.append_tail(2);
    while !q.is_empty() {
        let temp = q.delete_head().unwrap();
        println!("temp = {}", temp);
    }
    assert_eq!(q.is_empty(), true);
}

#[test]
fn test_queue_delete_elem() {
    let mut q = Queue::new();
    q.append_tail(1);
    q.append_tail(2);

    let temp = q.delete_head().unwrap();
    assert_eq!(temp, 1);
    let temp = q.delete_head().unwrap();
    assert_eq!(temp, 2);
}

#[test]
fn test_queue_delete_elem1() {
    let mut q = Queue::new();
    q.append_tail(1);
    q.append_tail(2);

    q.delete_head();
    assert_eq!(q.is_empty(), false);
}
