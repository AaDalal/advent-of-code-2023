use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default)]
struct Trie {
    value: u8, // 0 if not set (ie, not a terminal node)
    children: [Option<Box<Trie>>; 26]
}

impl Trie {
    fn add_word(&mut self, string: &str, value: u8) {
        let mut node = self; 
        for c in string.chars() {
            let child = &mut node.children[c as usize - 'a' as usize];
            match child {
                Some(child) => node = child,
                None => {
                    let new_child = Box::new(Trie::default());
                    *child = Some(new_child);
                    node = child.as_mut().unwrap();
                }
           }
        }
        node.value = value;
    }
    fn eager_match(&self, string: &str, result: &mut u8) -> u8 {
        let mut node = self;
        let mut count = 0;
        for c in string.chars() {
            match c {
                'a'..='z' => {
                    match &node.children[c as usize - 'a' as usize] {
                        Some(child) => {
                            if child.value > 0 {
                                *result = child.value;
                                return count; 
                            }
                            node = child;

                        },
                        None => {
                            return 0; // return match size
                        }
                    }
                    count += 1;
                },
                _ => {
                    return 0;
                }
            }
        }
        return 0; // return match size
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut trie = Trie::default();
    trie.add_word("one", 1);
    trie.add_word("two", 2);
    trie.add_word("three", 3);
    trie.add_word("four", 4);
    trie.add_word("five", 5);
    trie.add_word("six", 6);
    trie.add_word("seven", 7);
    trie.add_word("eight", 8);
    trie.add_word("nine", 9);

    let mut result: u32 = 0;
    for res_line in read_lines(file_path).unwrap() {
        let mut first: u8 = 0;
        let mut last: u8 = 0;

        // match against line
        let line = res_line.unwrap();
        let length = line.len();
        let mut idx = 0;
        while idx < length {
            // try to match by character 1..9
            let c = line.as_bytes()[idx] as char;
            match c {
                '1'..='9' => {
                    last = c as u8 - '0' as u8;
                    if first == 0 {
                        first = last;
                    }
                },
                'a'..='z' => {
                    // try to match by word (assume character is a..z)
                    let mut word_match: u8 = 0;
                    let match_size = trie.eager_match(&line[idx..length], &mut word_match);
                    if word_match > 0 {
                        last = word_match;
                        if first == 0 {
                            first = last;
                        }
                        idx += match_size as usize;
                        continue;
                    }
                },
                _ => ()
            };

            // otherwise, just continue
            idx += 1;
        }
        println!("{} {} {}", line, first, last);
        result += (first * 10 + last) as u32;
    }

    println!("{}", result);
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?; 
    Ok(io::BufReader::new(file).lines()) // create bufreader with default capacity
}
