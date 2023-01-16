use super::Solution;

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut used_k: Vec<i32> = vec![0; 26];
        s.as_bytes()
            .iter()
            .zip(t.as_bytes().iter())
            .for_each(|(&s_item, &t_item)| {
                let left = s_item as i32;
                let right = t_item as i32;
                // + 26 保证分子是正数，因为 s_item 可能小于 t_item
                let mod_number = (right - left + 26) % 26;
                println!("{left} {right} {mod_number}");

                used_k[mod_number as usize] += 1;
            });
        // ab -> bd; ab -> bc
        // '1' -> 1, '2' -> 1; '1' -> 2
        println!("{:?}", used_k);
        for (i, _) in used_k.iter().enumerate().take(26).skip(1) {
            println!("i: {i} left {} k: {}", i as i32 + (used_k[i] - 1) * 26, k);

            if i as i32 + (used_k[i] - 1) * 26 > k {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_convert_string_should_work() {
        assert_eq!(
            Solution::can_convert_string(String::from("input"), String::from("ouput"), 9),
            true
        );

        // assert_eq!(
        //     Solution::can_convert_string(String::from("abc"), String::from("bcd"), 10),
        //     false
        // );

        assert_eq!(
            Solution::can_convert_string(String::from("atmtxzjkz"), String::from("tvbtjhvjd"), 35),
            false
        )
    }
}
