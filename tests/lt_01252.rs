use tmp::leetcode::Solution;

#[test]
fn odd_cells_should_work() {
    let a = 1u64;
    let b = 1u64;
    let c = 0u64;
    println!("a: {:b}", a);
    println!("b: {:b}", b);
    println!("c: {:b}", c);
    println!("a ^ b: {:b}", a ^ b);
    println!("a ^ c: {:b}", a ^ c);
    println!("a ^ 1: {:b}", a ^ 1<<5);
    println!("ones: {}", a.count_ones());
    assert_eq!(
        Solution::odd_cells(2, 3, vec![vec![0,1], vec![1,1]]),
        6
    );
}