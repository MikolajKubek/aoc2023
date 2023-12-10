use std::{collections::HashMap, panic, cmp::Ordering, iter::zip};

const JOKER_CHAR: char = 'J';

#[derive(Debug, Eq)]
pub struct Hand {
    value: String,
    pub bid: u32,
    hand_type: Type,
    include_jokers: bool,
}

impl Hand {
    pub fn new(value: String, bid: u32, include_jokers: bool) -> Self {
        let hand_type = deduce_type(&value, include_jokers);
        Self {
            value,
            bid,
            hand_type,
            include_jokers,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (self_char, other_char) in zip(self.value.chars(), other.value.chars()) {
                if self_char != other_char {
                    return get_card_strength(self_char, self.include_jokers)
                        .cmp(&get_card_strength(other_char, self.include_jokers));
                }
            }
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
       self.hand_type.eq(&other.hand_type) && self.value.eq(&other.value)
    }
}

fn create_char_counts_map(value: &str, include_jokers: bool) -> HashMap<char, u32> {
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    let mut current_max = 0;
    let mut max_key: Option<char> = None;
    for ch in value.chars() {
        let mut current_value = 1;
        if let Some(value) = char_counts.get_mut(&ch) {
            *value += 1;
            current_value = *value;
        }
        else {
            char_counts.insert(ch, 1);
        }
        if ch != JOKER_CHAR && current_value > current_max {
            max_key = Some(ch);
            current_max = current_value;
        }
    }


    if include_jokers && char_counts.contains_key(&JOKER_CHAR) {
        let joker_count: u32 = *char_counts.get(&JOKER_CHAR).unwrap();
        
        if max_key.is_some() {
            *char_counts.get_mut(&max_key.unwrap()).unwrap() += joker_count;
            char_counts.remove(&JOKER_CHAR);
        }
    }

    char_counts
}

fn deduce_type(value: &str, include_jokers: bool) -> Type {
    let char_counts = create_char_counts_map(value, include_jokers);
    match char_counts.len() {
        1 => Type::FiveOfAKind,
        2 => {
            let value = char_counts.values().next().unwrap();
            if *value == 4 || *value == 1 {
                Type::FourOfAKind
            }
            else {
                Type::FullHouse
            }
        }
        3 => {
            for value in char_counts.values() {
                if *value == 3 {
                    return Type::ThreeOfAKind
                }
            }
            Type::TwoPair
        }
        4 => Type::OnePair,
        5 => Type::HighCard,
        _ => {panic!("type not covered")}
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_card_strength(ch: char, include_jokers: bool) -> u32 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            match include_jokers {
                true => 1,
                false => 11,
            }
        },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => {panic!("invalid card label")}
    }
}
