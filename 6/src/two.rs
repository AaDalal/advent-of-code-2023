use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut lines_iter = read_lines(file_path).unwrap();
    // read in line
    let mut times = Vec::new();
    let mut value: u64 = 0;
    for c in lines_iter.next().unwrap().unwrap().bytes().skip(9) {
        match c {
            b'0'..=b'9' => value = value * 10 + (c - b'0') as u64,
            _ => ()
        }
    }
    times.push(value);
    
    let mut distances = Vec::new();
    let mut value: u64 = 0;
    for c in lines_iter.next().unwrap().unwrap().bytes().skip(9) {
        match c {
            b'0'..=b'9' => value = value * 10 + (c - b'0') as u64,
            _ => () 
        }
    }
    distances.push(value);

    println!("{:?} {:?}", times, distances);

    let mut result = 1;
    for i in 0..times.len() {
        let (root1, root2) = solve(times[i], distances[i]);
        println!("{} {}", root1, root2);
        result *= (root2 - root1 + 1);
    }

    println!("result {}", result);

}

fn quadratic_eqn(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sqrt_discrim = f64::sqrt(b * b - 4.0 * a * c);
    return ((-b + sqrt_discrim) / (2.0 * a), (-b - sqrt_discrim) / (2.0 * a)); 
}

fn solve(time: u64, max_score: u64) -> (u64, u64) {
    // x = amount of time on button
    // x * (time - x) > max_score
    // -x * x + x * time - max_score > 0
    // a = -1, b = time, c = -max_score
    let (root1, root2) = quadratic_eqn(-1.0, time as f64, -(max_score as f64));
    // get next integer greater than and less than the roots, respectively
    println!("roots {} {}", root1, root2);
    return ((root1 + 1.0).floor() as u64, (root2 - 1.0).ceil() as u64);
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
