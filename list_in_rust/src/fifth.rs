pub struct List<T>{
    head: Link<T>,
    // tail: Link<T>,
    // tail: Option<&'a mut Node<T>>,
    tail: *mut Node<T>,
}
type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl < T> List< T>{
    pub fn new() -> Self{
        Self { head: None, tail: std::ptr::null_mut()}
    }

    // pub fn push(&mut self, elem: T) {
    //     let new_tail = Box::new(Node {
    //         elem,
    //         // when push elem next is aluway is none
    //         next: None,
    //     });

    //     // //swap the old_tail to point to the new tail
    //     // let old_tail = std::mem::replace(&mut self.tail, None);

    //     // match old_tail {
    //     //     Some(mut old_tail) => {
    //     //         // if the old tail existed, update it to point to the new tail
    //     //         old_tail.next = Some(new_tail);
    //     //     },
    //     //     None => {
    //     //         // otherwise, update the head to point to it
    //     //         self.head = Some(new_tail);
    //     //     }
    //     // }

    //     // put the box in the right place, and then grab a reference to its node
    //     let new_tail = match self.tail.take() {
    //         Some(old_tail) => {
    //             // if the old tail existed, update it to point to the new tail
    //             old_tail.next = Some(new_tail);
    //             old_tail.next.as_mut().map(|node| &mut **node)
    //         },
    //         None => {
    //             // otherwise, update the head to point to it
    //             self.head = Some(new_tail);
    //             self.head.as_mut().map(|node| &mut **node)
    //         }
    //     };

    //     self.tail = new_tail;
    // }

    // pub fn pop(&mut self) -> Option<T> {
    //     // grab the list's current head
    //     self.head.take().map(|head|{
    //         let head = *head;
    //         self.head = head.next;
            
    //         // if we're out of 'head', make sure to set tail to 'none'
    //         if self.head.is_none(){
    //             self.tail = None;
    //         }

    //         head.elem
    //     })
    // }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node{
            elem,
            next:None,
        });

        let raw_tail : *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }else {
            self.head = Some(new_tail)
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head|{
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter{ next: self.head.as_ref().map(|node|
            &** node
            )
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut{ next: self.head.as_mut().map(|node| 
            &mut **node)
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

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node|&**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.take().map(|node|{
            self.next = node.next.as_mut().map(|node| & mut **node);
            &mut node.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
#[cfg(test)]
mod test {

    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        //chech empty list behaves right
        assert_eq!(list.pop(), None);

        //populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        //push some more just to make sure nothing's corrputed
        list.push(4);
        list.push(5);

        // check normal removal 
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // chech exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2); 
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
    }

    #[test]
    fn iter() {
        let mut list  = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

    }
    #[test]
    fn iter_mut() {
        let mut list  = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }   
}