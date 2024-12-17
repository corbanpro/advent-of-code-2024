use std::{collections::HashSet, fmt::Display, fs};
use Dir::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => "Up",
                Right => "Right",
                Down => "Down",
                Left => "Left",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Soldier {
    x: i32,
    y: i32,
    dir: Dir,
}

impl Soldier {
    fn get_next_position(&self) -> (i32, i32) {
        match self.dir {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Right => (self.x + 1, self.y),
            Left => (self.x - 1, self.y),
        }
    }
    fn change_direction(&mut self) {
        match self.dir {
            Up => self.dir = Right,
            Right => self.dir = Down,
            Down => self.dir = Left,
            Left => self.dir = Up,
        }
    }
    fn walk(&mut self) {
        match self.dir {
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
    }
}

fn main() {
    let raw_map = fs::read_to_string("../../assets/map.txt").unwrap();

    let map: Vec<Vec<&str>> = raw_map
        .trim()
        .split("\n")
        .map(|row| {
            let mut row_iter = row.trim().split("").collect::<Vec<&str>>();
            row_iter.retain(|str| str != &"");
            row_iter
        })
        .collect();

    let mut starting_x = 0;
    let mut starting_y = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char == &"^" {
                starting_x = x as i32;
                starting_y = y as i32;
            }
        }
    }

    let (visited_squares, _) = run_simulation(&map, starting_x, starting_y, usize::MAX, usize::MAX);

    let visited_squares = visited_squares
        .into_iter()
        .map(|soldier| (soldier.x, soldier.y))
        .collect::<HashSet<(i32, i32)>>();

    let mut num_loops = 0;

    for square in visited_squares {
        let (_, loops) = run_simulation(&map, starting_x, starting_y, square.0 as usize, square.1 as usize);

        if loops {
            num_loops += 1;
        }
    }

    println!("{num_loops}");
}

fn run_simulation(
    map: &[Vec<&str>],
    starting_x: i32,
    starting_y: i32,
    obstruction_x: usize,
    obstruction_y: usize,
) -> (HashSet<Soldier>, bool) {
    let mut loops = false;
    let mut squares_visited = HashSet::<Soldier>::new();
    let map_width = map[0].len() as i32;
    let map_height = map.len() as i32;

    let mut soldier = Soldier {
        x: starting_x,
        y: starting_y,
        dir: Up,
    };

    let is_in_map = |x: i32, y: i32| -> bool { x >= 0 && x < map_width && y >= 0 && y < map_height };

    if obstruction_x as i32 == soldier.x && obstruction_y as i32 == soldier.y {
        return (squares_visited, false);
    }

    loop {
        if !squares_visited.insert(soldier.clone()) {
            loops = true;
            break;
        }

        let (next_x, next_y) = soldier.get_next_position();

        if !is_in_map(next_x, next_y) {
            break;
        }

        let next_square = map[next_y as usize][next_x as usize];

        if next_square == "#" || (next_x as usize == obstruction_x && next_y as usize == obstruction_y) {
            soldier.change_direction();
        } else {
            soldier.walk();
        }
    }

    (squares_visited, loops)
}
