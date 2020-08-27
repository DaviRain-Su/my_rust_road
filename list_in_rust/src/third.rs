use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {

    /// init list
    pub fn new() -> Self{
        Self{
            head : None,
        }
    }

    /// append value to list
    /// A -> B -> D
    /// append C
    /// C -> A -> B -> D
    pub fn append(&self, elem: T) -> List<T> {
        Self{
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }

    /// tail list
    /// example A -> B -> D
    /// tail
    /// B -> D
    pub fn tail(&self) -> List<T> {
        Self{
            head: self.head
                .as_ref()
                .and_then(|node| node.next.clone())
        }
    }

    /// get list head value by reference
    pub fn head(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter{
            next: self.head
                .as_ref()
                .map(|node|
                    &**node
                )
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}


impl <'a, T> Iterator for Iter<'a, T > {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node|{
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new()
            .append(1)
            .append(2)
            .append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}