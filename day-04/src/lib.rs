use std::collections::{HashSet, HashMap};

use card::card::Card;

mod card;

pub fn process_part1(input_str: &str) -> u32 {
    let mut sum_points = 0;

    for line in input_str.lines() {
        let mut game_details = line.split(':').last().unwrap().split("|");
        let winning_numbers = parse_input_numbers(game_details.next().unwrap());
        let owned_numbers = parse_input_numbers(game_details.next().unwrap());

        let points = count_points(winning_numbers, owned_numbers) as u32;

        if points > 0 {
            sum_points += (2 as u32).pow(points - 1);
        }
    }

    sum_points
}

pub fn process_part2(input_str: &str) -> u32 {
    let mut num_cards: u32;
    let mut cards_map: HashMap<usize, Card> = HashMap::new();
    let mut card_id: usize = 1;
    let mut cards_queue: Vec<usize> = vec![];

    for line in input_str.lines() {
        let mut game_details = line.split(':').last().unwrap().split("|");
        let winning_numbers = parse_input_numbers(game_details.next().unwrap());
        let owned_numbers = parse_input_numbers(game_details.next().unwrap());

        let points = count_points(winning_numbers, owned_numbers) as u32;
        let points_value = match points > 0 {
            true => (2 as u32).pow(points - 1),
            false => 0,
        };

        cards_map.insert(card_id, Card::new(card_id, points_value, points));
        for i in 0..points as usize {
            cards_queue.push(card_id + i + 1);
        }
        card_id += 1;

    }

    num_cards = card_id as u32 - 1;
    while !cards_queue.is_empty() {
        num_cards += 1;
        let card_id = cards_queue.pop().unwrap();
        if cards_map.contains_key(&card_id) {
            let card = cards_map.get(&card_id).expect("expected card not in a map");
            for i in 0..card.num_winning as usize {
                cards_queue.push(card_id + i + 1);
            }
        }
    }

    num_cards
}

fn count_points(winning_numbers: Vec<u32>, owned_numbers: Vec<u32>) -> usize {
    let winning_set: HashSet<u32> = HashSet::from_iter(winning_numbers);
    let owned_numbers: HashSet<u32> = HashSet::from_iter(owned_numbers);
    
    winning_set.intersection(&owned_numbers).count()
}

fn parse_input_numbers(input_numbers: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];

    for num_str in input_numbers.split(' ') {
        match num_str.parse::<u32>() {
            Ok(num) => {
                numbers.push(num);
            }
            Err(_) => {}
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_input() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(13, process_part1(&input_str));
    }

    #[test]
    fn test_example_input_part2() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(30, process_part2(&input_str));
    }
}
