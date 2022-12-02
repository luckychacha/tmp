use super::Solution;

impl Solution {
    fn happy_string_dfs(memo: &mut Vec<String>, arr: &mut Vec<char>, n: usize) {
        if arr.len() == n {
            memo.push(arr.iter().collect::<String>());
            return;
        }

        for c in ['a', 'b', 'c'] {
            if c == arr[arr.len() - 1] {
                continue;
            }
            arr.push(c);
            Solution::happy_string_dfs(memo, arr, n);
            arr.pop();
        }
    }

    pub fn get_happy_string(n: i32, k: i32) -> String {
        let k = (k - 1) as usize;

        let mut memo: Vec<String> = vec![];
        // let mut arr: Vec<char> = vec![];

        for c in ['a', 'b', 'c'] {
            Solution::happy_string_dfs(&mut memo, &mut vec![c], n as usize);
        }

        if memo.len() <= k {
            return "".to_string();
        }
        memo[k as usize].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_happy_string_should_work() {
        assert_eq!(Solution::get_happy_string(3, 9), String::from("cab"))
    }
}
