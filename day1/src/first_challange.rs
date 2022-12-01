use std::{fs::{File}, io::{BufReader, BufRead}, path::Path};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn find_most_calories() -> i32{
    let lines = lines_from_file("./src/input.txt");

    let mut current: i32 = 0;
    let mut highest: i32 = 0;

    for line in lines {
        if line != ""{
            current += line.parse::<i32>().unwrap();
        }else{
            if current > highest {
                highest = current;
            }
            current = 0;
        }
    }

    highest
}