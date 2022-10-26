use super::Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        // return Solution::dp(s);

        fn mid(chars: &[char], i: usize, j: usize, char_len: usize) -> i32 {
            let mut ans = 0;
            let mut i = i as i32;
            let mut j = j;
            while i >= 0 && j < char_len && chars[i as usize] == chars[j] {
                ans += 1;
                i -= 1;
                j += 1;
            }
            ans
        }

        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut ans = 0;

        for i in 0..n {
            ans += mid(&s, i, i, n);
            ans += mid(&s, i, i + 1, n);
        }

        ans
    }

    fn dp(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![false; n]; n];

        let mut ans = 0;

        for i in (0..n).rev() {
            dp[i][i] = true;
            ans += 1;
            // i = n-1 时不走 j 的 for 循环
            for j in i + 1..n {
                if s[i] != s[j] {
                    continue;
                }
                if i + 1 == j || i + 2 == j || dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_substrings_should_work() {
        assert_eq!(Solution::count_substrings(String::from("aaa")), 6);
    }
}
