use crate::leetcode::Solution;
use std::collections::HashMap;

static MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        let mut hash_map = HashMap::new();
        arr.iter().enumerate().fold(0, |mut count, (idx, &item)| {
            let tmp = Self::count_items(&hash_map, item, &arr[..idx]);
            hash_map.insert(item, tmp);
            count += tmp;
            count % MOD
        })
    }

    fn count_items(cache: &HashMap<i32, i32>, target: i32, items: &[i32]) -> i32 {
        let mut count = 1;
        for item in items {
            if target % item == 0 {
                if let Some(&m) = cache.get(item) {
                    if let Some(&n) = cache.get(&(target / item)) {
                        count += (m as i64 * n as i64 % MOD as i64) as i32;
                        count %= MOD;
                    }
                }
            }
        }
        count
    }
}
