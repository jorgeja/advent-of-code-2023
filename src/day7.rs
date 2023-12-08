use std::{collections::HashSet, error::Error, num, str::FromStr, cmp::Ordering};
const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

const EXAMPLE_2: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Card(char);

impl Card {
    fn strength(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => 0
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards : [Card; 5],
    bid : u32,
    kind : HandType,
    old_kind : HandType
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind == other.kind {
            for i in 0..5 {
                let comp = self.cards[i].partial_cmp(&other.cards[i]).unwrap();
                if comp != Ordering::Equal {
                    return comp;
                } 
            }
            Ordering::Equal
        } else {
            self.kind.cmp(&other.kind)
        }
    }
}

impl Hand {
    fn determine_hand_type(&mut self, include_jokers: bool) {
        let mut occurances = [0; 5];
        let mut jokers = 0;
        for i in 0..self.cards.len() {
            if self.cards[i].0 == 'J' {
                jokers += 1;
                continue;
            }
            if occurances[i] == -1 {continue};

            occurances[i] = 1;
            for j in 0..self.cards.len() {
                if i == j || occurances[j] == -1 { continue };
                if self.cards[i] == self.cards[j] {
                   occurances[i] += 1;
                   occurances[j] = -1; 
                }
            }
        }

        for count in occurances.iter() {
            use HandType::*;
            let new_kind = match count {
                2 => OnePair,
                3 => Three,
                4 => Four,
                5 => Five,
                _ => { continue }
            };
            match (new_kind, self.kind) {
                (OnePair, OnePair) => self.kind = TwoPair,
                (OnePair, Three) => self.kind = FullHouse,
                (Three, OnePair) => self.kind = FullHouse,
                (new_kind, old_kind) if new_kind > old_kind => self.kind = new_kind,
                _ => {},
            }
        }

        self.old_kind = self.kind;
        if include_jokers && jokers > 0 {
            use HandType::*;
            match self.kind {
                HighCard => {
                    match jokers {
                        1 => self.kind = OnePair,
                        2 => self.kind = Three,
                        3 => self.kind = Four,
                        4 => self.kind = Five,
                        _ => {}
                    }
                }, 
                OnePair => {
                    match jokers {
                        1 => self.kind = Three,
                        2 => self.kind = Four,
                        3 => self.kind = Five,
                        _ => {}
                    }
                },
                TwoPair => self.kind = FullHouse,
                Three => {
                    match jokers {
                        1 => self.kind = Four,
                        2 => self.kind = Five,
                        _ => {}
                    }
                },
                Four => {
                    self.kind = Five
                },
                _ => {}
            }
        }
    }
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = Hand {
            cards : [Card::default(); 5],
            bid: 0,
            kind : HandType::HighCard,
            old_kind : HandType::HighCard
        };
        let mut parts = s.split(' ');
        let cards = parts.next().ok_or("missing cards")?;
        for (i, card) in cards.chars().enumerate() {
            if i >= 5 {
                break;
            }

            hand.cards[i] = Card(card);
        }

        hand.bid = parts.next().ok_or("missing bid")?.parse()?;

        Ok(hand)
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().filter_map(|l| match Hand::from_str(l) {
        Ok(h) => Some(h),
        Err(e) => {
            println!("Bad Hand: {:?}", e);
            None
        }
    }).collect()
}

fn solve_part1(input: &str) -> u32 {
    let mut hands = parse(input);
    hands.iter_mut().for_each(|h| h.determine_hand_type(false));
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i+1) as u32 * h.bid).sum()
}

fn solve_part2(input: &str) -> u32 {
    let mut hands = parse(input);
    hands.iter_mut().for_each(|h| h.determine_hand_type(true));
    hands.sort();

    for (i, h) in hands.iter().enumerate() {
        if h.kind != h.old_kind {
            println!("[{i}] {:?}", h);   
        }
    }
    hands.iter().enumerate().map(|(i, h)| (i+1) as u32 * h.bid).sum()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day7_part1_test() {
        let res = solve_part1(EXAMPLE);
        assert_eq!(res, 6440)
    }

    #[test]
    fn day7_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 7)?;
        let res = solve_part1(&input);
        println!("day7 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day7_part2_test() {
        let res = solve_part2(EXAMPLE_2);
        assert_eq!(res, 5905);
    }

    #[test]
    fn day7_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 7)?;
        let res = solve_part2(&input);
        println!("day7 Part2 Result: {res}");
        Ok(())
    }
}
