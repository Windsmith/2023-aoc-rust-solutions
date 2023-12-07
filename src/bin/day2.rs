use regex::Regex;
use std::fs;

fn main() {
    let contents: String = utils::get_input(2).unwrap();

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    let mut sum: usize = 0;
    let re = Regex::new(r"Game \d{1,3}:").unwrap();

    let bag = Round {
        red: 12,
        blue: 14,
        green: 13,
    };

    for (index, line) in contents.lines().enumerate() {
        let mut rounds = re.replace(line, "");
        let mut valid = true;

        for round in rounds.split(";").collect::<Vec<&str>>() {
            let round_obj = Round::new(round);
            if !Round::is_possible(&bag, &round_obj) {
                valid = false;
                break;
            }
        }

        if valid {
            sum += index + 1;
        }
    }

    sum
}

fn part2(contents: &String) -> u32 {
    let mut sum: u32 = 0;
    let re = Regex::new(r"Game \d{1,3}:").unwrap();

    for (index, line) in contents.lines().enumerate() {
        let mut rounds = re.replace(line, "");

        let mut minimum_bag = Round {
            red: 0,
            blue: 0,
            green: 0,
        };

        for round in rounds.split(";").collect::<Vec<&str>>() {
            let round_obj = Round::new(round);

            if minimum_bag.red < round_obj.red {
                minimum_bag.red = round_obj.red
            };
            if minimum_bag.blue < round_obj.blue {
                minimum_bag.blue = round_obj.blue
            };
            if minimum_bag.green < round_obj.green {
                minimum_bag.green = round_obj.green
            };
        }

        let power = minimum_bag.red * minimum_bag.blue * minimum_bag.green;

        sum += power;
    }

    sum
}

struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

impl Round {
    fn new(round_string: &str) -> Self {
        let re_red = Regex::new(r"\d{1,3} red").unwrap();
        let re_blue = Regex::new(r"\d{1,3} blue").unwrap();
        let re_green = Regex::new(r"\d{1,3} green").unwrap();

        let red = match re_red.find(round_string) {
            Some(value) => value.as_str().split_whitespace().collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap(),
            None => 0,
        };

        let blue = match re_blue.find(round_string) {
            Some(value) => value.as_str().split_whitespace().collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap(),
            None => 0,
        };

        let green = match re_green.find(round_string) {
            Some(value) => value.as_str().split_whitespace().collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap(),
            None => 0,
        };

        return Round { red, blue, green };
    }

    fn is_possible(round1: &Round, round2: &Round) -> bool {
        if round1.blue >= round2.blue && round1.red >= round2.red && round1.green >= round2.green {
            return true;
        }

        return false;
    }
}
