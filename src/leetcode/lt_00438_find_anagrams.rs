use super::Solution;
// 438. 找到字符串中所有字母异位词

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s = s.bytes().collect::<Vec<u8>>();
        let p = p.bytes().collect::<Vec<u8>>();

        let mut res = vec![];
        let mut target = vec![0; 255];
        let mut current = vec![0; 255];

        p.iter().for_each(|&c| target[c as usize] += 1);

        for i in 0..s.len() - (p.len() - 1) {
            if i == 0 {
                for j in 0..p.len() {
                    current[s[j] as usize] += 1;
                }
            } else {
                current[s[i - 1] as usize] -= 1;
                current[s[i + (p.len() - 1)] as usize] += 1;
            }

            if current == target {
                res.push(i as i32);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_anagrams_should_work() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc"))
        );

        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams(String::from("abab"), String::from("ab"))
        );

        assert_eq!(
            vec![1],
            Solution::find_anagrams(String::from("baa"), String::from("aa"))
        );
    }
}
