use std::fs;

pub(crate) fn main() {
    let file_path: &str = "/home/jakob/Desktop/input";

    let numbers = read_file(file_path);
    let count = compare_ints(numbers);

    println!("Increased Count: {}", count)
}

fn read_file(file_path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    numbers
}

fn compare_ints(numbers: Vec<i32>) -> i32 {
    let mut count = 0;

    let mut x = 0;
    let mut y = 0;

    for (mut idx, number) in numbers.iter().enumerate() {
        if idx + 3 < numbers.len() {
            x = number + numbers[idx + 1] + numbers[idx + 2];
            y = numbers[idx + 1] + numbers[idx + 2] + numbers[idx + 3];

            if y > x {
                count += 1;
            }
        }
        idx += 1;
    }
    count
}
