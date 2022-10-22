use super::Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let letters: Vec<char> = s.chars().collect();
        let mut res = vec!['a'; s.len()];
        for (idx, &item) in indices.iter().enumerate() {
            res[item as usize] = letters[idx];
        }

        res.iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn restore_string_should_work() {
        assert_eq!(
            Solution::restore_string(String::from("codeleet"), vec![4,5,6,7,0,2,1,3]),
            String::from("leetcode")
        );
    }
}