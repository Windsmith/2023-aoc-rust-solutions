use std::error::Error;

use regex::Regex;

fn main() {
    let contents = utils::get_input(3).expect("Input not found");

    let sample = "467..114..
...*......
..35.633..
......#...
*617......
.....+.58.
..592.....
......755.
...$..*...
..664..598
";

    println!("{:?}", part1(&sample.to_string()));
    println!("{:?}", part1(&contents));
    println!("{:?}", part2(&sample.to_string()));
    println!("{:?}", part2(&contents));
}

fn get_num_left(characters: &Vec<char>, mut starting_index: usize) -> Result<u32, Box<dyn Error>> {
    let mut num = 0;
    let mut power = 1;
    while characters[starting_index].is_digit(10) {
        num += characters[starting_index].to_digit(10).unwrap() * power;
        if starting_index == 0 {
            break;
        }
        starting_index -= 1;
        power *= 10;
    }
    Ok(num.try_into()?)
}

fn get_num_right(characters: &Vec<char>, mut starting_index: usize) -> Result<u32, Box<dyn Error>> {
    let mut num = 0;
    while characters[starting_index].is_digit(10) {
        num = num * 10 + characters[starting_index].to_digit(10).unwrap();
        if starting_index > characters.len() {
            break;
        }
        starting_index += 1;
    }
    Ok(num.try_into()?)
}

fn part1(contents: &String) -> u32 {
    let re = Regex::new(r"[-!$%^&*()_+@|~=`{}\[\]:\;#'<>?,\/]").unwrap();

    //subtract with a symbol's index to get the index right above it and add for the index right below it.
    let divider = contents.find("\n").unwrap() + 1;

    let characters: Vec<char> = contents.chars().collect();

    let mut sum: u32 = 0;

    for symbol in re.find_iter(&contents) {
        let index = symbol.start();

        let up_indices = [index - divider - 1, index - divider, index - divider + 1];

        let down_indices = [index + divider - 1, index + divider, index + divider + 1];

        let left_index = index - 1;

        let right_index = index + 1;

        let mut symbol_sum = 0;

        if characters[up_indices[0]].is_digit(10)
            && characters[up_indices[1]].is_digit(10)
            && characters[up_indices[2]].is_digit(10)
        {
            let mut num = 0;
            let mut starting_index = up_indices[2] + 1;
            num = get_num_left(&characters, up_indices[2]).unwrap();
            while characters[starting_index].is_digit(10) {
                num = num * 10 + characters[starting_index].to_digit(10).unwrap();
                starting_index += 1;
            }
            symbol_sum += num;
        } else if characters[up_indices[0]].is_digit(10) && characters[up_indices[1]].is_digit(10) {
            symbol_sum += get_num_left(&characters, up_indices[1]).unwrap();
        } else if characters[up_indices[1]].is_digit(10) && characters[up_indices[2]].is_digit(10) {
            symbol_sum += get_num_right(&characters, up_indices[1]).unwrap();
        } else {
            if characters[up_indices[0]].is_digit(10) {
                symbol_sum += get_num_left(&characters, up_indices[0]).unwrap();
            }
            if characters[up_indices[1]].is_digit(10) {
                symbol_sum += characters[up_indices[1]].to_digit(10).unwrap();
            }
            if characters[up_indices[2]].is_digit(10) {
                symbol_sum += get_num_right(&characters, up_indices[2]).unwrap();
            }
        }

        if characters[down_indices[0]].is_digit(10)
            && characters[down_indices[1]].is_digit(10)
            && characters[down_indices[2]].is_digit(10)
        {
            let mut num = 0;
            let mut starting_index = down_indices[2] + 1;
            num = get_num_left(&characters, down_indices[2]).unwrap();
            while characters[starting_index].is_digit(10) {
                num = num * 10 + characters[starting_index].to_digit(10).unwrap();
                starting_index += 1;
            }
            symbol_sum += num;
        } else if characters[down_indices[0]].is_digit(10)
            && characters[down_indices[1]].is_digit(10)
        {
            symbol_sum += get_num_left(&characters, down_indices[1]).unwrap();
        } else if characters[down_indices[1]].is_digit(10)
            && characters[down_indices[2]].is_digit(10)
        {
            symbol_sum += get_num_right(&characters, down_indices[1]).unwrap();
        } else {
            if characters[down_indices[0]].is_digit(10) {
                symbol_sum += get_num_left(&characters, down_indices[0]).unwrap();
            }
            if characters[down_indices[1]].is_digit(10) {
                symbol_sum += characters[down_indices[1]].to_digit(10).unwrap();
            }
            if characters[down_indices[2]].is_digit(10) {
                symbol_sum += get_num_right(&characters, down_indices[2]).unwrap();
            }
        }

        if characters[left_index].is_digit(10) {
            symbol_sum += get_num_left(&characters, left_index).unwrap();
        }

        if characters[right_index].is_digit(10) {
            symbol_sum += get_num_right(&characters, right_index).unwrap();
        }

        sum += symbol_sum;
    }

    sum
}

fn part2(contents: &String) -> u32 {
    let re = Regex::new(r"[-!$%^&*()_+@|~=`{}\[\]:\;#'<>?,\/]").unwrap();

    //subtract with a symbol's index to get the index right above it and add for the index right below it.
    let divider = contents.find("\n").unwrap() + 1;

    let characters: Vec<char> = contents.chars().collect();

    let mut sum: u32 = 0;

    for symbol in re.find_iter(&contents) {
        let index = symbol.start();

        let up_indices = [index - divider - 1, index - divider, index - divider + 1];

        let down_indices = [index + divider - 1, index + divider, index + divider + 1];

        let left_index = index - 1;

        let right_index = index + 1;

        let mut symbol_sum = 1;

        let mut number_count = 0;

        if characters[up_indices[0]].is_digit(10)
            && characters[up_indices[1]].is_digit(10)
            && characters[up_indices[2]].is_digit(10)
        {
            number_count += 1;
            let mut num = 0;
            let mut starting_index = up_indices[2] + 1;
            num = get_num_left(&characters, up_indices[2]).unwrap();
            while characters[starting_index].is_digit(10) {
                num = num * 10 + characters[starting_index].to_digit(10).unwrap();
                starting_index += 1;
            }
            symbol_sum *= num;
        } else if characters[up_indices[0]].is_digit(10) && characters[up_indices[1]].is_digit(10) {
            number_count += 1;
            symbol_sum *= get_num_left(&characters, up_indices[1]).unwrap();
        } else if characters[up_indices[1]].is_digit(10) && characters[up_indices[2]].is_digit(10) {
            number_count += 1;
            symbol_sum *= get_num_right(&characters, up_indices[1]).unwrap();
        } else {
            if characters[up_indices[0]].is_digit(10) {
                number_count += 1;
                symbol_sum *= get_num_left(&characters, up_indices[0]).unwrap();
            }
            if characters[up_indices[1]].is_digit(10) {
                number_count += 1;
                symbol_sum *= characters[up_indices[1]].to_digit(10).unwrap();
            }
            if characters[up_indices[2]].is_digit(10) {
                number_count += 1;
                symbol_sum *= get_num_right(&characters, up_indices[2]).unwrap();
            }
        }

        if characters[down_indices[0]].is_digit(10)
            && characters[down_indices[1]].is_digit(10)
            && characters[down_indices[2]].is_digit(10)
        {
            number_count += 1;
            let mut num = 0;
            let mut starting_index = down_indices[2] + 1;
            num = get_num_left(&characters, down_indices[2]).unwrap();
            while characters[starting_index].is_digit(10) {
                num = num * 10 + characters[starting_index].to_digit(10).unwrap();
                starting_index += 1;
            }
            symbol_sum *= num;
        } else if characters[down_indices[0]].is_digit(10)
            && characters[down_indices[1]].is_digit(10)
        {
            number_count += 1;
            symbol_sum *= get_num_left(&characters, down_indices[1]).unwrap();
        } else if characters[down_indices[1]].is_digit(10)
            && characters[down_indices[2]].is_digit(10)
        {
            number_count += 1;
            symbol_sum *= get_num_right(&characters, down_indices[1]).unwrap();
        } else {
            if characters[down_indices[0]].is_digit(10) {
                number_count += 1;
                symbol_sum *= get_num_left(&characters, down_indices[0]).unwrap();
            }
            if characters[down_indices[1]].is_digit(10) {
                number_count += 1;
                symbol_sum *= characters[down_indices[1]].to_digit(10).unwrap();
            }
            if characters[down_indices[2]].is_digit(10) {
                number_count += 1;
                symbol_sum *= get_num_right(&characters, down_indices[2]).unwrap();
            }
        }

        if characters[left_index].is_digit(10) {
            number_count += 1;
            symbol_sum *= get_num_left(&characters, left_index).unwrap();
        }

        if characters[right_index].is_digit(10) {
            number_count += 1;
            symbol_sum *= get_num_right(&characters, right_index).unwrap();
        }

        if number_count != 2 {
            continue;
        }
        sum += symbol_sum;
    }

    sum
}
