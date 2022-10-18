use super::Solution;
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut front = 0;

        // let mut a: Vec<bool> = vec![false; 50];
        // let mut aa: Box<[bool]> = [false; 50].iter().cloned().collect();
        let mut left: Box<[i32]> = [0; 50].iter().cloned().collect();
        // let mut left: vec![0; 50];

        // top
        let top: i32 = grid
            .iter()
            .enumerate()
            .map(|(_, item)| {
                let mut item_max = 0;
                let item_count = item
                    .iter()
                    .enumerate()
                    .map(|(idx_y, &item)| {
                        left[idx_y] = left[idx_y].max(item);
                        item_max = item_max.max(item);
                        if item > 0 {
                            return 1;
                        }
                        0
                    })
                    .sum::<i32>();
                front += item_max;
                item_count
            })
            .sum::<i32>();

        front + left.iter().sum::<i32>() + top
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn projection_area_should_work() {
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);

        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    }
}
