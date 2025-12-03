use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut s: i64 = 0;

    for range in contents.split(",") {
        if range.is_empty() {
            continue;
        }
        let range = range.trim();
        let ids: Vec<&str> = range.split("-").collect();
        println!("{:?}", ids);
        s += process_range(ids[0].parse().unwrap(), ids[1].parse().unwrap());
    }
    println!("{}", s);
}

fn process_range(lower: i64, upper: i64) -> i64 {
    let mut s: i64 = 0;
    for n in lower..upper + 1 {
        if is_multiple(n) {
            s += n;
        }
    }
    return s;
}

fn is_multiple(n: i64) -> bool {
    let str = n.to_string();
    let len = str.len();

    for i in 1..len / 2 + 2 {
        if len % i != 0 {
            continue;
        }
        let first_part = &str[0..i];
        let mut repititions = 0;

        for k in 0..len / i {
            let part = &str[k * i..k * i + i];
            if part == first_part {
                repititions += 1;
            } else {
                repititions = 0;
                break;
            }
        }
        if repititions >= 2 {
            return true;
        }
    }
    return false;
}
