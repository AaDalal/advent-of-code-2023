use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // assumptions:
    // - lines are all the same length
    // - special characters are everything except '0'..='9' and '.'
    // - all ascii!

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut result = 0;

    let mut lines_iter = read_lines(file_path).unwrap();
    let mut opt_prev_line: Option<String> = None;
    let mut opt_curr_line: Option<String> = lines_iter.next().and_then(|result| result.ok());
    let mut opt_next_line: Option<String> = lines_iter.next().and_then(|result| result.ok());

    while let Some(ref curr_line) = opt_curr_line {
        let mut i = 0;
        while i < curr_line.len() {
            let c = curr_line.as_bytes()[i];
            match c {
                b'0'..=b'9' => {
                    let mut is_part_number = false;
                    let go_left = i > 0;
                    let go_right = (i+1) < curr_line.len();

                    // match side to side
                    is_part_number = is_part_number || go_left && is_special_char(curr_line.as_bytes()[i-1]);
                    is_part_number = is_part_number || go_right && is_special_char(curr_line.as_bytes()[i+1]);

                    if let Some(ref prev_line) = opt_prev_line {
                        // up
                        is_part_number = is_part_number || is_special_char(prev_line.as_bytes()[i]);
                        // up diagonal
                        is_part_number = is_part_number || go_left && is_special_char(prev_line.as_bytes()[i-1]);
                        is_part_number = is_part_number || go_right && is_special_char(prev_line.as_bytes()[i+1]);
                    }
                    
                    if let Some(ref next_line) = opt_next_line {
                        // down
                        is_part_number = is_part_number || is_special_char(next_line.as_bytes()[i]);
                        // down diagonal
                        is_part_number = is_part_number || go_left && is_special_char(next_line.as_bytes()[i-1]);
                        is_part_number = is_part_number || go_right && is_special_char(next_line.as_bytes()[i+1]);
                    }

                    if is_part_number {
                        // extract part number
                        let mut start = i;
                        while start > 0 {
                            match curr_line.as_bytes()[start-1] {
                                b'0'..=b'9' => start -= 1,
                                _ => break
                            }
                        }
                        let mut end = i;
                        while end+1 < curr_line.len() {
                            match curr_line.as_bytes()[end+1] {
                                b'0'..=b'9' => end += 1,
                                _ => break
                            }
                        }

                        let mut part_number: u32 = 0;
                        for j in start..=end {
                            part_number = part_number * 10 + (curr_line.as_bytes()[j] - b'0') as u32;
                        }
                        result += part_number;

                        // and move right to the first character not in this part number
                        i = end;
                    }
                }
                _ => ()
            }
            i += 1;
        }
        opt_prev_line = opt_curr_line;
        opt_curr_line = opt_next_line;
        opt_next_line = lines_iter.next().and_then(|result| result.ok());
    }
    println!("{}", result);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}

fn is_special_char(byte: u8) -> bool {
    match byte {
        b'0'..=b'9' | b'.' => return false,
        _ => return true
    }
}
