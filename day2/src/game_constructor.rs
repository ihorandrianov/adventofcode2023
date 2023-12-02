#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
    is_possible: bool,
    max_multiply: u32,
}

#[derive(Debug, Clone)]
pub struct Round {
    blue: u32,
    red: u32,
    green: u32,
    is_possible: bool,
}

impl Game {
    pub fn from_input(input: &str) -> Self {
        let mut input = input.split(":").collect::<Vec<&str>>();
        let mut id = input[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let rounds = input[1]
            .split("; ")
            .map(|round| Round::from_input(round))
            .collect::<Vec<Round>>();
        let possible = rounds.iter().map(|x| x.is_possible).all(|x| x);
        let maxes = rounds
            .iter()
            .map(|x| (x.red, x.blue, x.green))
            .reduce(|mut acc, v| {
                if v.0 > acc.0 {
                    acc.0 = v.0
                }
                if v.1 > acc.1 {
                    acc.1 = v.1
                }
                if v.2 > acc.2 {
                    acc.2 = v.2
                }
                acc
            })
            .unwrap();
        let max_multiply = maxes.0 * maxes.1 * maxes.2;
        match possible {
            true => id = id,
            false => id = 0,
        }
        Self {
            id,
            rounds,
            is_possible: possible,
            max_multiply,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn max_multiply(&self) -> u32 {
        self.max_multiply
    }
}

impl Round {
    pub fn from_input(input: &str) -> Self {
        let mut round = Round {
            blue: 0,
            red: 0,
            green: 0,
            is_possible: false,
        };

        input.to_string().trim().split(", ").for_each(|x| {
            let splited = x.split_whitespace().collect::<Vec<&str>>();
            let amount = splited[0].parse::<u32>().unwrap();
            match splited[1] {
                "blue" => round.blue = amount,
                "red" => round.red = amount,
                "green" => round.green = amount,
                &_ => (),
            }
        });
        round.is_possible();
        round
    }

    fn is_possible(&mut self) {
        let blue_possible = self.blue <= 14;
        let red_possible = self.red <= 12;
        let green_possible = self.green <= 13;

        self.is_possible = blue_possible && red_possible && green_possible;
    }
}

fn every<T, I>(v: I) -> bool
where
    I: IntoIterator<Item = T>,
    T: std::ops::Not<Output = bool>,
{
    v.into_iter().all(|x| !!x)
}
