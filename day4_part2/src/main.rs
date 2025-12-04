use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut fields: Vec<Vec<bool>> = vec![];

    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        };
        let mut row: Vec<bool> = vec![];
        for char in line.chars() {
            row.push(char == '@');
        }
        fields.push(row);
    }

    let mut total = 0;

    loop {
        let mut this_loop = 0;
        let mut to_remove: Vec<Vec<usize>> = vec![];

        for y in 0..fields.len() {
            let row = &fields[y];

            for x in 0..row.len() {
                let count = count_rolls(x as i32, y as i32, &fields);
                if is_roll(x as i32, y as i32, &fields) && count < 4 {
                    total += 1;
                    this_loop += 1;
                    to_remove.push(vec![x, y]);
                }
            }
        }

        for roll in to_remove {
            fields[roll[1]][roll[0]] = false;
        }

        if this_loop == 0 {
            break;
        }
        println!("{}", this_loop);
    }
    println!("{}", total);
}

fn count_rolls(x: i32, y: i32, fields: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for offset_x in -1..2 {
        for offset_y in -1..2 {
            if offset_x == 0 && offset_y == 0 {
                continue;
            }
            if is_roll(x + offset_x, y + offset_y, &fields) {
                count += 1;
            }
        }
    }
    count
}
fn is_roll(x: i32, y: i32, fields: &Vec<Vec<bool>>) -> bool {
    if x < 0 || y < 0 || y >= fields.len() as i32 {
        return false;
    }
    let row = &fields[y as usize];
    if x >= row.len() as i32 {
        return false;
    }
    return row[x as usize];
}
