use super::Solution;

// https://leetcode.cn/problems/reorganize-string/
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut frequency = vec![0; 26];
        let n = s.len();
        let mut max = 0;
        s.bytes().into_iter().for_each(|letter| {
            frequency[(letter - 97) as usize] += 1;
            max = max.max(frequency[(letter - 97) as usize]);
        });
        if max > (n / 2 + n % 2) {
            return String::from("");
        }

        let mut frequency: Vec<Vec<u8>> = frequency
            .into_iter()
            .enumerate()
            .filter(|&item| item.1 > 0)
            .map(|item| vec![(item.0 + 97) as u8; item.1])
            .collect();
        frequency.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

        let mut answer: Vec<u8> = vec![0; n];
        let mut bytes: Vec<u8> = frequency.into_iter().flatten().collect();

        (0..n).step_by(2).for_each(|idx| {
            answer[idx] = bytes.pop().unwrap();
        });
        (1..n).step_by(2).for_each(|idx| {
            answer[idx] = bytes.pop().unwrap();
        });

        // answer.iter().map(|&n| n as char).collect()
        String::from_utf8(answer).unwrap()
    }
}
