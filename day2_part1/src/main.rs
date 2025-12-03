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
    println!("{}", s)
}

fn process_range(lower: i64, upper: i64) -> i64 {
    let mut s: i64 = 0;
    for n in lower..upper + 1 {
        if is_doubled(n) {
            s += n;
        }
    }
    return s;
}

fn is_doubled(n: i64) -> bool {
    let str = n.to_string();
    let len = str.len();
    if len % 2 == 1 {
        return false;
    }
    let first_half = &str[0..len / 2];
    let second_half = &str[len / 2..];
    first_half == second_half
}
