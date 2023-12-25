use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // assumptions:
    // - lines are all the same length
    // - special characters are everything except '0'..='9' and '.'
    // - all ascii!
    // - gears always have exactly two part numbers next to them (this is stated in the problem?)

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut result = 0;

    let mut lines_iter = read_lines(file_path).unwrap();
    let mut opt_prev_line: Option<String> = None;
    let mut opt_curr_line: Option<String> = lines_iter.next().and_then(|result| result.ok());
    let mut opt_next_line: Option<String> = lines_iter.next().and_then(|result| result.ok());

    while let Some(ref curr_line) = opt_curr_line {
        let mut debug_line_total = 0;
        let mut i = 0;
        while i < curr_line.len() {
            let c = curr_line.as_bytes()[i];
            match c {
                b'*' => {
                    let mut gear_ratio: u32 = 1;
                    let mut n_adjacent_numbers = 0;
                    let go_left = i > 0;
                    let go_right = (i+1) < curr_line.len();

                    // match side to side
                    if go_left && is_digit(curr_line.as_bytes()[i-1]) {
                        gear_ratio *= extract_number(curr_line, i-1);
                        n_adjacent_numbers += 1;
                    }
                    if go_right && is_digit(curr_line.as_bytes()[i+1]) {
                        gear_ratio *= extract_number(curr_line, i+1);
                        n_adjacent_numbers += 1;
                    }

                    // up
                    if let Some(ref prev_line) = opt_prev_line {
                        // directly up
                        if is_digit(prev_line.as_bytes()[i]) {
                            gear_ratio *= extract_number(prev_line, i);
                            n_adjacent_numbers += 1;
                        } else {
                            // know the left and right diagonals cannot collide since there
                            // is not a digit directly above!
                            if go_left && is_digit(prev_line.as_bytes()[i-1]) {
                                gear_ratio *= extract_number(prev_line, i-1);
                                n_adjacent_numbers += 1;
                            }
                            if go_right && is_digit(prev_line.as_bytes()[i+1]) {
                                gear_ratio *= extract_number(prev_line, i+1);
                                n_adjacent_numbers += 1;
                            }
                        }
                    }
                    
                    if let Some(ref next_line) = opt_next_line {
                        // directly up
                        if is_digit(next_line.as_bytes()[i]) {
                            gear_ratio *= extract_number(next_line, i);
                            n_adjacent_numbers += 1;
                        } else {
                            // know the left and right diagonals cannot collide since there
                            // is not a digit directly below!
                            if go_left && is_digit(next_line.as_bytes()[i-1]) {
                                gear_ratio *= extract_number(next_line, i-1);
                                n_adjacent_numbers += 1;
                            }
                            if go_right && is_digit(next_line.as_bytes()[i+1]) {
                                gear_ratio *= extract_number(next_line, i+1);
                                n_adjacent_numbers += 1;
                            }
                        }
                    }
                    if n_adjacent_numbers == 2 {
                        debug_line_total += gear_ratio;
                        result += gear_ratio;
                    }
                }
                _ => ()
            }
            i += 1;
        }
        println!("{} {}", format!("{:10}", debug_line_total), curr_line);
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

fn is_digit(byte: u8) -> bool {
    match byte {
        b'0'..=b'9' => return true,
        _ => return false
    }
}

/**
 * Given a string and the index of a digit from a number, extract
 * the entire number
 */
fn extract_number(string: &str, digit_idx: usize) -> u32 {
    let mut start = digit_idx;
    while start > 0 {
        match string.as_bytes()[start-1] {
            b'0'..=b'9' => start -= 1,
            _ => break
        }
    }
    let mut end = digit_idx;
    while end+1 < string.len() {
        match string.as_bytes()[end+1] {
            b'0'..=b'9' => end += 1,
            _ => break
        }
    }

    let mut part_number: u32 = 0;
    for j in start..=end {
        part_number = part_number * 10 + (string.as_bytes()[j] - b'0') as u32;
    }
    part_number
}
