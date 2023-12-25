use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut result: u32 = 0;
    for res_line in read_lines(file_path).unwrap() {
        let line = res_line.unwrap();
        println!("{}", line);
        let mut group_iter = line.split(": ").last().unwrap().split(" | ");
        let winning_nums = group_iter.next().unwrap(); // cheating, but know 10 winning numbers
        let our_nums = group_iter.next().unwrap(); // cheating, but know 10 winning numbers
        
        // bit vector to store results
        let mut winning_bv: u128 = 0;
        {
            let mut value = 0;
            for (i, c) in winning_nums.bytes().enumerate() {
                value = value * 10 + match c {
                    b'0'..=b'9' => c - b'0',
                    _ => 0
                };
                if (i % 3) == 1 {
                    winning_bv |= 1 << value;
                    value = 0;
                }
            }
        }

        // iterate through our numbers
        let mut hit_count = 0;
        {
            let mut value = 0;
            for (i, c) in our_nums.bytes().enumerate() {
                value = value * 10 + match c {
                    b'0'..=b'9' => c - b'0',
                    _ => 0
                };
                if (i % 3) == 1 {
                    if (winning_bv & 1 << value) != 0 {
                        hit_count += 1;
                    }
                    value = 0;
                }
            }
        }

        if hit_count > 0 {
            result += 1 << (hit_count - 1);
        }
    }

    println!("{}", result);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}
