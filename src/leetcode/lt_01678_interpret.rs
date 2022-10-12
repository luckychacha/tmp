use super::Solution;

impl Solution {
    pub fn interpret(command: String) -> String {

        (command + " ")
            .as_bytes()
            .windows(2)
            .map(|w| match (w[0], w[1]) {
                (b'G', _) => "G",
                (b'(', b')') => "o",
                (b'a', b'l') => "al",
                _ => "",
            })
            .collect::<String>()

        // let mut iter = command.chars();
        // let mut res = String::new();

        // while let Some(c) = iter.next() {
        //     if c == 'G' {
        //         res.push(c);
        //     } else if iter.next() == Some(')') {
        //             res.push('o');
        //     } else {
        //         res.push_str("al");
        //         iter.next();
        //         iter.next();
        //     }
        // }

        // res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpret_should_work() {
        assert_eq!(
            String::from("alGalooG"),
            Solution::interpret(String::from("(al)G(al)()()G"))
        );
    }
}
