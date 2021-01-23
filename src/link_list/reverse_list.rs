/*
反转一个单链表。

示例:

输入: 1->2->3->4->5->NULL
输出: 5->4->3->2->1->NULL
进阶:
你可以迭代或递归地反转链表。你能否用两种方法解决这道题？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reverse-linked-list
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

use crate::link_list::util::ListNode;
use std::borrow::Borrow;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut lhs = head;
    let mut rhs = None;
    while let Some(mut node) = lhs {
        lhs = node.next.take();
        node.next = rhs;
        rhs = Some(node);
    }
    rhs
}

#[test]
fn test() {
    let mut n1 = Some(Box::new(ListNode::new(1)));
    let mut n2 = Some(Box::new(ListNode::new(2)));
    let mut n3 = Some(Box::new(ListNode::new(3)));
    //n1.borrow().unwrap().next = n2;
    //let n2 = n1.unwrap().next;
    //n2.unwrap().next = n3;
    let mut res = reverse_list(n1);
    while res.is_some() {
        let x = res.unwrap();
        print!("{}->", x.val);
        res = x.next;
    }
}

fn build_link_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut idx = nums.len() - 1;
    let dummy = ListNode::new(0);
    let mut pre = ListNode::new(0);
    return None;
}
