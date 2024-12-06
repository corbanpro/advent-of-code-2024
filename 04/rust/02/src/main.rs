use std::fs;

fn main() {
    let word_search_raw = fs::read_to_string("../../assets/test.txt").unwrap();
    let mut word_search: Vec<Vec<&str>> = word_search_raw
        .split("\n")
        .map(|row| {
            let mut letters: Vec<&str> = row.split("").collect();
            letters.retain(|letter| letter != &"");
            letters
        })
        .collect();
    word_search.pop();

    let mut result = 0;
    let possible_xmas = ["MMASS", "SSAMM", "SMASM", "MSAMS"];

    // Diagonal down-right
    for row_index in 0..word_search.len() - 2 {
        let row = &word_search[row_index];

        for letter_index in 0..row.len() - 2 {
            let xmas = word_search[row_index][letter_index].to_owned()
                + word_search[row_index][letter_index + 2]
                + word_search[row_index + 1][letter_index + 1]
                + word_search[row_index + 2][letter_index]
                + word_search[row_index + 2][letter_index + 2];
            if possible_xmas.contains(&xmas.as_str()) {
                result += 1
            }
        }
    }

    println!("total: {}", result)
}
