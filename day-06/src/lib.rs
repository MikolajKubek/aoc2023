use std::iter::zip;

use race::Race;

mod race;

pub fn process_puzzle(input_str: &str) -> u64 {
    let races = parse_input(input_str);

    let mut result = 1;
    for race in races {
        let mut winning_scenarios = 0;
        for hold_time in 0..race.time {
            if get_travelled_distance(hold_time, race.time) > race.record_distance {
                winning_scenarios += 1;
            }
        }
        result *= winning_scenarios;
    }

    result
}

fn get_travelled_distance(hold_time: u64, total_time: u64) -> u64 {
    (total_time - hold_time) * hold_time
}

// Input file should consist of two lines as follows:
// Time:    time1   time2   time3 ...
// Distance dist1   dist2   dist3 ...
fn parse_input(input_str: &str) -> Vec<Race> {
    let times = extract_u64_from_text(input_str.lines().next().unwrap());
    let distances = extract_u64_from_text(input_str.lines().nth(1).unwrap());

    zip(times, distances).map(|(time, distance)| Race::new(time, distance)).collect::<Vec<Race>>()
}

fn extract_u64_from_text(text: &str) -> Vec<u64> {
    if text.len() == 1 {
        return match text.chars().next().unwrap().to_digit(10) {
            Some(number) => vec![number.into()],
            None => vec![]
        }
    }

    let mut nums: Vec<u64> = vec![];
    let mut num_start: usize = 0;
    let mut current_index: usize = 0;

    while current_index < text.len() {
        if text.chars().nth(current_index).unwrap().is_numeric() {
            if num_start == 0 {
                num_start = current_index;
            }
        }
        else if num_start != 0 {
            nums.push(text[num_start..current_index].parse::<u64>().unwrap());
            num_start = 0;
        }
        current_index += 1;
    }

    if num_start != 0 {
        nums.push(text[num_start..current_index].parse::<u64>().unwrap());
    }


    nums
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_input() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input file");
        assert_eq!(288, process_puzzle(&input_str));
    }
    
    #[test]
    fn test_example_input_part2() {
        let input_str = read_to_string("example_input_2.txt").expect("unable to read example input file");
        assert_eq!(71503, process_puzzle(&input_str));
    }
}
