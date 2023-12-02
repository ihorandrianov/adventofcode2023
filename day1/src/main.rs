use rayon::prelude::*;
use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let mut nums_map: HashMap<String, String> = HashMap::new();
    nums_map.insert("one".to_string(), "o1e".to_string());
    nums_map.insert("two".to_string(), "t2o".to_string());
    nums_map.insert("three".to_string(), "t3e".to_string());
    nums_map.insert("four".to_string(), "f4r".to_string());
    nums_map.insert("five".to_string(), "f5e".to_string());
    nums_map.insert("six".to_string(), "s6x".to_string());
    nums_map.insert("seven".to_string(), "s7n".to_string());
    nums_map.insert("eight".to_string(), "e8t".to_string());
    nums_map.insert("nine".to_string(), "n9e".to_string());
    nums_map.insert("1".to_string(), "1".to_string());
    nums_map.insert("2".to_string(), "2".to_string());
    nums_map.insert("3".to_string(), "3".to_string());
    nums_map.insert("4".to_string(), "4".to_string());
    nums_map.insert("5".to_string(), "5".to_string());
    nums_map.insert("6".to_string(), "6".to_string());
    nums_map.insert("7".to_string(), "7".to_string());
    nums_map.insert("8".to_string(), "8".to_string());
    nums_map.insert("9".to_string(), "9".to_string());
    let file: u32 = read_to_string("input.txt")
        .unwrap()
        .lines()
        .par_bridge()
        .map(|line| {
            let nums: Vec<&str> = nums_map.keys().map(|x| x.as_str()).collect();
            let line = replace_with_map(line, &nums_map);
            let parsed_line = parse_line(&line, &nums);
            let result = Num::from_str(parsed_line[0]);
            let result = result.concat(&Num::from_str(parsed_line.last().unwrap()));
            result.to_u32()
            
        })
        .sum();
           
    println!("{}", file);
}


// fn first_solution() {
//     let file = read_to_string("input.txt")
//     .unwrap()
//     .lines()
//     .par_bridge()
//     .map(|line| {
//        let line = line.as_bytes();
//        let length = line.len();
//        let rng = 0..length;
//        let reversed_rng = rng.clone().rev();
//        let mut res = ['a', 'a'];
       
//        for (i,j) in rng.zip(reversed_rng) {
//             if res[0].is_numeric() && res[1].is_numeric() {
//                 break
//             }
//             if !res[0].is_numeric() {
//                 res[0] = line[i] as char;
//             }
//             if !res[1].is_numeric() {
//                 res[1] = line[j] as char;
//             }
//        }
//        let output: u32 = res.iter().collect::<String>().parse().unwrap();
//        output
//     })
//     .reduce(|| 0, |a, b| a + b);
// println!("{}", file);
// }

struct Num(u32);

impl Num {
    fn from_str(input: &str) -> Self {
        match input {
            "one" => Num(1),
            "two" => Num(2),
            "three" => Num(3),
            "four" => Num(4),
            "five" => Num(5),
            "six" => Num(6),
            "seven" => Num(7),
            "eight" => Num(8),
            "nine" => Num(9),
            "1" => Num(1),
            "2" => Num(2),
            "3" => Num(3),
            "4" => Num(4),
            "5" => Num(5),
            "6" => Num(6),
            "7" => Num(7),
            "8" => Num(8),
            "9" => Num(9),
            _ => Num(0),
        }
    }
    
    fn concat(&self, other: &Self) -> Self {
        let mut result = self.0.to_string();
        result.push_str(&other.0.to_string());
        Num(result.parse().unwrap())
    }
    
    fn to_u32(&self) -> u32 {
        self.0
    }
    
}



fn replace_with_map<'a>(text: &'a str, map: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut start = 0;
    let text_len = text.len();
    
    while start < text_len {
        if let Some((word, len)) = parse_word(&text[start..], &map.keys().map(|x| x.as_str()).collect::<Vec<&str>>()) {
            result.push_str(map.get(word).unwrap());
            start += len;
        } else {
            result.push(text.chars().nth(start).unwrap());
            start += 1;
        }
    }
    
    result
}

fn parse_line<'a>(line: &str, words_to_find: &[&'a str]) -> Vec<&'a str> {
    let mut start = 0;
    let text_len = line.len();
    let mut result: Vec<&str> = vec![];
    
    while start < text_len {
        if let Some((word, len)) = parse_word(&line[start..], words_to_find) {
            result.push(word);
            start += len;
        } else {
            start += 1;
        }
    }
    
    result
}


fn parse_word<'a>(text: &str, search_words: &[&'a str]) -> Option<(&'a str, usize)> {
    for &word in search_words {
        if text.starts_with(word) {
            return Some((word, word.len()))
        }
    }
    None
}