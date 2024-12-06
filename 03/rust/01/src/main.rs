use regex::Regex;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("../../assets/mul.txt").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let num_regex = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;
    for mul in mul_regex.find_iter(&instructions) {
        let mut nums = num_regex.find_iter(mul.into());
        sum += nums.next().unwrap().as_str().parse::<i32>().unwrap()
            * nums.next().unwrap().as_str().parse::<i32>().unwrap()
    }
    println!("{}", sum)
}
