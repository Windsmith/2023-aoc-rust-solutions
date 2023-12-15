use regex::Regex;

fn main() {
    let contents: String = utils::get_input(4).unwrap();

    let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    println!("{}", part1(&sample.to_string()));
    println!("{}", part1(&contents));
    println!("{}", part2(&sample.to_string()));
    println!("{}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    let mut sum: usize = 0;
    let re_num = Regex::new(r"\d+").unwrap();

    for line in contents.lines() {
        let card = line.split(":").collect::<Vec<_>>()[1];
        let scratch_card_numbers: Vec<&str> = card.split("|").collect();

        let scratch_card: Vec<&str> = re_num
            .find_iter(scratch_card_numbers[0])
            .map(|x| x.as_str())
            .collect();

        let winning_numbers: Vec<&str> = re_num
            .find_iter(scratch_card_numbers[1])
            .map(|x| x.as_str())
            .collect();

        let mut final_val = 1;

        for number in scratch_card {
            if winning_numbers.contains(&number) {
                final_val *= 2;
            }
        }
        sum += final_val / 2;
    }

    sum
}

fn part2(contents: &String) -> usize {
    let mut sum: usize = 0;
    let re_num = Regex::new(r"\d+").unwrap();

    let mut card_copies: Vec<usize> = vec![];

    for (index, line) in contents.lines().enumerate() {
        card_copies.push(index + 1);

        let card = line.split(":").collect::<Vec<_>>()[1];
        let scratch_card_numbers: Vec<&str> = card.split("|").collect();

        let scratch_card: Vec<&str> = re_num
            .find_iter(scratch_card_numbers[0])
            .map(|x| x.as_str())
            .collect();

        let winning_numbers: Vec<&str> = re_num
            .find_iter(scratch_card_numbers[1])
            .map(|x| x.as_str())
            .collect();

        let mut card_number = 1;
        for number in scratch_card {
            if winning_numbers.contains(&number) {
                for i in 0..card_copies.iter().filter(|x| **x == index + 1).count() {
                    card_copies.push(index + 1 + card_number);
                }
                card_number += 1
            }
        }
    }

    card_copies.len()
}
