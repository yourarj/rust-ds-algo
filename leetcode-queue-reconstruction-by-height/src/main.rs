use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pep_ref = people.clone();
        pep_ref.sort_by(|one, two| {
            let mut ord = Ordering::Equal;
            if two[0] < one[0] {
                ord = Ordering::Less;
            } else if two[0] > one[0] {
                ord = Ordering::Greater;
            } else {
                if two[1] > one[1] {
                    ord = Ordering::Less;
                } else if two[1] < one[1]{
                    ord = Ordering::Greater;
                }
            }
            ord
        });

        let mut reconstructed_ref = Vec::<Vec<i32>>::new();

        for element in pep_ref.into_iter() {
            reconstructed_ref.insert(element[1] as usize, element);
        }

        reconstructed_ref
    }
}

fn main() {
    assert_eq!(
        vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1]
        ],
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ]),
    );
    assert_eq!(
        vec![
            vec![4, 0],
            vec![5, 0],
            vec![2, 2],
            vec![3, 2],
            vec![1, 4],
            vec![6, 0]
        ],
        Solution::reconstruct_queue(vec![
            vec![6, 0],
            vec![5, 0],
            vec![4, 0],
            vec![3, 2],
            vec![2, 2],
            vec![1, 4]
        ]),
    );

    assert_eq!(
        vec![
            vec![3, 0],
            vec![6, 0],
            vec![7, 0],
            vec![5, 2],
            vec![3, 4],
            vec![5, 3],
            vec![6, 2],
            vec![2, 7],
            vec![9, 0],
            vec![1, 9]
        ],
        Solution::reconstruct_queue(vec![
            vec![9, 0],
            vec![7, 0],
            vec![1, 9],
            vec![3, 0],
            vec![2, 7],
            vec![5, 3],
            vec![6, 0],
            vec![3, 4],
            vec![6, 2],
            vec![5, 2]
        ])
    )
}
