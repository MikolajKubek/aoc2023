use lexer::lexer::{Lexer, Token, TokenType};

mod lexer;

pub fn process_part1(input_str: &str, part2: bool) -> u32 {
    let mut sum_ids = 0;
    let mut sum_powers = 0;

    for line in input_str.lines() {
        let mut lexer: Lexer = Lexer::new(line);
        let mut token: Token = lexer.next_token();

        let mut num_stack: Vec<u32> = Vec::new();
        let mut game: Game = Game::new();
        let mut round: Round = Round::new();

        while token.t_type != TokenType::EOF {
            match token.t_type {
                TokenType::Game => {}
                TokenType::Number => {
                    let numeric = str::parse::<u32>(&token.literal).unwrap();
                    if game.id.is_none() {
                        game.id = Some(numeric);
                    } else {
                        num_stack.push(numeric);
                    }
                }
                TokenType::Colon => {
                    round = Round::new();
                }
                TokenType::Comma => {}
                TokenType::SEMICOLON => {
                    game.rounds.push(round);
                    round = Round::new();
                }
                TokenType::ColorRed => {
                    round.red = num_stack.pop().unwrap();
                }
                TokenType::ColorGreen => {
                    round.green = num_stack.pop().unwrap();
                }
                TokenType::ColorBlue => {
                    round.blue = num_stack.pop().unwrap();
                }
                TokenType::EOF => {}
                TokenType::ILLEGAL => panic!("illegal token"),
            }
            token = lexer.next_token();
        }
        game.rounds.push(round);
        if game.is_valid(12, 13, 14) {
            sum_ids += game.id.unwrap();
        }
        sum_powers += game.find_max_color_for_one_round_power();
    }

    if part2 {
        sum_powers
    }
    else {
        sum_ids
    }
}

#[derive(Debug)]
struct Game {
    id: Option<u32>,
    rounds: Vec<Round>,
}

impl Game {
    fn new() -> Self {
        Game {
            id: None,
            rounds: Vec::new(),
        }
    }

    fn is_valid(&self, red_bound: u32, green_bound: u32, blue_bound: u32) -> bool {
        for round in self.rounds.iter() {
            if round.red > red_bound || round.green > green_bound || round.blue > blue_bound {
                return false;
            }
        }
        true
    }

    fn find_max_color_for_one_round_power(&self) -> u32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in self.rounds.iter() {
            max_red = max_red.max(round.red);
            max_green = max_green.max(round.green);
            max_blue = max_blue.max(round.blue);
        }

        max_red * max_green * max_blue
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn new() -> Self {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_input() {
        let input_str =
            read_to_string("example_input.txt").expect("unable to read the example_input file");
        assert_eq!(8, process_part1(&input_str, false));
    }

    #[test]
    fn test_example_input_part_2() {
        let input_str =
            read_to_string("example_input.txt").expect("unable to read the example_input file");
        assert_eq!(2286, process_part1(&input_str, true));
    }
}
