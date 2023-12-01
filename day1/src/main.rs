use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part1(contents: &String) -> u32 {
    let mut sum: u32 = 0;

    for line in contents.lines() {
        let mut number: u32 = 0;

        for letter in line.chars() {
            if letter.is_digit(10) {
                number += letter.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for letter in line.chars().rev() {
            if letter.is_digit(10) {
                number += letter.to_digit(10).unwrap();
                break;
            }
        }

        sum += number;
    }

    return sum;
}

fn part2(contents: &String) -> u32 {
    let mut sum: u32 = 0;

    for line in contents.lines() {
        let mut number: u32 = 0;

        let ones: Vec<_> = line.match_indices("one").map(|x| (x.0, '1')).collect();
        let twos: Vec<_> = line.match_indices("two").map(|x| (x.0, '2')).collect();
        let threes: Vec<_> = line.match_indices("three").map(|x| (x.0, '3')).collect();
        let fours: Vec<_> = line.match_indices("four").map(|x| (x.0, '4')).collect();
        let fives: Vec<_> = line.match_indices("five").map(|x| (x.0, '5')).collect();
        let sixs: Vec<_> = line.match_indices("six").map(|x| (x.0, '6')).collect();
        let sevens: Vec<_> = line.match_indices("seven").map(|x| (x.0, '7')).collect();
        let eights: Vec<_> = line.match_indices("eight").map(|x| (x.0, '8')).collect();
        let nines: Vec<_> = line.match_indices("nine").map(|x| (x.0, '9')).collect();

        let num1s: Vec<_> = line.match_indices("1").map(|x| (x.0, '1')).collect();
        let num2s: Vec<_> = line.match_indices("2").map(|x| (x.0, '2')).collect();
        let num3s: Vec<_> = line.match_indices("3").map(|x| (x.0, '3')).collect();
        let num4s: Vec<_> = line.match_indices("4").map(|x| (x.0, '4')).collect();
        let num5s: Vec<_> = line.match_indices("5").map(|x| (x.0, '5')).collect();
        let num6s: Vec<_> = line.match_indices("6").map(|x| (x.0, '6')).collect();
        let num7s: Vec<_> = line.match_indices("7").map(|x| (x.0, '7')).collect();
        let num8s: Vec<_> = line.match_indices("8").map(|x| (x.0, '8')).collect();
        let num9s: Vec<_> = line.match_indices("9").map(|x| (x.0, '9')).collect();

        let mut all_nums: Vec<_> = vec![
            ones, twos, threes, fours, fives, sixs, sevens, eights, nines, num1s, num2s, num3s,
            num4s, num5s, num6s, num7s, num8s, num9s,
        ]
        .concat();

        all_nums.sort_by(|a, b| a.0.cmp(&b.0));

        sum += all_nums[0].1.to_digit(10).unwrap() * 10
            + all_nums.last().unwrap().1.to_digit(10).unwrap();
    }

    return sum;
}
