use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Plot {
    perimeter: i32,
    members: HashSet<(i32, i32)>,
}

fn main() {
    let raw_map = fs::read_to_string("../../assets/map.txt").unwrap();
    let map: Vec<Vec<&str>> = raw_map
        .trim()
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.split("").filter(|char| !char.is_empty()).collect())
        .collect();
    let mut plots: Vec<Plot> = vec![];

    let mut visited_letters: HashSet<(i32, i32)> = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if !visited_letters.insert((y as i32, x as i32)) {
                continue;
            }

            let (perimeter, members) = get_plot(&map, char, y as i32, x as i32);

            for plot in members.iter() {
                visited_letters.insert(*plot);
            }

            let new_plot = Plot { members, perimeter };

            plots.push(new_plot);
        }
    }

    let mut total_cost = 0;

    for plot in plots {
        total_cost += plot.perimeter * plot.members.len() as i32
    }

    println!("Total Cost: {}", total_cost)
}

fn is_in_map(map: &[Vec<&str>], y: i32, x: i32) -> bool {
    x >= 0 && y >= 0 && y < map.len() as i32 && x < map[y as usize].len() as i32
}

fn get_plot(map: &[Vec<&str>], char: &str, y: i32, x: i32) -> (i32, HashSet<(i32, i32)>) {
    let mut visited_letters: HashSet<(i32, i32)> = HashSet::from([(y, x)]);
    let mut perimeter = 0;

    find_neighbors(map, char, y, x, &mut perimeter, &mut visited_letters);

    (perimeter, visited_letters)
}

fn find_neighbors(
    map: &[Vec<&str>],
    char: &str,
    y: i32,
    x: i32,
    perimeter: &mut i32,
    visited_letters: &mut HashSet<(i32, i32)>,
) -> i32 {
    let neighbors = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    for (y_diff, x_diff) in neighbors {
        if !is_in_map(map, y + y_diff, x + x_diff) {
            *perimeter += 1;
            continue;
        }

        let neighbor_y = y + y_diff;
        let neighbor_x = x + x_diff;
        let neighbor_char = map[neighbor_y as usize][neighbor_x as usize];

        if neighbor_char == char {
            if !visited_letters.insert((neighbor_y, neighbor_x)) {
                continue;
            }
            find_neighbors(map, char, neighbor_y, neighbor_x, perimeter, visited_letters);
        } else {
            *perimeter += 1;
        }
    }

    *perimeter
}
