use tmp::codewars::to_camel_case;
const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn dotest(s: &str, expected: &str) {
    assert_eq!(
        to_camel_case::to_camel_case(s),
        expected,
        "{ERR_MSG} with text = \"{s}\""
    )
}
#[test]
fn fixed_tests() {
    dotest("", "");
    dotest("the_stealth_warrior", "theStealthWarrior");
    dotest("The-Stealth-Warrior", "TheStealthWarrior");
    dotest("A-B-C", "ABC");
}
