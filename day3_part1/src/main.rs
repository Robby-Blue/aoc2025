use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;

    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut biggest_first_digit = 0;
        let mut biggest_first_index = 0;

        for (i, char) in line.chars().enumerate() {
            if i + 1 == line.len() {
                continue;
            }
            let n: i32 = char.to_string().parse().unwrap();
            if n > biggest_first_digit {
                biggest_first_digit = n;
                biggest_first_index = i;
            }
        }

        let mut biggest_second_digit = 0;

        for char in line[biggest_first_index + 1..].chars() {
            let n: i32 = char.to_string().parse().unwrap();
            if n > biggest_second_digit {
                biggest_second_digit = n;
            }
        }

        println!("{} {} {}", line, biggest_first_digit, biggest_second_digit);

        total += 10 * biggest_first_digit + biggest_second_digit;
    }

    println!("{}", total);
}
