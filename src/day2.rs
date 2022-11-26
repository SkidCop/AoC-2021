use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub(crate) fn main() {
    let file_path: &str = "/home/jakob/Desktop/input";

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let split = line.split(" ");
            let vec: Vec<&str> = split.collect();

            for (mut idx, &cmd) in vec.iter().enumerate() {
                if cmd == "forward" {
                    x += vec[idx + 1].parse::<i32>().unwrap();
                    y += (aim * vec[idx + 1].parse::<i32>().unwrap());
                } else if cmd == "down" {
                    aim += vec[idx + 1].parse::<i32>().unwrap();
                } else if cmd == "up" {
                    aim -= vec[idx + 1].parse::<i32>().unwrap();
                }
                idx += 1;
            }
        }
        println!("{}", x * y)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
