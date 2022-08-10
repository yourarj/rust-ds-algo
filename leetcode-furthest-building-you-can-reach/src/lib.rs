pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let size = heights.len();
        let mut min_heap = BinaryHeap::new();

        for i in 0..size - 1 {
            let diff = heights[i + 1] - heights[i];

            if diff > 0 {
                min_heap.push(-diff);
            }

            if min_heap.len() > ladders as usize {
                if let Some(min) = min_heap.pop() {
                    bricks += min;
                    if bricks < 0 {
                        return i as i32;
                    }
                }
            }
        }
        size as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let input_heights = vec![4, 2, 7, 6, 9, 14, 12];
        let input_bricks = 5;
        let input_ladders = 1;
        let expected_result = 4;
        let actual_result = Solution::furthest_building(input_heights, input_bricks, input_ladders);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_02() {
        let input_heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let input_bricks = 10;
        let input_ladders = 2;
        let expected_result = 7;
        let actual_result = Solution::furthest_building(input_heights, input_bricks, input_ladders);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_03() {
        let input_heights = vec![14, 3, 19, 3];
        let input_bricks = 17;
        let input_ladders = 0;
        let expected_result = 3;
        let actual_result = Solution::furthest_building(input_heights, input_bricks, input_ladders);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_04() {
        let input_heights = vec![14, 3, 22, 3];
        let input_bricks = 17;
        let input_ladders = 0;
        let expected_result = 1;
        let actual_result = Solution::furthest_building(input_heights, input_bricks, input_ladders);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_05() {
        let input_heights = vec![3, 21, 3, 4, 5, 6, 7, 8, 9, 10, 25, 26, 27, 28, 35];
        let input_bricks = 17;
        let input_ladders = 2;
        let expected_result = 14;
        let actual_result = Solution::furthest_building(input_heights, input_bricks, input_ladders);
        assert_eq!(actual_result, expected_result);
    }
}
