use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut result: u32 = 0;
    let mut line_id = 1;
    for res_line in read_lines(file_path).unwrap() {
        let line = res_line.unwrap();
        let results = line.split(": ").last().unwrap();
        
        let mut ok = true;
        'outer: for subset in results.split("; ") {
            for piece in subset.split(", ") {
                let mut count: u8 = 0;
                for c in piece.chars() {
                    match c  {
                        '0'..='9' => {
                            count = count * 10 + (c as u8 - '0' as u8);
                            // early break
                            if count > 14 {
                                ok = false;
                                break 'outer;
                            }
                        },
                        'r' => {
                            if count > 12 {
                                ok = false;
                                break 'outer;
                            }
                            break;
                        },
                        'g' => {
                            if count > 13 {
                                ok = false;
                                break 'outer;
                            }
                            break;
                        },
                        ' ' | ',' => (),
                        _ => break
                    }
                }
            }
        }
        
        // update with line_id
        if ok {
            result += line_id;
        }
        println!("{} : {} : {}", line_id, ok, line);

        line_id += 1;
    }

    println!("{}", result);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}
