// use super::Solution;
use crate::leetcode::Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut additional_rocks = additional_rocks;
        let mut rest_capacity = capacity
            .iter()
            .zip(rocks.iter())
            .map(|(&capacity_item, &rocks_item)| capacity_item - rocks_item)
            .collect::<Vec<i32>>();

        // 改用 sort_unstable 用时节约一半
        // rest_capacity.sort();
        rest_capacity.sort_unstable();

        // fold 可以代替在循环前初始化一个变量，在循环中不断处理这个值的场景
        rest_capacity.iter().fold(0, |maximum_bags, &item| {
            if item <= additional_rocks {
                additional_rocks -= item;
                maximum_bags + 1
            } else {
                maximum_bags
            }
        })

        // let mut maximum_bags = 0;
        // for item in rest_capacity {
        //     if item == 0 || item <= additional_rocks {
        //         maximum_bags += 1;
        //         additional_rocks -= item;
        //     } else {
        //         break;
        //     }
        // }
        // maximum_bags
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn maximum_bags_should_work() {
        assert_eq!(
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100),
            3
        );
        assert_eq!(
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2),
            3
        );
    }
}
