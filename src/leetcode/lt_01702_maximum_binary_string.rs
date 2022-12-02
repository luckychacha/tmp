use super::Solution;

/// 1702. 修改后的最大二进制字符串
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        if binary.is_empty() {
            return binary;
        }

        if !binary.contains('0') {
            return binary;
        }
        let mut start_idx = binary.len() + 1;

        // let mut flag = 0;
        let mut zero_count = 0;

        for (flag, c) in binary.chars().enumerate() {
            if c == '0' {
                if flag < start_idx {
                    start_idx = flag;
                } else {
                    zero_count += 1;
                }
            }
            // flag += 1;
        }

        let zero_position = start_idx + zero_count;

        let mut tmp = vec![1; binary.len()];
        tmp[zero_position] = 0;

        tmp.into_iter().map(|i| i.to_string()).collect::<String>()
    }

    pub fn better_maximum_binary_string(binary: String) -> String {
        let zero_count = binary.bytes().filter(|&b| b == b'0').count();
        let continous_one_count = binary.bytes().take_while(|&b| b == b'1').count();

        let mut binary = binary.into_bytes();

        if zero_count > 1 {
            binary = vec![b'1'; binary.len()];
            binary[zero_count + continous_one_count - 1] = b'0';
        }

        String::from_utf8(binary).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn better_maximum_binary_string_should_work() {
        let res = Solution::better_maximum_binary_string(String::from("000110"));

        println!("res: {:?}", res);
    }
}
