use std::collections::HashSet;

use seed_mapping::seed_mapping::SeedMap;

mod seed_mapping;

pub fn process_puzzle(input_str: &str, part2: bool) -> u64 {
    let mut lines = input_str.lines();
    let seeds = match part2 {
        true => parse_seeds_part2(lines.next().unwrap()),
        false => HashSet::from_iter(parse_seeds(lines.next().unwrap())),
    };
    println!("number of seeds: {}", seeds.len());


    let mut mappings: Vec<Box<SeedMap>> = vec![];
    let mut current_mapping: Option<Box<SeedMap>> = None;


    while let Some(current_line) = lines.next() {
        if current_line.is_empty() {
            continue;
        }
        let line_split = current_line.split(' ');
        let split_count = line_split.clone().count();

        match split_count {
            2 => {
                if current_mapping.as_ref().is_some() {
                    mappings.push(current_mapping.take().unwrap());
                }
                current_mapping.replace(Box::new(SeedMap::new()));
            }
            3 => {
                if current_mapping.is_some() {
                    let range = parse_range(line_split);
                    current_mapping.as_mut().unwrap().add_range(range[0], range[1], range[2]);
                }

            }
            _ => {}
        }
    }
    if current_mapping.is_some() {
        mappings.push(current_mapping.take().unwrap());
    }

    let mut min_location = u64::MAX;

    for seed in seeds {
        let mut current_value = seed;
        for mapping in &mappings {
            current_value = mapping.get_destination(current_value);
        }
        min_location = min_location.min(current_value);
    }

    min_location
}

fn parse_range(line_split: std::str::Split<'_, char>) -> Vec<u64> {
    line_split.map(|item| item.parse::<u64>().unwrap()).collect()
}

fn parse_seeds_part2(seeds_str: &str) -> HashSet<u64> {
    let mut seeds: HashSet<u64> = HashSet::new();
    let seed_input = parse_seeds(seeds_str);
    for index in 0..seed_input.len() {
        if index % 2 == 0 && index + 1 < seed_input.len() {
            for i in 0..seed_input[index + 1] {
                seeds.insert(seed_input[index] + i);
            }
        }
    }
    
    seeds
}

fn parse_seeds(next: &str) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];
    for item in next.split(" ") {
        match item.parse::<u64>() {
            Ok(seed) => seeds.push(seed),
            Err(_) => {}
        }
    }

    seeds
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_input() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(35, process_puzzle(&input_str, false));
    }

    #[test]
    fn test_example_input_part2() {
        let input_str = read_to_string("example_input.txt").expect("unable to read example input");
        assert_eq!(46, process_puzzle(&input_str, true));
    }
}
