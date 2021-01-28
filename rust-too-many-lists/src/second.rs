/// # 一个可用的单链栈
///
/// an ok singly-linked stack
///
/// 前面的章节中，我们写了一个最小可行的单链栈。然后，有一些设计决定了它有点糟糕。
/// 让我们把它变得不那么糟糕吧。为此，将要
/// - 去发明轮子
/// - 使我们的列表能够处理任何元素类型
/// - 减伤 peeking
/// - 使我们的列表可迭代
///
/// 在这个过程中，我们将了解：
/// - 高级Option的使用
/// - Generics 泛型
/// - Lifetimes 生命周期
/// - iterators 迭代器
///
/// ## 使用 Option
///
/// 特别细心的读者可能已经注意到，我们实际上重新发明了一个非常糟糕的版本：
///
/// enum Liknk {
///     Empty,
///     More(Box<Node>),
/// }
///
/// Link只是Option<Box<Node>>, 现在,不用到处写Option<Box<Node>>很好，而且不想pop，
/// 我们没有向外界暴露Option, 所以可能没有什么问题。然而， Option有一些非常好的方法，我们一直
/// 在手动实现自己的。让我们不要这样做，用Option替换所有的东西，首先，我们可以自然的把所有的东西
/// 都重命名为Some和None：
///
///
/// pub struct List {
///     head: Link,
/// }
///
/// type Link = Option<Box<Node>>;
///
/// struct Node {
///     elem: i32,
///     next: Link,
/// }
///
///
///
/// impl List {
///     pub fn new() -> Self {
///         List { head: None }
///     }
///
/// //     pub fn push(&mut self, elem: i32) {
/// //        let new_node = Box::new( Node {
/// //            elem,
/// //            next: std::mem::replace(&mut self.head, None),
/// //        });
/// //
/// //        self.head = Some(new_node);
/// //    }
///
/// //    pub fn pop(&mut self) -> Option<i32> {
/// //        match std::mem::replace(&mut self.head, None) {
/// //            None => None,
/// //           Some(node) => {
/// //                self.head = node.next;
/// //                Some(node.elem)
/// //           },
/// //        }
/// //    }
/// // }
///
/// // impl Drop for List {
/// //    fn drop(&mut self) {
/// //        let mut cur_link = std::mem::replace(&mut self.head, None);
/// //
/// //        while let Some(mut boxed_node) = cur_link{
/// //            cur_link = std::mem::replace(&mut boxed_node.next, None);
/// //        }
/// //    }
/// // }
///
///
/// 这样稍微好一点了，但是最大的胜利来自Option的方法
///
/// 首先，mem::replace(&mut option, None), 是一个非常常见的惯用法，以至于Option实际上直接将其
/// 变成了一种方法take()
///
///
/// // pub struct List {
/// //     head: Link,
/// // }
/// //
/// // enum Link {
/// //     Empty,
/// //     More(Box<Node>),
/// // }
/// //
/// // struct Node {
/// //     elem: i32,
/// //     next: Link,
/// // }
/// //
/// //
/// //
/// // impl List {
/// //     pub fn new() -> Self {
/// //         List { head: Link::Empty }
/// //     }
/// //
/// //     pub fn push(&mut self, elem: i32) {
/// //         let new_node = Box::new( Node {
/// //             elem,
/// //             next: self.head.take(),
/// //         });
/// //
/// //         self.head = Some(new_node);
/// //     }
/// //
/// //     pub fn pop(&mut self) -> Option<i32> {
/// //         match self.head.take() {
/// //             None => None,
/// //             Some(node) => {
/// //                 self.head = node.next;
/// //                 Some(node.elem)
/// //             },
/// //         }
/// //     }
/// // }
/// //
/// // impl Drop for List {
/// //     fn drop(&mut self) {
/// //         let mut cur_link = self.head.take();
/// //
/// //         while let Some(mut boxed_node) = cur_link {
/// //             cur_link = boxed_node.next.take();
/// //         }
/// //     }
/// // }
///
/// 其次, match 选项match option { None => None, Some(x) => Some(y) } 是一个非常常见的习惯方法，
/// 因此被称为map, Map在Some(x)中的x上执行一个函数，已在Some(y)中生成y。 我们可以编写一个释放的fn并将其
/// 传递给map，但我们更愿意编写内联的操作。
///
/// 做到这一点的方式是闭包，闭包是具有额外超能力的匿名函数： 他们可以引用闭包之外的局部变量！这使得他们在执行各种
/// 条件逻辑时非常有用。我们唯一使用match 是pop，改写下。
///
/// // pub fn pop(&mut self) -> Option<i32> {
/// //     self.head.take().map( |node| {
/// //         self.head = node.next;
/// //         node.elem
/// //    })
/// // }
///
///
/// # 让他们都变成泛型
///
/// 我们已经用Option和Box触及了一些泛型，然而，到目前为止，我们设法避免声明任何实际上的新的类型
///
/// 试试证明这其实很简单，让我们现在把我们所有的类型变成通用类型：
///
///
///
/// // pub struct List<T> {
/// //    head: Link<T>,
/// // }
///
/// // type Link<T> = Option<Box<Node<T>>>;
///
/// // struct Node<T> {
/// //    elem: T,
/// //    next: Link<T>,
/// // }
///
/// 你只是把所有的东西都变得更尖锐一点，然后你的代码就突然变得通用。当然，我们不能这样做，
/// 否则编译器会非常疯狂。
///
/// 问题很明显，我们正在谈论列表的事情，但那已经不再是真实的了，就像Option和Box一样。我们现在
/// 要讨论List<Something>
///
/// 但是我们在impl中使用的something是什么呢？就像List一样，我们希望我们的实现能够与所有的t一起工作。
/// 所以就像List一样， 让我们把我们的impl变得尖锐
///
///
///
/// // impl<T> List<T> {
/// //    pub fn new() -> Self {
/// //        List { head: None }
/// //    }
/// //
/// //    pub fn push(&mut self, elem: T) {
/// //        let new_node = Box::new( Node {
/// //            elem,
/// //            next: self.head.take(),
/// //        });
/// //
/// //        self.head = Some(new_node);
/// //    }
/// //
/// //    pub fn pop(&mut self) -> Option<T> {
/// //        self.head.take().map(|node | {
/// //            self.head = node.next;
/// //            node.elem
/// //        })
/// //    }
/// //}
/// //
/// //impl<T> Drop for List<T> {
/// //    fn drop(&mut self) {
/// //        let mut cur_link = self.head.take();
/// //
/// //        while let Some(mut boxed_node) = cur_link{
/// //            cur_link = boxed_node.next.take();
/// //        }
/// //   }
/// //}
///
/// 我们所有的代码现在完全适用于任何值， rust很容易，我想对那些甚至没有改变的new函数
/// 说一些：
///
/// // pub fn mew() -> Self {
/// //    List { head: None }
/// // }
///
/// 沐浴在Self中，重构和复制编码是编码方式的保证。同样的有趣的，当我们构造一个List的实例时
/// 我们不写List<T>, 这个部分是基于这样一个事实推断出来的，即我们从从一个函数返回它，该函数期望
/// List<T>.
///
///
/// # Peek
///
/// 有一件事我们上次没有去做，那就是peeking, 让我们继续这样做，我们所需要做的就是返回对列表头部的元素的引用，
/// 如果它存在的话。
///
/// pub fn peek(&self) -> Option<&T> {
///     self.head.map(|node| {
///         &node.elem
///     })
/// }
///
/// Map takes self by value, which would move the option out of the thing it's in.
/// 以前这样做很好，因为我们就是把它取出来，用take方法。但是现在，我们实际上想把它留在原来的地方。
/// 正确的处理方法是使用Option上的as_ref方法，
///
/// // impl<T> Option<T> {
/// //    pub fn as_ref(&self) -> Option<&T>;
/// // }
///
/// It demotes the Option to an Option to a reference to its internals.
/// We could do this ourselves with an explicit match but ugh no.
/// It does mean that we need to do an extra dereference to cut through the extra indirection,
/// but thankfully the . operator handles that for us.
///
/// // pub fn peek(&self) -> Option<&T> {
/// //    self.head.as_ref().map(|node| {
/// //        &node.elem
/// //    })
/// // }
///
/// 我们也可以使用as_mut创建一个可变的版本
///
/// // pub fn peek_mut(&mut self) -> Option<&mut T> {
/// //    self.head.as_mut().map(|node| {
/// //        &mut node.elem
/// //   })
/// // }
///
///
/// //     #[test]
/// //    fn peek() {
/// //        let mut list = List::<i32>::new();
/// //        assert_eq!(list.peek(), None);
/// //        assert_eq!(list.peek_mut(), None);
/// //        list.push(1);
/// //        list.push(2);
/// //        list.push(3);
/// //
/// //        assert_eq!(list.peek(), Some(&3));
/// //        assert_eq!(list.peek_mut(), Some(&mut 3));
/// //    }
///
/// 这很好，但是我们并没有真正测试是否可以改变这个peek_mut的返回值。如果一个引用是可变的，但是
/// 没有人对它，进行改变，那么我们真的测试过它的可变性了吗？让我们尝试在这个Option<&mut T> 上使用map 来
/// 重新输出一个改变的值
///
/// //     #[test]
/// //    fn peek() {
/// //        let mut list = List::<i32>::new();
/// //        assert_eq!(list.peek(), None);
/// //        assert_eq!(list.peek_mut(), None);
/// //        list.push(1);
/// //        list.push(2);
/// //        list.push(3);
/// //
/// //        assert_eq!(list.peek(), Some(&3));
/// //        assert_eq!(list.peek_mut(), Some(&mut 3));
/// //        list.peek_mut().map(|&mut value| {
/// //            value = 42
/// //        });
/// //
/// //        assert_eq!(list.peek(), Some(&42));
/// //        assert_eq!(list.pop(), Some(42));
/// //
/// //   }
///
/// 编译器抱怨值是不可变的，但是我们非常清楚的写了&mut value, 这是怎么回事呢？事实证明，以这种方式编写的闭包参数
/// 并不能执行这个值是一个可变引用，相反，它创建的模式将与这个参数匹配到闭包了 |&mut value| 的意思是"参数是一个可变引用
/// ，当请将它指向的值复制到value中，（这里将相当于重新绑定了一个不可变的值）， 如果只使用|value| 那么
/// value的类型将是&mut i32, 我们实际上可以改变head
///
/// //     #[test]
/// //    fn peek() {
/// //        let mut list = List::<i32>::new();
/// //        assert_eq!(list.peek(), None);
/// //        assert_eq!(list.peek_mut(), None);
/// //        list.push(1);
/// //        list.push(2);
/// //        list.push(3);
/// //
/// //        assert_eq!(list.peek(), Some(&3));
/// //        assert_eq!(list.peek_mut(), Some(&mut 3));
/// //
/// //        list.peek_mut().map(|value| {
/// //            *value = 42
/// //        });
/// //
/// //        assert_eq!(list.peek(), Some(&42));
/// //        assert_eq!(list.pop(), Some(42));
/// //     }
///
///
/// # IntoIter
///
/// 集合使用Iterator特性在Rust中迭代，但是比Drop稍微复杂一点：
///
/// // pub trait Iterator {
/// //    type Item;
/// //    fn next(&mut self) -> Option<Self::Item>;
/// // }
///
///
/// 第一就是type Item, 这意味着Iterator的每个实现都有一个关联类型，称为Item, 在这种情况下，当你下次调用
/// 时，它可以吐出这种类型
///
/// 迭代器产生Option<Self::Item>的原因是接口合并了has_next和get_next的概念。 当你有一个值时，你产生一些值，
/// 当你没有产生一个，这使得API在使用和实现上更符合人体工程学，更安全，同时避免了冗余的检查和has_next和get_next之间的逻辑
///
/// 遗憾是，Rust还没有一个像yield statement, 所以我们必须自己实现这个逻辑，此外，实际上每个集合
/// 应该努力实现三种不同的迭代器：
/// - IntoIter - T
/// - IterMut - &mut T
/// - Iter - &T
///
/// 实际上，我们已经有了所有的工具，可以使用List的接口实现IntoIter; 只需要一遍一遍地调用pop, 因此，我们只需
/// 实现作为List的新类型包装器的IntoIter;
///
/// // tuple structs are an alternative form of struct,
/// // useful for trivial wrappers around other types
/// // pub struct IntoIter<T>(List<T>);
/// //
/// // impl <T> List<T> {
/// //    pub fn into_iter(self) -> IntoIter<T> {
/// //        IntoIter(self)
/// //   }
/// // }
/// //
/// // impl <T> Iterator for IntoIter<T> {
/// //    type  Item = T;
/// //    fn next(&mut self) -> Option<Self::Item> {
/// //        self.0.pop()
/// //   }
/// // }
/// //     #[test]
/// //    fn into_iter() {
/// //        let mut list = List::new();
/// //        list.push(1);
/// //        list.push(2);
/// //        list.push(3);
/// //
/// //        let mut iter = list.into_iter();
/// //        assert_eq!(iter.next(), Some(3));
/// //        assert_eq!(iter.next(), Some(2));
/// //        assert_eq!(iter.next(), Some(1));
/// //        assert_eq!(iter.next(), None);
/// //    }
///
///
/// # Iter
///
/// 开始实现iter, 这一次，不能依靠List来提供我们想要的功能，要自己动手了，
/// 我们需要的基本逻辑时保持一个指针，指向我们下一个要生成的当前节点。 因为该节点可能不存在
/// （列表为空或者我们已经进行了迭代）， 所以我们希望引用是Option， 当我们产生一个元素时，我们希望继续到
/// 当前节点的下一个节点
///
///
///
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new( Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node | {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
        }
    }
}

// tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types
pub struct IntoIter<T>(List<T>);

impl <T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <T> Iterator for IntoIter<T> {
    type  Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a,T> {
    next: Option<&'a Node<T>>,
}

impl <T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node )}
    }
}

impl<'a, T> Iterator for Iter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}


pub struct IterMut<'a,T> {
    next: Option<&'a mut Node<T>>,
}

impl <T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node )}
    }
}

impl<'a, T> Iterator for IterMut<'a,T> {
    type Item = &'a mut  T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}
#[cfg(test)]
mod test {
    // use crate::first::List;
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        //检查空列表行为正确
        assert_eq!(list.pop(), None);

        // 填充列表
        list.push(1);
        list.push(2);
        list.push(3);


        // 检查通常的移除
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //push 更多元素来确认没有问题
        list.push(4);
        list.push(5);

        //检查通常的移除
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //检查完全移除
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::<i32>::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
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
    }
}




