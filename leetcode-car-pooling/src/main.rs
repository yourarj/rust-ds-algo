use std::collections::BTreeMap;

fn main() {
    let input = vec![vec![2, 1, 5], vec![3, 3, 7]];
    let capacity = 4;
    let _output = Solution::car_pooling(input, capacity);
}
struct Solution;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut occupancy_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut result = true;
        for trip in trips {
            let passengers: i32 = trip[0];
            let on_board_at: i32 = trip[1];
            let off_board_at: i32 = trip[2];

            if passengers > capacity {
                result = false;
            } else {
                for distance in on_board_at..=off_board_at {
                    let current_passengers = occupancy_map.get(&distance).unwrap_or(&0);
                    if current_passengers + passengers > capacity {
                        result = false;
                    } else {
                        *occupancy_map.entry(distance).or_insert(0) += passengers;
                    }
                }
            };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    #[test]
    fn positive_test() {
        let input = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity = 4;
        assert_eq!(false, Solution::car_pooling(input, capacity))
    }
    #[test]
    fn negative_test() {
        let input = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity = 5;
        assert_eq!(true, Solution::car_pooling(input, capacity))
    }
    #[test]
    fn negative_test_more_passenger_than_capacity() {
        let input = vec![vec![2, 1, 5], vec![3, 3, 7], vec![100, 6, 7]];
        let capacity = 100;
        assert_eq!(false, Solution::car_pooling(input, capacity))
    }
}
