use std::fs;

pub(crate) fn main() {
    let file_path: &str = "/home/jakob/Desktop/input";

    read_file(file_path)
}

fn read_file(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split_whitespace();
    let binaries: Vec<&str> = split.collect();

    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    let mut x_count;
    let mut y_count;

    let mut idx = 1;

    while idx < 13 {
        x_count = 0;
        y_count = 0;

        for binary in &binaries {
            let number: Vec<&str> = binary.split("").collect();

            if number[idx] == "1" {
                x_count += 1;
            } else {
                y_count += 1;
            }
        }
        idx += 1;

        if x_count > y_count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!(
        "{}",
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
    );
}
