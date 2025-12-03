use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total: i64 = 0;

    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        let biggest_str = find_biggest(line, 11, 0, line.len()).unwrap();
        println!("{}", biggest_str);
        total += biggest_str.parse::<i64>().unwrap();
    }

    println!("{}", total);
}

fn find_biggest(line: &str, remaining: i64, start: usize, end: usize) -> Option<String> {
    let mut biggest_num: Option<i32> = None;
    let mut biggest_index: Option<usize> = None;

    for (i, char) in line[start..end].chars().enumerate() {
        let num: i32 = char.to_string().parse().unwrap();
        let is_biggest = match biggest_num {
            None => true,
            Some(biggest_num) => num > biggest_num,
        };
        if is_biggest {
            biggest_num = Some(num);
            biggest_index = Some(i);
        }
    }

    if matches!(biggest_num, None) {
        return None;
    }

    let biggest_num = biggest_num.unwrap();
    let biggest_index = biggest_index.unwrap() + start;

    if remaining >= 1 {
        let biggest_following = find_biggest(line, remaining - 1, biggest_index + 1, line.len());
        match biggest_following {
            None => {
                return find_biggest(line, remaining, start, biggest_index);
            }
            Some(biggest) => {
                return Some(biggest_num.to_string() + &biggest);
            }
        }
    }
    return Some(biggest_num.to_string());
}
