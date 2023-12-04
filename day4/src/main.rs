use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let games = line
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .collect::<Vec<&str>>();
            let mut win_nums = games[0].chars().peekable();
            let mut win_nums_set: HashSet<u32> = HashSet::new();
            let mut buff = String::new();
            while let Some(x) = win_nums.next() {
                if x.is_whitespace() && buff.len() != 0 {
                    let res = buff.parse::<u32>().unwrap();
                    win_nums_set.insert(res);
                    buff.clear();
                    continue;
                } else if x.is_numeric() {
                    buff.push(x);
                    continue;
                } else {
                    continue;
                }
            }
            let mut win_counter: u32 = 0;
            let mut cards_won: u32 = 0;
            let mut flag = false;
            let mut card = games[1].chars().peekable();
            while let Some(x) = card.next() {
                if x.is_whitespace() && buff.len() != 0 {
                    let res = buff.parse::<u32>().unwrap();
                    let contains = win_nums_set.contains(&res);
                    if contains {
                        if flag {
                            cards_won += 1;
                            win_counter *= 2;
                        } else {
                            cards_won += 1;
                            win_counter = 1;
                            flag = true;
                        }
                    }
                    buff.clear();
                } else if x.is_numeric() {
                    buff.push(x);
                };
                if let None = card.peek() {
                    if buff.len() > 0 {
                        let res = buff.parse::<u32>().unwrap();
                        let contains = win_nums_set.contains(&res);
                        if contains {
                            if flag {
                                cards_won += 1;
                                win_counter *= 2;
                            } else {
                                cards_won += 1;
                                win_counter = 1;
                                flag = true;
                            }
                        }
                    }
                }
            }

            (cards_won, win_counter)
        })
        .enumerate()
        .collect::<Vec<(usize, (u32, u32))>>();

    let result: u32 = input
        .iter()
        .map(|x| {
            let sum = count_deps(&input, x.0);
            sum
        })
        .sum();
    println!("{}", result)
}

fn count_deps(slice: &[(usize, (u32, u32))], caller_idx: usize) -> u32 {
    let mut queue: VecDeque<(usize, (u32, u32))> = VecDeque::new();
    let mut result = 0;
    queue.push_front(slice[caller_idx]);
    while !queue.is_empty() {
        let x = queue.pop_front().unwrap();
        result += 1;
        if x.1 .0 != 0 {
            for i in &slice[x.0 + 1..x.0 + 1 + x.1 .0 as usize] {
                queue.push_back(*i)
            }
        }
    }
    result
}
