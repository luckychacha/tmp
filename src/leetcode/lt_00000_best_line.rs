// https://leetcode.cn/problems/best-line-lcci/
// 面试题 16.14. 最佳直线
use super::Solution;

impl Solution {
    pub fn best_line(points: Vec<Vec<i32>>) -> Vec<i32> {
        let n = points.len();
        let mut ans = vec![0; 2];
        let mut max_points = 0;

        let mut tmp = 0;

        for i in 0..n {
            for j in i+1..n {
                tmp = 2;

                let rest_number = n - j - 1;
                if max_points >= tmp + rest_number {
                    break;
                }

                // 计算斜率 delta_y / delta_x
                let delta_x_ij = points[j][0] - points[i][0];
                let delta_y_ij = points[j][1] - points[i][1];

                for k in j+1..n {
                    let delta_x_ik = points[k][0] - points[i][0];
                    let delta_y_ik = points[k][1] - points[i][1];

                    if delta_x_ij * delta_y_ik == delta_x_ik * delta_y_ij {
                        tmp += 1;
                    }
                }
                if tmp > max_points {
                    max_points = tmp;
                    ans[0] = i as i32;
                    ans[1] = j as i32;
                }
            }
        }

        ans
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn best_line_should_work() {
        assert_eq!(
            Solution::best_line(
                vec![vec![0,0],vec![1,1],vec![1,0],vec![2,0]]
            ),
            vec![0, 2]
        )
    }
}