/// # stack
///
/// 栈是一种数据结构，是一种只能在一端进行插入和删除操作的数据结构
///
/// 按照先进后出的原则存储数据，先进入的数据结构压入栈底，最后的数据在栈顶
/// 需要读数据的时候从栈顶开始弹出数据 LIFO
///

#[derive(Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode{
            val,
            next: None
        }
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack{ top: None }
    }

    pub fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}