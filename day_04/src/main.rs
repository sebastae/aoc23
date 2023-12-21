use std::{collections::{HashSet, HashMap}, str::FromStr};

type AocError = String;

#[derive(Debug, PartialEq)]
struct CardNumber {
    number: u32,
    index: usize,
}

impl CardNumber {
    fn new(number: u32, index: usize) -> Self {
        CardNumber { number, index }
    }

    fn vec_from_str(s: &str) -> Result<Vec<Self>, AocError> {
        s.trim()
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, n)| {
                n.trim()
                    .parse::<u32>()
                    .map_err(|e| format!("parse to CardNumber ({n}): {}", e.to_string()))
                    .and_then(|n| Ok(CardNumber::new(n, i)))
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    number: u32,
    winning_numbers: Vec<CardNumber>,
    card_numbers: Vec<CardNumber>,
}

impl FromStr for Card {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (title, numbers) = s.split_once(":").ok_or(AocError::from("split line"))?;
        let (_, card_number) = title
            .trim()
            .split_once(" ")
            .ok_or(AocError::from("split title"))?;

        let card_number = card_number
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("parse card number ({card_number}): {e}"))?;

        let (winning_numbers, your_numbers) = numbers
            .trim()
            .split_once("|")
            .ok_or(AocError::from("split numbers"))?;

        let winning_numbers = CardNumber::vec_from_str(winning_numbers)?;
        let your_numbers = CardNumber::vec_from_str(your_numbers)?;

        Ok(Card {
            number: card_number,
            winning_numbers: winning_numbers,
            card_numbers: your_numbers,
        })
    }
}

impl Card {
    fn get_matching_numbers(self: &Self) -> Vec<u32> {
        let mut winning_numbers: HashSet<u32> = HashSet::new();
        self.winning_numbers.iter().for_each(|n| {
            winning_numbers.insert(n.number);
        });

        self.card_numbers
            .iter()
            .filter(|n| winning_numbers.contains(&n.number))
            .map(|n| n.number)
            .collect()
    }

    fn get_points(self: &Self) -> u32 {
        let mut winning_numbers: HashSet<u32> = HashSet::new();
        self.winning_numbers.iter().for_each(|n| {
            winning_numbers.insert(n.number);
        });

        let num_winning_numbers = self.get_matching_numbers().len();
        if num_winning_numbers > 0 {
            2u32.pow(num_winning_numbers as u32 - 1u32)
        } else {
            0
        }
    }
}

fn calculate_won_cards(cards: Vec<Card>) -> u32 {

    let mut num_cards: HashMap<u32, u32> = HashMap::from_iter(cards.iter().map(|c| (c.number, 1)));

    for card in cards {
        let num_cards_won = card.get_matching_numbers().len() as u32;
        let won_cards = (card.number + 1)..(card.number + 1 + num_cards_won);

        let num_current_card = {num_cards.get(&card.number).unwrap_or(&1).clone()};

        for crd in won_cards {
            let current_num = {num_cards.get(&crd).unwrap_or(&1)};
            num_cards.insert(crd, *current_num + num_current_card);
        }

    }


    num_cards.values().sum()
}

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let cards = INPUT
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, AocError>>()
        .unwrap();

    println!(
        "Part 1: {}",
        cards.iter().map(Card::get_points).sum::<u32>()
    );

    println!("Part 2: {}", calculate_won_cards(cards));
}

#[cfg(test)]
mod test {

    use crate::*;
    use test_case::test_case;

    #[test]
    fn it_parses_line() {
        const INPUT: &str = "Card 1: 1 2 3 | 3 4 5";

        let expect = Card {
            number: 1,
            winning_numbers: vec![
                CardNumber::new(1, 0),
                CardNumber::new(2, 1),
                CardNumber::new(3, 2),
            ],
            card_numbers: vec![
                CardNumber::new(3, 0),
                CardNumber::new(4, 1),
                CardNumber::new(5, 2),
            ],
        };

        assert_eq!(INPUT.parse::<Card>().unwrap(), expect);
    }

    #[test]
    fn it_parses_line_with_multiple_whitespaces() {
        const INPUT: &str = "Card 1: 1 2 3 | 12 13  4";

        let expect = Card {
            number: 1,
            winning_numbers: vec![
                CardNumber::new(1, 0),
                CardNumber::new(2, 1),
                CardNumber::new(3, 2),
            ],
            card_numbers: vec![
                CardNumber::new(12, 0),
                CardNumber::new(13, 1),
                CardNumber::new(4, 2),
            ],
        };

        assert_eq!(Card::from_str(INPUT), Ok(expect));
    }

    #[test_case(vec![1, 2], vec![0], 0)]
    #[test_case(vec![1, 2], vec![1, 3, 4, 5], 1)]
    #[test_case(vec![1, 2], vec![1, 2], 2)]
    #[test_case(vec![1, 2], vec![1, 2, 2, 2, 3], 8)]
    #[test_case(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53], 8)]
    #[test_case(vec![13, 32, 20, 16, 61, ], vec![ 61, 30, 68, 82, 17, 32, 24, 19], 2)]
    #[test_case(vec![87, 83, 26, 28, 32 ], vec![ 88, 30, 70, 12, 93, 22, 82, 36], 0)]
    fn it_calculates_points(winning: Vec<u32>, nums: Vec<u32>, points: u32) {
        let card = Card {
            number: 0,
            winning_numbers: winning
                .iter()
                .enumerate()
                .map(|(i, n)| CardNumber::new(*n, i))
                .collect(),
            card_numbers: nums
                .iter()
                .enumerate()
                .map(|(i, n)| CardNumber::new(*n, i))
                .collect(),
        };

        assert_eq!(card.get_points(), points)
    }

    const EXAMPLE_INPUT: &str = include_str!("./example.txt");

    #[test]
    fn it_passes_part_1_example() {
        let cards = EXAMPLE_INPUT
            .lines()
            .map(Card::from_str)
            .collect::<Result<Vec<Card>, AocError>>()
            .unwrap();
        assert_eq!(cards.iter().map(Card::get_points).sum::<u32>(), 13);
    }

    #[test]
    fn it_passes_part_2_example() {
        let cards = EXAMPLE_INPUT
            .lines()
            .map(Card::from_str)
            .collect::<Result<Vec<Card>, AocError>>()
            .unwrap();

        assert_eq!(calculate_won_cards(cards), 30);
    }
}
