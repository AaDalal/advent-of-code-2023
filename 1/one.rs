use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut result: u32 = 0;
    for line in read_lines(file_path).unwrap() {
        let mut first: char = 'a'; // 'a' is just a placeholder
        let mut last: char = 'a';
        for c in line.unwrap().chars() {
            if '0' <= c && c <= '9' {
                if first == 'a' {
                    first = c;
                    last = c;
                } else {
                    last = c;
                }
            }
        }

        // assumes first and last are properly defined!
        if first == 'a' {
            println!("first is still 'a'!")
        }
        let value: u32 = (first as u32 - '0' as u32) * 10 + (last as u32 - '0' as u32);
        println!("{}", value);
        result += value;
    }

    println!("{}", result);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?; 
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}
