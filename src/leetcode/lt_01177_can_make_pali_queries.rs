use super::Solution;

impl Solution {
    // pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    //     // 1. 将字符串 s 转换为数字
    //     // 2. 构建一个长度为 s 的长度的 vector，且每个元素都是一个长度为 26 的 vector，初始值为 0。 这个的作用是统计到第 n 位为止，每个数字出现的次数。
    //     // 3. 构建一个长度为 queries 的长度的 vector，初始值为 false。
    //     // 4. 遍历 queries，根据 query 的 0 和 1 得到子串中的每个数字的个数，并统计其中奇数数字的个数 odd_count，如果子串的长度是奇数，那么 odd_count = odd_count + 1。
    //     // 5. 计算可调整的次数 = query[2]，如果子串长度是奇数，那么可以保留一个奇数，所以调整的次数多一次。
    //     // 6. 如果 odd_count / 2 小于或等于 可调整的次数，则子串可以改为回文。

    //     let queries_length = queries.len();
    //     let mut res = vec![false; queries_length];

    //     let s_length = s.len();
    //     let s = s.chars().map(|c| (c as u8 - 'a' as u8) as usize).collect::<Vec<usize>>();

    //     let mut memo = vec![vec![0; 26]; s_length + 1];
    //     for i in 0..s_length {
    //         memo[i+1] = memo[i].clone();
    //         memo[i+1][s[i]] += 1;
    //     }

    //     for (idx, query) in queries.iter().enumerate() {
    //         let left = query[0] as usize;
    //         let right = query[1] as usize;

    //         let odd_chance = (right - left + 1) % 2;
    //         let total_chance = query[2] as usize + odd_chance;

    //         let mut odd_count = 0;
    //         for letter in 0..26 {
    //             odd_count += (memo[right+1][letter] - memo[left][letter]) % 2
    //         }
    //         println!("odd_count: {:?}", odd_count);
    //         odd_count = odd_count + odd_count % 2;
    //         println!("odd_count / 2: {:?}", odd_count);
    //         println!("total_chance: {:?}", total_chance);

    //         res[idx] = odd_count / 2 <= total_chance;
    //     }

    //     res
    // }

    // pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    //     let len = s.len();
    //     let mut prefix = vec![0; len + 1];
    //     let mut bits = 0;
    //     for (i, &byte) in s.as_bytes().iter().enumerate() {
    //         let idx = (byte - b'a') as i32;
    //         let bit = 1 << idx;
    //         println!("bit: {:08b}", bit);
    //         if bits & bit != 0 {
    //             bits &= !bit;
    //             println!("!0: bits: {:08b}", bits);
    //         } else {
    //             bits |= bit;
    //             println!("0: bits: {:08b}", bits);
    //         }
    //         prefix[i + 1] = bits;
    //     }
    //     println!("prefix:{:?}", prefix);
    //     let mut ans = vec![false; queries.len()];
    //     for (i, query) in queries.iter().enumerate() {
    //         let left = prefix[query[0] as usize];
    //         let right = prefix[query[1] as usize + 1];
    //         let diff = (i32::from(left ^ right)).count_ones() as i32;
    //         ans[i] = diff / 2 <= query[2];
    //     }
    //     ans
    // }

    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut cnt = vec![0];

        for (idx, v) in s.bytes().enumerate() {
            cnt.push(cnt[idx] ^ (1 << (v - 97)))
        }

        queries
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize, v[2] as u32))
            .map(|(left, right, chance_count)| (cnt[left] ^ cnt[right + 1]) / 2 <= chance_count)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_make_pali_queries_should_work() {
        println!("1177. 构建回文串检测(can_make_pali_queries)");

        // "rkzavgdmdgt"
        // "01234567890"
        // [[5,8,0],[7,9,1],[3,6,4],[5,5,1],[8,10,0],[3,9,5],[0,10,10],[6,8,3]]
        let res =
            Solution::can_make_pali_queries(String::from("rkzavgdmdgt"), vec![vec![8, 10, 0]]);

        assert_eq!(vec![false], res);

        // & 位与	相同位置均为1时则为1，否则为0
        // | 位或	相同位置只要有1时则为1，否则为0
        // ^ 异或	相同位置不相同则为1，相同则为0
        // ! 位非	把位中的0和1相互取反，即0置为1，1置为0
        // << 左移	所有位向左移动指定位数，右位补零
        // >> 右移	所有位向右移动指定位数，左位补零
        // println!("位与：3 & 4: {:08b}", 3 & 4);
        // println!("位与：3 & 4: {:?}", 3 & 4);
        // println!("位或：3 | 4: {:08b}", 3 | 4);
        // println!("异或：3 ^ 4: {:08b}", 3 ^ 4);
        // println!("位非：!4: {:08b}", !4);
        // println!("a: {:08b} {:?}", b'a', b'a');
        // println!("a: {:08b} {:?}", 1 << (b'a' - 97), 1 << (b'a' - 97));
        // println!("a: {:08b}", 1 << (b'b' - 97));
    }
}
