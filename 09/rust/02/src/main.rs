use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Debug)]
struct Map {
    antennas: HashMap<String, Vec<Position>>,
    antinodes: HashSet<Position>,
}

impl Map {
    fn new() -> Map {
        Map {
            antennas: HashMap::new(),
            antinodes: HashSet::new(),
        }
    }
}

fn main() {
    let raw_map = fs::read_to_string("../../assets/antennas.txt").unwrap();
    let mut map = Map::new();
    let mut map_width = 0;
    let mut map_height = 0;
    for (y, row) in raw_map.trim().split("\n").enumerate() {
        map_height = y;

        for (x, char) in row.trim().split("").filter(|char| !char.is_empty()).enumerate() {
            map_width = x;
            //print!("'{char}' ");
            if char != "." {
                let entry = map.antennas.entry(char.to_string()).or_default();
                entry.push(Position::new(x, y))
            }
        }
        //println!();
    }
    //println!("{:#?}", map.antennas);

    println!("{map_width} {map_height}");

    let is_in_map = |position: &Position| -> bool {
        position.x >= 0 && position.x <= map_width as i32 && position.y >= 0 && position.y <= map_height as i32
    };

    for antenna_group in map.antennas.values() {
        for source_antenna in antenna_group.iter() {
            for target_antenna in antenna_group.iter() {
                if source_antenna == target_antenna {
                    continue;
                }
                let antinode_positions = antinode_positions(source_antenna, target_antenna, is_in_map);
                for antinode_position in antinode_positions {
                    map.antinodes.insert(antinode_position);
                }
            }
        }
    }

    //for antinode in map.antinodes.iter() {
    //    println!("({},{})", antinode.x, antinode.y);
    //}

    let mut display_map: Vec<Vec<String>> = vec![];

    for _y in 0..map_height + 1 {
        display_map.push(vec![]);
        for _x in 0..map_width + 1 {
            display_map.last_mut().unwrap().push(".".to_owned());
        }
    }

    for (antenna_char, antenna_group) in map.antennas {
        for antenna in antenna_group {
            let char = display_map
                .get_mut(antenna.y as usize)
                .unwrap()
                .get_mut(antenna.x as usize)
                .unwrap();
            *char = antenna_char.clone();
        }
    }

    for antinode in map.antinodes.iter() {
        let char = display_map
            .get_mut(antinode.y as usize)
            .unwrap()
            .get_mut(antinode.x as usize)
            .unwrap();
        *char = "#".to_owned();
    }

    for row in display_map {
        for char in row {
            print!("{char}")
        }
        println!()
    }

    println!("{}", map.antinodes.len());
}

fn antinode_positions(first: &Position, second: &Position, is_in_map: impl Fn(&Position) -> bool) -> Vec<Position> {
    let mut antinode_positions = vec![];
    let mut loop_count = 0;
    loop {
        let x_diff = (first.x - second.x) * loop_count;
        let y_diff = (first.y - second.y) * loop_count;

        let new_x = first.x + x_diff;
        let new_y = first.y + y_diff;

        let new_pos = Position { x: new_x, y: new_y };
        if is_in_map(&new_pos) {
            antinode_positions.push(new_pos)
        } else {
            break;
        }
        loop_count += 1;
    }

    loop_count = 0;

    loop {
        let x_diff = (second.x - first.x) * loop_count;
        let y_diff = (second.y - first.y) * loop_count;

        let new_x = second.x + x_diff;
        let new_y = second.y + y_diff;

        let new_pos = Position { x: new_x, y: new_y };
        if is_in_map(&new_pos) {
            antinode_positions.push(new_pos)
        } else {
            break;
        }
        loop_count += 1;
    }

    antinode_positions

    //let new_x = (first.x * 2) - second.x;
    //let new_y = (first.y * 2) - second.y;
    //
    //let new_pos = Position { x: new_x, y: new_y };
    ////println!(
    ////    "({},{}) ({},{}) ({},{})",
    ////    first.x, first.y, second.x, second.y, new_pos.x, new_pos.y
    ////);
    //new_pos
}
