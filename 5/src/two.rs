use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut lines_iter = read_lines(file_path).unwrap();
    
    let mut first = 0;
    let mut is_first = true;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for num in lines_iter.next().unwrap().unwrap().split(": ").last().unwrap().split(" ") {
        let value = str_to_u64(&num);
        if is_first {
            first = value;
        } else {
            ranges.push((first, value));
        }
        is_first = !is_first;
    }
    println!("initial ranges {:?}", ranges);
    lines_iter.nth(1);

    for _i in 0..7 {
        let mut map: Vec<Vec<u64>> = Vec::new();
        while let Some(line) = lines_iter.next().and_then(|result| result.ok()) {
            // // println!("{}", line);
            if line == "" {
                break;
            }
            map.push(line_to_vec(&line, 3));
        }
        
        let mut new_ranges: Vec<(u64, u64)> = Vec::new();
        'outer: while ranges.len() > 0 {
            // // println!("in loop");
            let range = ranges.pop().unwrap();
            for mapping in &map {
                if let Some(new_range) = range_overlap(range, (mapping[0], mapping[1], mapping[2]), &mut ranges) {
                    println!("{:?}", new_range);
                    new_ranges.push(new_range);
                    continue 'outer;
                }
            }
            new_ranges.push(range);
        }
        ranges = new_ranges;
        lines_iter.next();
        println!("{:?}", ranges);
    }

    println!("{}", ranges.iter().min().unwrap().0);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}

fn str_to_u64(string: &str) -> u64 {
    let mut value: u64 = 0;
    for c in string.bytes() {
        match c {
            b'0'..=b'9' => value = value * 10 + (c - b'0') as u64,
            _ => ()
        }
    }
    return value;
}

fn line_to_vec(line: &str, capacity: usize) -> Vec<u64> { 
    let mut vec = Vec::with_capacity(capacity);
    for num_str in line.split(" ") {
        vec.push(str_to_u64(num_str));
    }
    vec
}

fn range_overlap(in_range: (u64, u64), out_range: (u64, u64, u64), ranges: &mut Vec<(u64, u64)>) -> Option<(u64, u64)> {
    let start = cmp::max(in_range.0, out_range.1);
    let end = cmp::min(in_range.0 + in_range.1, out_range.1 + out_range.2); // non-inclusive 
    // no overlap
    if start >= end {
        return None;
    }

    if in_range.0 < start {
        // println!("push {:?}", (in_range.0, start - in_range.0));
        ranges.push((in_range.0, start - in_range.0));
    }
    if (in_range.0 + in_range.1) > end {
        // println!("push {:?}", (end, in_range.0 + in_range.1 - end));
        ranges.push((end, in_range.0 + in_range.1 - end));
    }
    println!("in {:?} out {:?} start {} end {}", in_range, out_range, start, end);
    return Some((start - out_range.1 + out_range.0, end - start));
}
