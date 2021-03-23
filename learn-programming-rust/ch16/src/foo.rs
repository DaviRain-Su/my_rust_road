use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::borrow::BorrowMut;

#[derive(Debug)]
struct Foo<T: Clone + Copy> {
    bar: Vec<Rc<RefCell<T>>>,
}

impl<T: Clone + Copy> Foo<T> {
    pub fn new() -> Self {
        Self {
            bar: vec![],
        }
    }

    pub fn push(&mut self, value: T) {
        self.bar.push(Rc::new(RefCell::new(value)));
    }

    // pub fn get_bar(&mut self, index: usize) -> &T {
    //     self.bar[index].into_inner()
    // }
}

// #[test]
// fn test_foo() {
//     let mut v = Foo::new();
//     v.push(1);
//     let ret = v.get_bar(0);
//     assert_eq!(1, *ret);
// }