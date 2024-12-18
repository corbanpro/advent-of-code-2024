use std::fs;

fn main() {
    let raw_map = fs::read_to_string("../../assets/map.txt").unwrap();
    let map_rows = raw_map.trim().split("\n").filter(|line| !line.is_empty());

    let mut map: Vec<Vec<i32>> = vec![];

    for row in map_rows {
        let mut map_row = vec![];
        for char in row.trim().split("").filter(|char| !char.is_empty()) {
            map_row.push(char.parse().unwrap());
        }
        map.push(map_row)
    }

    let mut trailheads = 0;

    for (y, row) in map.iter().enumerate() {
        let y = y as i32;

        for (x, height) in row.iter().enumerate() {
            let x = x as i32;
            if height == &0 {
                parse_neighbors(x, y, &map, &mut trailheads)
            }
        }
    }

    println!("{:?}", trailheads);
}

fn is_in_map(x: i32, y: i32, map: &[Vec<i32>]) -> bool {
    let map_height = map.len();
    let map_width = map.first().unwrap().len();

    x >= 0 && y >= 0 && x < map_width as i32 && y < map_height as i32
}

fn parse_neighbors(x: i32, y: i32, map: &[Vec<i32>], trailheads: &mut i32) {
    let height = map.get(y as usize).unwrap().get(x as usize).unwrap();
    for x_move in [-1, 0_i32, 1] {
        for y_move in [-1, 0_i32, 1] {
            let dist = x_move + y_move;
            if dist.abs() != 1 {
                continue;
            }
            let neighbor_x = x + x_move;
            let neighbor_y = y + y_move;

            if !is_in_map(neighbor_x, neighbor_y, map) {
                continue;
            }

            let neighbor_h = map.get(neighbor_y as usize).unwrap().get(neighbor_x as usize).unwrap();

            if neighbor_h - height != 1 {
                continue;
            }

            if neighbor_h == &9 {
                *trailheads += 1;
            } else {
                parse_neighbors(neighbor_x, neighbor_y, map, trailheads)
            }
        }
    }
}
