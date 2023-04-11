// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// populating singly linked list from array
pub fn from_array(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = None;

    for val in arr {
        let new_node = ListNode::new(*val);

        if tail.is_none() {
            // This is the first node in the list
            head = Some(Box::new(new_node));
            tail = Some(head.as_mut());
        } else {
            // Append new node to the tail of the list
            if let Some(Some(ref mut node)) = tail {
                node.next = Some(Box::new(new_node));
            }

            tail = tail.flatten().map(|val| val.next.as_mut());
        }
    }
    head
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(n)) | (Some(n), None) => Some(n),
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    Some(Box::new(ListNode {
                        val: l.val,
                        next: Self::merge_two_lists(Some(r), l.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: r.val,
                        next: Self::merge_two_lists(Some(l), r.next),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = from_array(&[1, 2, 4]);
        let list2 = from_array(&[1, 3, 4]);
        let expected = from_array(&[1, 1, 2, 3, 4, 4]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }
    #[test]
    fn it_works_02() {
        let list1 = from_array(&[]);
        let list2 = from_array(&[]);
        let expected = from_array(&[]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }
    #[test]
    fn it_works_03() {
        let list1 = from_array(&[]);
        let list2 = from_array(&[1]);
        let expected = from_array(&[1]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }
}
