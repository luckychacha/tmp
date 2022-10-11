
use super::Solution;
/// 394. 字符串解码
/// 给定一个经过编码的字符串，返回它解码后的字符串。
/// 
/// 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
/// 
/// 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
/// 
/// 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。

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