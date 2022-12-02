use super::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut res = vec![0; 0];

        for i in operations {
            match i.parse::<i32>() {
                Ok(number) => {
                    res.push(number);
                }
                Err(e) => {
                    let n = res.len();
                    match i.as_str() {
                        "+" => res.push(res[n - 1] + res[n - 2]),
                        "D" => {
                            res.push(res[n - 1] * 2);
                        }
                        "C" => {
                            res.pop();
                        }
                        _ => {}
                    }
                }
            }
        }

        res.iter().sum()
        // let mut res: VecDeque<i32> = VecDeque::new();

        // operations.into_iter().enumerate().for_each(|(idx, op)| {
        //     match op.as_str() {
        //         "+" => res.push_back(
        //             res.get(res.len() - 1).unwrap_or_else(|| &0)
        //                 + res.get(res.len() - 2).unwrap_or_else(|| &0),
        //         ),
        //         "D" => res.push_back(res.back().unwrap_or_else(|| &0) * 2),
        //         "C" => {
        //             res.pop_back();
        //         }
        //         _ => res.push_back(op.parse::<i32>().unwrap()),
        //     };
        //     println!("{idx} {:?}", res);
        // });

        // res.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cal_points_should_work() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        )
    }
}
