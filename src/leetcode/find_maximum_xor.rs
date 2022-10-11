use super::Solution;

/// 剑指 Offer II 067. 最大的异或
#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, num: i32) {
        let mut s = self;

        for i in (0..31).rev() {
            let index = if (num >> i) & 1 == 1 { 1 } else { 0 };
            s = s.children[index].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    pub fn check(&self, num: i32) -> i32 {
        let mut res = 0;

        let mut s = self;

        for i in (0..31).rev() {
            if (num >> i) & 1 == 0 {
                if let Some(ref child) = s.children[1] {
                    s = child;
                    res = (res << 1) + 1;
                } else if let Some(ref child) = s.children[0] {
                    s = child;
                    res <<= 1;
                }
            } else if let Some(ref child) = s.children[0] {
                    s = child;
                    res = (res << 1) + 1;
            } else if let Some(ref child) = s.children[1] {
                    s = child;
                    res <<= 1;
            }
        }

        res
    }


}



impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {

        let mut res = 0;
        let mut trie = Trie::new();

        for index in 1..nums.len() {

            trie.insert(nums[index-1]);
            res = res.max(trie.check(nums[index]));
        }
        res

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_xor_should_work() {
        let max_xor = Solution::find_maximum_xor(vec![3,10,5,25,2,8]);

        println!("{:?}", max_xor);
    }
}