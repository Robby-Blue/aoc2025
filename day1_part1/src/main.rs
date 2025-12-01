use std::fs;

fn main() {
    let mut n = 50;
    let mut i = 0;

    let contents = fs::read_to_string("input.txt").unwrap();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().nth(0).unwrap();
        let num_str = &line.to_string()[1..];
        let num: i32 = num_str.parse().unwrap();

        let multiplier = match direction {
            'R' => 1,
            'L' => -1,
            _ => unreachable!(),
        };

        n += multiplier * num;

        if n < 0 {
            n = 100 + n;
        }
        n = n % 100;
        if n == 0 {
            i += 1
        }
    }
    print!("{}", i);
}
