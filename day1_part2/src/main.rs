use std::fs;

fn main() {
    let mut n = 50;
    let mut i = 0;

    let contents = fs::read_to_string("input.txt").unwrap();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let cn = n;

        let direction = line.chars().nth(0).unwrap();
        let num_str = &line.to_string()[1..];
        let num: i32 = num_str.parse().unwrap();

        match direction {
            'R' => {
                n += num;
                i += (n as f32 / 100.0).floor() as i32;
                n = n % 100;
            }
            'L' => {
                if n == 0 {
                    i -= 1
                }
                n -= num;

                if n < 0 {
                    i += (-n as f32 / 100.0).ceil() as i32;
                    // n = 100 - ((n * -1) % 100);
                    while n < 0 {
                        n += 100
                    }
                }

                if n == 0 {
                    i += 1
                }
            }
            _ => unreachable!(),
        };
        if i < 100 {
            println!("{} {} -> {} {}", line, cn, n, i);
        }
    }
    println!("{}", i);
}
