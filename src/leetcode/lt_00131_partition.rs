use super::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        Solution::backstrace(&s.chars().collect(), 0, &mut res, &mut vec![]);
        res
    }

    fn backstrace(
        s: &Vec<char>,
        start_index: usize,
        res: &mut Vec<Vec<String>>,
        path: &mut Vec<String>,
    ) {
        if start_index >= s.len() {
            res.push(path.clone());
            return;
        }
        for i in start_index..s.len() {
            match Solution::checked(start_index, i, s) {
                true => {
                    path.push(s[start_index..=i].iter().collect());
                    Solution::backstrace(s, i + 1, res, path);
                    path.pop();
                }
                _ => continue,
            }
        }
    }

    fn checked(start: usize, end: usize, s: &[char]) -> bool {
        let mut start = start;
        let mut end = end;
        while start < end {
            if s.get(start) != s.get(end) {
                return false;
            }
            start += 1;
            end -= 1;
        }

        true
    }
}
