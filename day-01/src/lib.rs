pub fn process_part1(input_str: &str) -> u32 {
    let mut sum = 0;
    for line in input_str.lines() {
        sum += get_first_and_last_digit(line, false);
    }
    sum
}

static ALLOWED_DIGIT_STRINGS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

pub fn process_part2(input_str: &str) -> u32 {
    let mut sum = 0;
    for line in input_str.lines() {
        sum += get_first_and_last_digit(line, true);
    }
    sum
}

fn get_first_and_last_digit(line: &str, part_2: bool) -> u32 {
    let mut first_pointer: usize = 0;
    let mut last_pointer: usize = line.len() - 1;

    let mut first_digit: Option<char> = None;
    let mut second_digit: Option<char> = None;

    while (first_digit.is_none() || second_digit.is_none()) && last_pointer >= first_pointer {
        if first_digit.is_none() {
            match line.chars().nth(first_pointer) {
                Some(first_pointer_value) => {
                    if first_pointer_value.is_digit(10) {
                        first_digit = Some(first_pointer_value);
                    }
                    else if part_2 {
                        match allowed_digit_starts_at_index(line, first_pointer) {
                            Some(allowed_digit) => {
                                first_digit = Some(allowed_digit);
                            }
                            None => {
                                first_pointer += 1;
                            }
                        }
                    }
                    else {
                        first_pointer += 1;
                    }
                }
                None => {} 
            }
        }

        if second_digit.is_none() {
            match line.chars().nth(last_pointer) {
                Some(last_pointer_value) => {
                    if last_pointer_value.is_digit(10) {
                        second_digit = Some(last_pointer_value);
                    }
                    else if part_2 {
                        match allowed_digit_ends_at_index(line, last_pointer) {
                            Some(allowed_digit) => {second_digit = Some(allowed_digit);}
                            None => {last_pointer -= 1;}
                        }
                    }
                    else {
                        last_pointer -= 1;
                    }
                }
                None => {}
            }
        }
    }

    merge_digits(first_digit, second_digit)
}

fn allowed_digit_starts_at_index(line: &str, index: usize) -> Option<char> {
    for allowed_digit in ALLOWED_DIGIT_STRINGS {
        let last_index = index + allowed_digit.len();
        if last_index <= line.len() && allowed_digit.eq(&line[index..last_index]) {
            return convert_allowed_digit_to_char(allowed_digit);
        }
    }
    None
}

fn allowed_digit_ends_at_index(line: &str, index: usize) -> Option<char> {
    for allowed_digit in ALLOWED_DIGIT_STRINGS {
        if allowed_digit.len() > index {
            continue;
        }
        let first_index = index - allowed_digit.len() + 1;
        if allowed_digit.eq(&line[first_index..index + 1]) {
            return convert_allowed_digit_to_char(allowed_digit);
        }
    }
    None
}

fn merge_digits(first_digit: Option<char>, second_digit: Option<char>) -> u32 {
    let first_digit: char = first_digit.expect("unable to unwrap first digit");
    let second_digit: char = second_digit.expect("unable to unwrap second digit");

    10 * first_digit.to_digit(10).unwrap() + second_digit.to_digit(10).unwrap()
}

fn convert_allowed_digit_to_char(allowed_digit: &str) -> Option<char> {
    match allowed_digit {
        "zero" => Some('0'),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None 
    }
}


#[cfg(test)]
mod test { 
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_input_part1() {
        let input_str = read_to_string("example_input.txt").expect("unable to read the example_input file");
        assert_eq!(142, process_part1(&input_str));
    }

    #[test]
    fn test_example_input_part2() {
        let input_str = read_to_string("example_input_part2.txt").expect("unable to read the example_input file");
        assert_eq!(281, process_part2(&input_str));
    }
}
