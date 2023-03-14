use tmp::codewars::decode_morse;

#[test]
fn decode_morse_should_work() {
    assert_eq!(
        decode_morse::decode_morse(".... . -.--   .--- ..- -.. ."),
        "HEY JUDE"
    );
    println!("-----");
    assert_eq!(decode_morse::decode_morse("....   ...."), "H H");
    // "....   ....".split(" ") result is:
    // "...."
    // ""
    // ""
    // "...."
}
