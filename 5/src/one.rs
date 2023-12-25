use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut lines_iter = read_lines(file_path).unwrap();
    let ranges_iter = line_to_vec(lines_iter.next().unwrap().unwrap().split(": ").last().unwrap());   
    let mut values = Vec::new();
    for i in (0..ranges_iter.len()).step_by(2) {
        let start = ranges_iter[i];
        let range_size = ranges_iter[i+1];
        for value in start..(start+range_size) {
            values.push(value);
        }
    }
    lines_iter.nth(1);

    println!("{:?}", values);
    
    for _i in 0..7 {
        let mut map: Vec<Vec<u64>> = Vec::new();
        while let Some(line) = lines_iter.next().and_then(|result| result.ok()) {
            println!("{}", line);
            if line == "" {
                break;
            }
            map.push(line_to_vec(&line));
        }
        let mut new_values = Vec::with_capacity(values.len());
        
        'outer: for value in values {
            for mapping in &map {
                if mapping[1] <= value && (mapping[1] + mapping[2]) >= value {
                    new_values.push(mapping[0] + (value - mapping[1]));
                    continue 'outer;
                }
            }
            new_values.push(value);
        }
        values = new_values;
        println!("{:?}", values);
        lines_iter.next();
    }

    println!("{}", values.iter().min().unwrap());
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}

fn line_to_vec(line: &str) -> Vec<u64> { 
    let mut seeds = Vec::new();
    for num_str in line.split(" ") {
        let mut value: u64 = 0;
        for c in num_str.bytes() {
            match c {
                b'0'..=b'9' => value = value * 10 + (c - b'0') as u64,
                _ => ()
            }
        }
        seeds.push(value);
    }
    seeds
}
