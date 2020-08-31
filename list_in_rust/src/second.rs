/// 对外提供暴露出去的
pub struct List<T> {
    head: Link<T>,
}

/// 可以类比为C中的指针，不过这里的这个指针是Option
/// 当指针存在时，中间存放数据的是Box做的管理
type Link<T> = Option<Box<Node<T>>>;

///定义的链表的节点
struct Node<T> {
    // elem是链表中存放的数据
    elem: T,
    next: Link<T>,
}

impl<T> List<T>{

    /// 静态构造函数
    pub fn new() -> Self{
        Self{ head: None }
    }

    /// push 一个元素
    pub fn push(&mut self, elem: T){
        // 构造新的节点
        let new_node = Box::new(Node{
            elem,
            // 这里使用take方法返回的是self.head东西，并将self.head设置为了None
            // 这里这样做也是因为移动语义，所有权原因。
            next: self.head.take(),
        });

        // 因此这里需要给self.head重新复制，移动语义
        self.head = Some(new_node);
    }

    /// pop出节点中的值，如果为空的list返回的就是None
    /// 如果不为空返回T
    pub fn pop(&mut self) -> Option<T> {
        // map的方法默认是获去调用者的值，也就是所有权
        // pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U>
        // 因此这里需要用到take方法
        self.head.take().map(|node|{
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    /// 返回list的head的节点中值的引用（不可变引用)
    ///
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref()
            .map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut()
            .map(|node|{
                &mut node.elem
            })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter{
            next: self.head.as_ref().map(|node| & **node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut{
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


pub struct IntoIter<T>(List<T>);

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct  Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        list.peek_mut().map(|value|{
            *value = 42;
        });

        assert_eq!(list.peek_mut(), Some(&mut 42));
        assert_eq!(list.peek(), Some(& 42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);


        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);


        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

}

