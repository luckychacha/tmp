pub fn to_camel_case(text: &str) -> String {
    if text.is_empty() {
        return text.to_owned();
    }
    text.split(&['-', '_'])
        .enumerate()
        .map(|(idx, item)| match idx {
            0 => item.to_owned(),
            _ => item[..1].to_ascii_uppercase() + &item[1..],
        })
        .collect()
}
