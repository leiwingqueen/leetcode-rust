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
    //哑节点，方便删除操作
    let mut dummy = Box::new(ListNode { val: MIN, next: head });
    //需要修改pre的next指针，因此类型需要是&mut,同时pre的指针也会修改，所以定义的时候也需要是mut
    let mut pre = &mut dummy;
    //不能直接用head，因为head的所有权已经给dummy了
    let mut cur = &mut pre.next;
    while let Some(ref mut node) = cur {
        //删除节点
        /*if node.val == pre.val {
            //let next_node = node.next;
            (*pre).next = node.next.take();
            cur = &mut pre.next;
        } else {
            pre = node;
            cur = &mut node.next;
        }*/
        cur = &mut node.next;
    }
    return dummy.next;
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

