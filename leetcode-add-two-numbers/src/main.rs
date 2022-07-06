impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut linked_list_dummy_head = ListNode::new(0);
        let mut current_node_ref = &mut linked_list_dummy_head;
        let mut node_from_one = l1;
        let mut node_from_two = l2;

        let mut carry_forward: i32 = 0;

        while node_from_one != None || node_from_two != None {
            let sum = match (&node_from_one, &node_from_two) {
                (Some(one), Some(two)) => one.val + two.val + carry_forward,
                (Some(one), None) => one.val + carry_forward,
                (None, Some(two)) => two.val + carry_forward,
                (None, None) => carry_forward,
            };

            carry_forward = sum / 10;
            current_node_ref.next = Some(Box::new(ListNode::new(sum % 10)));
            current_node_ref = current_node_ref.next.as_mut().unwrap();

            node_from_one = if node_from_one != None {
                node_from_one.unwrap().next
            } else {
                node_from_one
            };
            node_from_two = if node_from_two != None {
                node_from_two.unwrap().next
            } else {
                node_from_two
            };
        }
        if carry_forward > 0 {
            current_node_ref.next = Some(Box::new(ListNode::new(carry_forward)));
        }

        linked_list_dummy_head.next
    }
}

fn main() {
    let one = Option::Some(Box::new(ListNode {
        val: 2,
        next: Option::Some(Box::new(ListNode {
            val: 4,
            next: Option::Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    let two = Option::Some(Box::new(ListNode {
        val: 5,
        next: Option::Some(Box::new(ListNode {
            val: 6,
            next: Option::Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let sol = Option::Some(Box::new(ListNode {
        val: 7,
        next: Option::Some(Box::new(ListNode {
            val: 0,
            next: Option::Some(Box::new(ListNode { val: 8, next: None })),
        })),
    }));

    let real_outcome = Solution::add_two_numbers(one, two);

    assert_eq!(real_outcome, sol);
}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
