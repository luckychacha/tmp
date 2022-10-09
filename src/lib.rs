pub struct Solution();

impl Solution {
    pub fn decode_string(s: String) -> String {
        Self::decode(&mut s.chars())
    }

    fn decode(iter: &mut std::str::Chars) -> String {
        let mut number_cache = String::new();
        let mut s_cache = String::new();

        while let Some(c) = iter.next() {
            match c {
                '[' => {
                    let inner = Self::decode(iter);
                    s_cache.push_str(&inner.repeat(number_cache.parse::<usize>().unwrap_or(1)));
                    number_cache = String::new();
                }
                ']' => {
                    break;
                }
                '0'..='9' => number_cache.push(c),
                _ => s_cache.push(c),
            }
        }
        s_cache.repeat(number_cache.parse::<usize>().unwrap_or(1))
    }
}

// fn main() {
//     let res = Solution::decode_string("3[a]2[bc]".to_string());

//     println!("res: {:?}", res);
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work() {
        let t = |s: &str| Solution::decode_string(s.into());
        assert_eq!("aaabcbc", t("3[a]2[bc]"));
        assert_eq!("accaccacc", t("3[a2[c]]"));
        assert_eq!("abcabccdcdcdef", t("2[abc]3[cd]ef"));
        assert_eq!("abccdcdcdxyz", t("abc3[cd]xyz"));
    }
}