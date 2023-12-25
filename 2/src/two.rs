use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut result: u32 = 0;
    for res_line in read_lines(file_path).unwrap() {
        let line = res_line.unwrap();

        let mut red_req: u8 = 0;
        let mut blue_req: u8 = 0;
        let mut green_req: u8 = 0;
        for subset in line.split(": ").last().unwrap().split("; ") {
            // each piece is like "2 green"
            for piece in subset.split(", ") {
                let mut count: u8 = 0;
                for c in piece.chars() {
                    match c  {
                        '0'..='9' => {
                            count = count * 10 + (c as u8 - '0' as u8);
                        },
                        'r' => {
                            red_req = cmp::max(red_req, count);
                            break;
                        },
                        'g' => {
                            green_req = cmp::max(green_req, count);
                            break;
                        },
                        'b' => {
                            blue_req = cmp::max(blue_req, count);
                            break;
                        },
                        ' ' => (),
                        _ => break
                    }
                }
            }
        }
        result += red_req as u32 * blue_req as u32 * green_req as u32
    }

    println!("{}", result);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}
