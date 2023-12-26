use std::cmp::Ordering;
use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut hands: Vec<(Vec<u8>, HandType, u64)> = Vec::new();
    for res_line in read_lines(file_path).unwrap() {
        let line = res_line.unwrap();
        let mut splits = line.split(" ");
        let mut hand: Vec<u8> = Vec::with_capacity(5);
        let mut freqs: [u8; 13] = [0; 13];
        for c in splits.next().unwrap().bytes() {
            let card_num = match c {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'T' => 9,
                b'2'..=b'9' => c - b'2' + 1,
                b'J' => 0,
                _ => panic!("unexpected char")
            };
            freqs[card_num as usize] += 1;
            hand.push(card_num);
        }
        // remove jokers before sorting
        let joker_count = freqs[0];
        freqs[0] = 0;
        freqs.sort();
        freqs[12] += joker_count; // add joker to most frequent count
        let bet = str_to_u64(splits.next().unwrap());
        let hand_type = hand_type(freqs);
        println!("{} {:?} {:?}", line, freqs, (&hand_type, bet));
        hands.push((hand, hand_type, bet));
    }

    hands.sort_by(|a, b| {
        if a.1 > b.1 {
            return Ordering::Greater;
        } else if a.1 < b.1 {
            return Ordering::Less;
        }
        for i in 0..13 {
            if a.0[i] > b.0[i] {
                return Ordering::Greater;
            } else if a.0[i] < b.0[i] {
                return Ordering::Less;
            }
        }
        return Ordering::Equal; 
    });

    println!("{:?}", hands);
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += (i+1) as u64 * hand.2;
    }
    println!("{}", result);
}

fn hand_type(freqs: [u8; 13]) -> HandType {
    match freqs {
        [.., 5] => HandType::FiveOfKind,
        [.., 4] => HandType::FourOfKind,
        [.., 2, 3] => HandType::FullHouse,
        [.., 3] => HandType::ThreeOfKind,
        [.., 2, 2] => HandType::TwoPair,
        [.., 2] => HandType::OnePair,
        _ => HandType::HighCard
    }
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

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}
