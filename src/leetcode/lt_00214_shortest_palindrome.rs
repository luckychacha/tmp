use crate::leetcode::Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut palindrome_end = s.len();
        let chars = s.chars().collect::<Vec<char>>();

        if palindrome_end <= 1 || Solution::is_palindrome(&chars, palindrome_end) {
            return s;
        }
        palindrome_end -= 1;
        while palindrome_end >= 0 {
            if Solution::is_palindrome(&chars[0..palindrome_end], palindrome_end) {
                break;
            }
            palindrome_end -= 1;
        }

        chars[palindrome_end..]
            .iter()
            .rev()
            .collect::<String>() + &s
    }

    fn is_palindrome(source: &[char], size: usize) -> bool {
        if size == 1 {
            return true;
        }
        let mut i = 0;
        let mut j = size - 1;
        while i <= j {
            if source[i] != source[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}