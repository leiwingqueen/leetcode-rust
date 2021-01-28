/*
给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。

示例 1:

输入: 1->1->2
输出: 1->2
示例 2:

输入: 1->1->2->3->3
输出: 1->2->3

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
由于链表是排序的，理论上我们只关注跟上一个元素是否相同即可知道
 */
use crate::link_list::util::ListNode;
use std::i32::MIN;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut dummy = Some(Box::new(ListNode { val: MIN, next: head.clone() }));
    let mut pre = &dummy;
    let mut cur = &mut head.clone();
    while let Some(node) = cur {
        println!("{}",node.val);
        /*let n = pre.unwrap();
        if node.val == n.val {
            let next_node = &mut node.next;
            node.next = next_node.take();
        }*/
        cur = &mut node.next;
    }
    return dummy.unwrap().next;
}

fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode { val: 1, next: head }));
    let mut root = &mut head;
    while let Some(node) = root {
        let next_node = &mut node.next;
        match next_node {
            None => break,
            Some(next_node) => {
                if next_node.val == val {
                    // 当前节点的下一个节点等于目标节点
                    node.next = next_node.next.take();
                    break;
                }
            }
        }
        root = &mut node.next;
    }
    head.unwrap().next
}

#[test]
fn test() {
    let mut head = Option::Some(Box::new(ListNode { val: 1, next: Option::Some(Box::new(ListNode::new(2))) }));
    let res = delete_duplicates(head);
}

