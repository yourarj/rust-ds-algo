fn main() {}

use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complement_map_with_index = HashMap::<i32, i32>::new();
    let mut answer = Vec::new();
    for (index, element) in nums.into_iter().enumerate() {
        if complement_map_with_index.contains_key(&element) {
            answer.push(*complement_map_with_index.get(&element).unwrap());
            answer.push(index as i32);
            break;
        }
        let complement = target - element;
        complement_map_with_index.insert(complement, index as i32);
    }
    answer
}
