use map::{RangeMap, Ranger};
use rayon::prelude::*;

mod map;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let res: Vec<_> = file.split("\n\n").collect();
    let seeds = res[0].split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut seed_new: Vec<(usize, usize)> = vec![];
    for x in 0..=seeds.len() - 2 {
        if x % 2 == 0 {
            seed_new.push((
                seeds[x] as usize,
                (seeds[x] + seeds[x + 1]).try_into().unwrap(),
            ))
        }
    }
    let maps: Vec<_> = res[1..]
        .into_par_iter()
        .map(|x| {
            x.split('\n')
                .skip(1)
                .map(|x| {
                    let res = x
                        .split(' ')
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    Ranger::new(res)
                })
                .collect::<Vec<_>>()
        })
        .map(RangeMap::from)
        .collect::<Vec<_>>();
    println!("start iterate");
    //merge and sort ranges
    seed_new.sort_by(|a, b| a.0.cmp(&b.0));
    println!("start transform");

    let res = seed_new
        .par_iter()
        .map(|seed_new| {
            let res = (seed_new.0..=seed_new.1)
                .into_par_iter()
                .map(|x| {
                    let mut f: u32 = x as u32;
                    maps.iter().for_each(|m| f = m.transform(f));
                    f
                })
                .min()
                .unwrap();
            println!("res: {}", res);
            res
        })
        .min()
        .unwrap();
    println!("res: {}", res);

    // let res = seed_new
    //     .par_iter()
    //     .map(|seed_new| {
    //         (seed_new.0..=seed_new.1)
    //             .into_par_iter()
    //             .map(|x| {
    //                 let mut f: u32 = x as u32;
    //                 maps.iter().for_each(|m| f = m.transform(f));
    //                 f
    //             })
    //             .min()
    //             .unwrap()
    //     })
    //     .min()
    //     .unwrap();
}
