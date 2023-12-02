use crate::game_constructor::*;
use std::fs::read_to_string;

mod game_constructor;
fn main() {
    let input = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let game = Game::from_input(line);
            game.max_multiply()
        })
        .sum::<u32>();

    println!("{:?}", input);
}
