use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl <T> Node<T>{
    fn new(elem: T) -> Rc<RefCell<Self<>>> {
        Rc::new(RefCell::new(Node{
            elem,
            prev : None,
            next : None,
        }))
    }
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T){
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match  self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            },
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head  = Some(new_head);
                },
                None => {
                    self.tail.take();
                },
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        }
        )
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail|{
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|
            Ref::map(node.borrow(), |node| &node.elem)
        )
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}


impl <T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() { }
    }
}
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek_front().is_none(), true);
        assert_eq!(list.peek_back().is_none(), true);
        assert_eq!(list.peek_front_mut().is_none(), true);
        assert_eq!(list.peek_back_mut().is_none(), true);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}