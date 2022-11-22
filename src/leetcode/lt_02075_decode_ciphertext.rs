use crate::leetcode::Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        if rows == 1 {
            return encoded_text;
        }
        let col_count = encoded_text.len() as i32 / rows;
        let mut new_vec = vec![vec![]; rows as usize];
        let mut res: Vec<char> = Vec::new();
        let _ =
            encoded_text
                .chars()
                .into_iter()
                .fold((0, 0), |(mut tmp_row, mut tmp_col), item| {
                    println!("item:{}", item);
                    if tmp_col >= col_count {
                        tmp_col = 0;
                        tmp_row += 1;
                    }
                    println!("tmp_row: {tmp_row} tmp_col: {tmp_col}");

                    new_vec[tmp_row].push(item);
                    tmp_col += 1;

                    (tmp_row, tmp_col)
                });

        for i in 0..col_count {
            for j in 0..rows as usize {
                let y = i as i32 + j as i32;
                if y >= col_count {
                    continue;
                }
                res.push(new_vec[j][y as usize]);
            }
        }

        res.iter().collect::<String>().trim_end().to_string()
    }
}
