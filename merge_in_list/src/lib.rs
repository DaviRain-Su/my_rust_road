// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}
// 使用归并排序算法对单链表进行排序，然后输出结果 
// merge sort
#[inline]
pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let len = length(&head);

    let mut ptr = head.as_mut();
    for _ in 0..(len / 2) {
        if let Some(node) = ptr {
            ptr = node.next.as_mut();
        }
    }

    let (right, left) = (ptr.cloned().take(), head);

    match (left, right) {
        (None, None) => None,
        (left, None) => left,
        (None, right) => right,
        (left, right) => merge_two_lists(left, right),
    }

}
fn length(mut head: &Option<Box<ListNode>>) -> usize {
    let mut count = 0;
    while let Some(node) = head {
        head = &node.next;
        count += 1;
    }
    count
}
// 合并两个链表
fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result = ListNode::new(0);
    let mut p = &mut result;

    while l1.is_some() && l2.is_some() {
        let deref_val = |node: &Box<ListNode>| (*node).val;
        let digit1 = l1.as_ref().map_or(0, deref_val);
        let digit2 = l2.as_ref().map_or(0, deref_val);
        if digit1 <= digit2 {
            let tmp = l1.as_mut().unwrap().next.take();
            p.next = l1.take();
            l1 = tmp;
        } else {
            let tmp = l2.as_mut().unwrap().next.take();
            p.next = l2.take();
            l2 = tmp;
        }
        p = p.next.as_mut().unwrap();

        let deref_next = |node: Box<ListNode>| (*node).next;

        l1 = l1.and_then(deref_next);
        l2 = l2.and_then(deref_next);
    }

    if l1.is_none() {
        p.next = l2.take();
    } else {
        p.next = l1.take();
    }
    result.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_148() {
        let example1 = to_list(vec![4, 2, 1, 3]);
        // println!("example1 = {:?}", example1);
        println!("[1, 2, 3, 4] = {:?}", to_list(vec![1,2, 3, 4]));
        // println!("sorted = {:?}", Solution::sort_list(example1));
        assert_eq!(sort_list(example1), to_list(vec![1, 2, 3, 4]));
        // 实际返回 to_list(vec![1,2,1,3])
        // let example2 = to_list(vec![-1, 5, 3, 4, 0]);
        // println!("example2 = {:?}", example2);
        // assert_eq!(Solution::sort_list(example2), to_list(vec![-1, 0, 3, 4, 5]));
    }
}