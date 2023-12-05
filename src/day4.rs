use std::{error::Error, num, str::FromStr};

const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    fn num_matches(&self) -> u32 {
        let mut num_matches = 0;
        for win_v in self.winning_numbers.iter() {
            if self.numbers.contains(win_v) {
                num_matches += 1;
            }
        }
        num_matches
    }

    fn value(&self) -> i32 {
        let num_wins = self.num_matches();
        if num_wins > 0 {
            2i32.pow(num_wins - 1)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card = Card {
            id: 0,
            winning_numbers: Vec::default(),
            numbers: Vec::default(),
        };

        let mut card_split = s.split(": ");
        let card_id = card_split
            .next()
            .ok_or("bad format")?
            .strip_prefix("Card ")
            .ok_or("bad format")?;

        match card_id.parse::<i32>() {
            Ok(id) => card.id = id,
            Err(_) => {
                format!("Could not parse to int: '{}'", card_id);
            }
        }

        let nums = card_split.next().ok_or("missing number info")?;
        let mut num_split = nums.split(" | ");
        let winning_numbers_split = num_split
            .next()
            .ok_or("missing winning numbers")?
            .split(" ");
        card.winning_numbers = winning_numbers_split
            .map(str::parse::<i32>)
            .filter_map(|res| res.ok())
            .collect();
        let numbers_split = num_split.next().ok_or("missing numbers")?.split(" ");
        card.numbers = numbers_split
            .map(str::parse::<i32>)
            .filter_map(|res| res.ok())
            .collect();
        Ok(card)
    }
}

fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(Card::from_str)
        .filter_map(|c| match c {
            Ok(c) => Some(c),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        })
        .map(|c| c.value())
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let mut card_matches = input
        .lines()
        .map(Card::from_str)
        .filter_map(|c| c.ok())
        .map(|c| (c.num_matches(), 1u32))
        .collect::<Vec<_>>();

    let mut num_cards = 0;
    for i in 0..card_matches.len() {
        let (matches, instances) = card_matches[i];
        if matches > 0 {
            for _ in 1..=instances {
                for next in 1..=matches {
                    let next_card = i + (next as usize);
                    if next_card < card_matches.len() {
                        card_matches[next_card].1 += 1;
                    }
                }
            }
        }
        num_cards += instances;
    }

    num_cards
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day4_part1_test() {
        let res = solve_part1(EXAMPLE);
        assert_eq!(res, 13);
    }

    #[test]
    fn day4_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 4)?;
        let res = solve_part1(&input);
        println!("Day4 Part1 Result: {res}");
        Ok(())
    }
    #[test]
    fn day4_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 30);
    }

    #[test]
    fn day4_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 4)?;
        let res = solve_part2(&input);
        println!("Day4 Part2 Result: {res}");
        Ok(())
    }
}
