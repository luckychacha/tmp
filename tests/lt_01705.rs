use tmp::leetcode::Solution;

// 指定文件进行test：cargo test --test lt_01705
// 输出包含println! 的结果：cargo test --test lt_01705 -- --nocapture

#[test]
fn _01705_eaten_apples_should_work() {
    println!("1");
    assert_eq!(
        Solution::eaten_apples(vec![3,0,0,0,0,2], vec![3,0,0,0,0,2]),
        5
    );
    println!("2");
    assert_eq!(
        Solution::eaten_apples(vec![2,1,10], vec![2,10,1]),
        4
    );
    println!("3");
    assert_eq!(
        Solution::eaten_apples(vec![1,2,3,5,2], vec![3,2,1,4,2]),
        7
    );
    

}