use std::fs;

fn main() {
    let word_search_raw = fs::read_to_string("../assets/wordSearch.txt").unwrap();
    let mut word_search: Vec<Vec<&str>> = word_search_raw
        .split("\n")
        .map(|row| {
            let mut letters: Vec<&str> = row.split("").collect();
            letters.retain(|letter| letter != &"");
            letters
        })
        .collect();
    word_search.pop();

    let mut forward_back = 0;

    // Forward and backward
    for row in word_search.iter() {
        for letter_index in 0..row.len() - 3 {
            if row[letter_index..letter_index + 4].join("") == "XMAS"
                || row[letter_index..letter_index + 4].join("") == "SAMX"
            {
                forward_back += 1;
            }
        }
    }

    let mut up_down = 0;

    // Up and Down
    for column_index in 0..word_search[0].len() {
        let column: Vec<&str> = word_search.iter().map(|row| row[column_index]).collect();
        for row_index in 0..word_search.len() - 3 {
            if column[row_index..row_index + 4].join("") == "XMAS"
                || column[row_index..row_index + 4].join("") == "SAMX"
            {
                up_down += 1;
            }
        }
    }

    let mut down_right = 0;

    // Diagonal down-right
    for row_index in 0..word_search.len() - 3 {
        let row = &word_search[row_index];
        for letter_index in 0..row.len() - 3 {
            let word = word_search[row_index][letter_index].to_owned()
                + word_search[row_index + 1][letter_index + 1]
                + word_search[row_index + 2][letter_index + 2]
                + word_search[row_index + 3][letter_index + 3];
            if word == "XMAS" || word == "SAMX" {
                down_right += 1;
            }
        }
    }

    let mut down_left = 0;

    // Diagonal down-left
    for row_index in 0..word_search.len() - 3 {
        let row = &word_search[row_index];
        for letter_index in 3..row.len() {
            let word = word_search[row_index][letter_index].to_owned()
                + word_search[row_index + 1][letter_index - 1]
                + word_search[row_index + 2][letter_index - 2]
                + word_search[row_index + 3][letter_index - 3];
            if word == "XMAS" || word == "SAMX" {
                down_left += 1;
            }
        }
    }
    println!(
        "forward and back: {}\nup and down: {}\ndown and left: {}\ndown and right: {}\n\ntotal: {}",
        forward_back,
        up_down,
        down_left,
        down_right,
        forward_back + up_down + down_left + down_right
    )
}
