use std::fs;
use std::thread;
use std::time::Instant;

const BLINKS: i32 = 25;

fn main() {
    let start = Instant::now();

    let raw_stones = fs::read_to_string("../../assets/stones.txt").unwrap();
    let mut stones: Vec<i64> = raw_stones.trim().split(" ").map(|char| char.parse().unwrap()).collect();
    let mut depth = 0;

    while stones.len() < 12 {
        (stones, depth) = expand_stones(stones, depth);
    }

    let (sender, receiver) = std::sync::mpsc::channel();

    let mut handles = vec![];
    let num_start_stones = stones.len();

    for stone in stones {
        let sender = sender.clone();
        let handle = thread::spawn(move || {
            let stones = process_stone(stone, depth);
            sender.send(stones).unwrap();
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let num_stones: i64 = receiver.iter().take(num_start_stones).sum();

    let duration = start.elapsed();

    println!("{num_stones}");
    println!("Time Taken: {}ms", duration.as_micros());
}

fn expand_stones(stones: Vec<i64>, depth: i32) -> (Vec<i64>, i32) {
    let mut new_stones: Vec<i64> = vec![];
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }

        let num_digits = num_digits(stone);

        if num_digits % 2 == 0 {
            let (first_stone, second_stone) = split_number(stone, num_digits);
            new_stones.push(first_stone);
            new_stones.push(second_stone)
        } else {
            new_stones.push(stone * 2024)
        };
    }
    (new_stones, depth + 1)
}

fn process_stone(stone: i64, depth: i32) -> i64 {
    if depth == BLINKS {
        return 1;
    }

    if stone == 0 {
        return process_stone(1, depth + 1);
    }

    let num_digits = num_digits(stone);

    if num_digits % 2 == 0 {
        let (first_stone, second_stone) = split_number(stone, num_digits);
        process_stone(first_stone, depth + 1) + process_stone(second_stone, depth + 1)
    } else {
        process_stone(stone * 2024, depth + 1)
    }
}

fn split_number(num: i64, digits: i64) -> (i64, i64) {
    let half_digits = digits / 2;
    let divisor = 10_i64.pow(half_digits as u32);

    let first_half = num / divisor; // The first half
    let second_half = num % divisor; // The second half

    (first_half, second_half)
}

fn num_digits(num: i64) -> i64 {
    let mut digits = 0;
    let mut temp = num;

    // Count the number of digits
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }
    digits
}
