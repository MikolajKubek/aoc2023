use card::Hand;

mod card;

pub fn process_puzzle(input_str: &str, part2: bool) -> u32 {
    let mut cards = read_input_data(input_str, part2);
    cards.sort();
    
    let mut sum = 0;
    for (i, card) in cards.iter().enumerate() {
        sum += (i as u32 + 1) * card.bid;
    }

    sum
}

fn read_input_data(input_str: &str, part2: bool) -> Vec<Hand> {
    let mut cards: Vec<Hand> = vec![];
    for line in input_str.lines() {
        let mut line_split = line.split(' ');
        cards.push(Hand::new(
            line_split.next().unwrap().to_owned(),
            line_split.next().unwrap().parse::<u32>().unwrap(),
            part2
        ));
    }

    cards
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, fs::read_to_string};

    use super::*;

    #[test]
    fn test_example_input_part1() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(6440, process_puzzle(&input_str, false));
    }

    #[test]
    fn test_example_input_part2() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(5905, process_puzzle(&input_str, true));
    }
}
