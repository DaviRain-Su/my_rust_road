pub struct List {
   head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self{
        Self {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
            //将Link::Empty转移到dest也就是self.head的地方，然后这个函数返回原来的值,也就是原来的
            // self.head的值

            // next: self.head, // cannot move out of `self.head` which is behind a mutable reference
            // 这个是因为head中存放的是Box<node>的话，由于Box的所有权的关系，next: self.head将会将self中的head移动给next
            //这是self就会变成为初始化的部分，但是在push函数中我们使用的&mut 这是独占引用不能发生所有权的转移
            // 这里违反了借用规则，所以报错。将self中的值移动走，self中就没有东西了。这是的可变借用就会指向一个没有任何东西的地方。
            //这里需要寻找一种方法避免可变借用检测到这个变化
            // mem::replace
            //Moves src into the referenced dest, returning the previous dest value.
        });

        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match 会发生移动语义的转移
        match std::mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) =>{
                self.head = node.next; // 更改Self.head = Link::Empty的值为node.next，达到了删除一个节点的目的

                Some(node.elem) //返回删除的节点中的elem
            }
        }
    }


}

impl  Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link{
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}
#[cfg(test)]
mod test{

    use super::List;

    #[test]
    fn basic(){
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

}
