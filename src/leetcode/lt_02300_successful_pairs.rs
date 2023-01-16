use crate::leetcode::Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        let potion_count = potions.len();
        potions.sort_unstable();

        let mut result = vec![0; spells.len()];

        spells.iter().enumerate().for_each(|(idx, &spell)| {
            let mut left = 0;
            let mut right = potion_count;

            while left < right {
                let mid = left + (right - left) / 2;
                if potions[mid] as i64 * spell as i64 >= success {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            result[idx] = (potion_count - left) as i32;
        });

        result
    }
}
