use std::fs;

fn main() {
    let raw_files = fs::read_to_string("../../assets/files.txt").unwrap();
    let mut files = raw_files
        .trim()
        .split("")
        .filter(|char| !char.is_empty())
        .map(|char| char.parse::<i32>().unwrap());

    let mut next_filelength = files.next();
    let mut next_freespace = files.next();
    let mut disk: Vec<Option<i32>> = vec![];
    let mut file_id = 0;

    while next_filelength.is_some() {
        for _ in 0..next_filelength.unwrap() {
            disk.push(Some(file_id))
        }
        next_filelength = files.next();

        if next_freespace.is_some() {
            for _ in 0..next_freespace.unwrap() {
                disk.push(None)
            }
            next_freespace = files.next();
        }

        file_id += 1;
    }

    let mut compressed_disk: Vec<i32> = vec![];

    loop {
        if disk.is_empty() {
            break;
        }

        let first_val = disk.remove(0);

        let next_val = match first_val {
            Some(file_id) => file_id,

            None => {
                let mut last_val = None;

                while last_val.is_none() {
                    let last_val_res = disk.pop();
                    if last_val_res.is_none() {
                        break;
                    }
                    last_val = last_val_res.unwrap()
                }
                last_val.unwrap()
            }
        };
        compressed_disk.push(next_val)
    }

    let mut checksum = 0;
    for (i, file_id) in compressed_disk.into_iter().enumerate() {
        checksum += i * file_id as usize
    }

    println!("{checksum}")
}
