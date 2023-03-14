use std::collections::HashMap;

pub fn decode_morse(encoded: &str) -> String {
    let morse = morse_init();
    encoded
        .trim()
        .split(" ")
        .map(|letter_morse| {
            println!("{letter_morse:?}");
            if let Some(letter) = morse.get(letter_morse) {
                letter
            } else {
                " "
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// .map(|letter_morse| match letter_morse {
//     ".-" => "A",
//     "-..." => "B",
//     "-.-." => "C",
//     "-.." => "D",
//     "." => "E",
//     "..-." => "F",
//     "--." => "G",
//     "...." => "H",
//     ".." => "I",
//     ".---" => "J",
//     "-.-" => "K",
//     ".-.." => "L",
//     "--" => "M",
//     "-." => "N",
//     "---" => "O",
//     ".--." => "P",
//     "--.-" => "Q",
//     ".-." => "R",
//     "..." => "S",
//     "-" => "T",
//     "..-" => "U",
//     "...-" => "V",
//     ".--" => "W",
//     "-..-" => "X",
//     "-.--" => "Y",
//     "--.." => "Z",
//     ".----" => "1",
//     "..---" => "2",
//     "...--" => "3",
//     "....-" => "4",
//     "....." => "5",
//     "-...." => "6",
//     "--..." => "7",
//     "---.." => "8",
//     "----." => "9",
//     "-----" => "0",
//     _ => " ",
// })

fn morse_init() -> HashMap<String, String> {
    let mut morse: HashMap<String, String> = HashMap::new();
    morse.insert("....".to_string(), "H".to_string());
    morse.insert(".".to_string(), "E".to_string());
    morse.insert(".---".to_string(), "J".to_string());
    morse.insert("--".to_string(), "M".to_string());
    morse.insert(".--.".to_string(), "P".to_string());
    morse.insert("-".to_string(), "T".to_string());
    morse.insert("..-".to_string(), "U".to_string());
    morse.insert(".--".to_string(), "W".to_string());
    morse.insert("-.--".to_string(), "Y".to_string());
    morse.insert("-...".to_string(), "B".to_string());
    morse.insert("-.-".to_string(), "K".to_string());
    morse.insert("--..".to_string(), "Z".to_string());
    morse.insert("-..-".to_string(), "X".to_string());
    morse.insert(".----".to_string(), "1".to_string());
    morse.insert(".-.-.-".to_string(), ".".to_string());
    morse.insert("..--..".to_string(), "?".to_string());
    morse.insert("..".to_string(), "I".to_string());
    morse.insert(".-.".to_string(), "R".to_string());
    morse.insert("-----".to_string(), "0".to_string());
    morse.insert("-....-".to_string(), "-".to_string());
    morse.insert(".--.-.".to_string(), "@".to_string());
    morse.insert("...---...".to_string(), "SOS".to_string());
    morse.insert("....-".to_string(), "4".to_string());
    morse.insert("-.-.--".to_string(), "!".to_string());
    morse.insert(".----.".to_string(), "'".to_string());
    morse.insert("---...".to_string(), ".to_string(),".to_string());
    morse.insert("-....".to_string(), "6".to_string());
    morse.insert("..---".to_string(), "2".to_string());
    morse.insert("-.--.".to_string(), "morse.insert(".to_string());
    morse.insert(".-..".to_string(), "L".to_string());
    morse.insert("..-.".to_string(), "F".to_string());
    morse.insert(".....".to_string(), "5".to_string());
    morse.insert("--..--".to_string(), ".to_string(),".to_string());
    morse.insert(".-".to_string(), "A".to_string());
    morse.insert("-.-.".to_string(), "C".to_string());
    morse.insert("--.-".to_string(), "Q".to_string());
    morse.insert("---".to_string(), "O".to_string());
    morse.insert("-.--.-".to_string(), ")".to_string());
    morse.insert("--.".to_string(), "G".to_string());
    morse.insert("-.-.-.".to_string(), ";".to_string());
    morse.insert("-..".to_string(), "D".to_string());
    morse.insert("...-".to_string(), "V".to_string());
    morse.insert("-...-".to_string(), "=".to_string());
    morse.insert(".-.-.".to_string(), "+".to_string());
    morse.insert(".-..-.".to_string(), "\"".to_string());
    morse.insert("...-..-".to_string(), "$".to_string());
    morse.insert("----.".to_string(), "9".to_string());
    morse.insert(".-...".to_string(), "&".to_string());
    morse.insert("...--".to_string(), "3".to_string());
    morse.insert("---..".to_string(), "8".to_string());
    morse.insert("..--.-".to_string(), "_".to_string());
    morse.insert("--...".to_string(), "7".to_string());
    morse.insert("-.".to_string(), "N".to_string());
    morse.insert("...".to_string(), "S".to_string());
    morse.insert("-..-.".to_string(), "/".to_string());
    morse
}
