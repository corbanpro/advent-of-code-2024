use rustc_hash::FxHashMap;
use std::fs;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();
    let raw_stones = fs::read_to_string("../../assets/stones.txt").unwrap();
    let mut input = parse_input(&raw_stones);
    println!("{}", solve(&mut input, 200));
    let duration = start.elapsed();
    println!("Time Taken: {}ms", duration.as_millis());
}

fn parse_input(input: &str) -> FxHashMap<u128, u128> {
    input
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn solve(map: &mut FxHashMap<u128, u128>, n_steps: u32) -> u128 {
    (0..n_steps).for_each(|_| {
        map.clone()
            .iter()
            .filter(|(_, &value)| value != 0)
            .for_each(|(&key, &value)| {
                if key == 0 {
                    add(map, 1, value);
                    sub(map, key, value);
                } else {
                    let str_key = key.to_string();
                    if str_key.len() % 2 == 0 {
                        let (key0, key1) = {
                            let key_len_half = str_key.len() / 2;
                            (
                                str_key[..key_len_half].parse().unwrap(),
                                str_key[key_len_half..].parse().unwrap(),
                            )
                        };
                        add(map, key0, value);
                        add(map, key1, value);
                        sub(map, key, value);
                    } else {
                        add(map, 2024 * key, value);
                        sub(map, key, value);
                    }
                }
            });
    });
    map.values().sum()
}

fn add(map: &mut FxHashMap<u128, u128>, key: u128, value: u128) {
    if let Some(key) = map.get_mut(&key) {
        *key += value;
    } else {
        map.insert(key, value);
    }
}

fn sub(map: &mut FxHashMap<u128, u128>, key: u128, value: u128) {
    *map.get_mut(&key).unwrap() -= value;
}
