use tmp::leetcode::Solution;

#[test]
fn _02075_decode_ciphertext_should_work() {
    assert_eq!(
        Solution::decode_ciphertext(String::from("iveo    eed   l te   olc"), 4),
        String::from("i love leetcode")
    );
}
