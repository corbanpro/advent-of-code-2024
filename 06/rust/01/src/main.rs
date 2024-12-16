use std::{collections::HashSet, fs};
use Direction::*;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn get_next_position(&self) -> (i32, i32) {
        match self.direction {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Right => (self.x + 1, self.y),
            Left => (self.x - 1, self.y),
        }
    }
    fn change_direction(&mut self) {
        match self.direction {
            Up => self.direction = Right,
            Right => self.direction = Down,
            Down => self.direction = Left,
            Left => self.direction = Up,
        }
    }
    fn move_soldier(&mut self) {
        match self.direction {
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
    let mut soldier_position = Position {
        x: 0,
        y: 0,
        direction: Up,
    };
    let map_width = map[0].len() as i32;
    let map_height = map.len() as i32;
    let mut squares_visited = HashSet::<String>::new();

    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            print!("'{char}', ");
            if char == &"^" {
                soldier_position.y = y as i32;
                soldier_position.x = x as i32;
            }
        }
        println!();
    }

    let is_in_map = |x: i32, y: i32| -> bool { x >= 0 && x < map_width && y >= 0 && y < map_height };

    loop {
        squares_visited.insert(soldier_position.x.to_string() + "," + &soldier_position.y.to_string());
        println!("soldier position: ({},{})", soldier_position.x, soldier_position.y);

        let (next_x, next_y) = soldier_position.get_next_position();

        if !is_in_map(next_x, next_y) {
            break;
        }

        let next_square = map.get(next_y as usize).unwrap().get(next_x as usize).unwrap();
        println!("next square: ({},{}): {}\n", next_x, next_y, next_square);

        if next_square == &"#" {
            soldier_position.change_direction()
        }

        soldier_position.move_soldier();
    }

    println!("{}", squares_visited.len())
}
