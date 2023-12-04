use std::collections::HashMap;

use crate::day1;

const EXAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

#[derive(Debug, Clone, Copy)]
struct Number {
    number: i32,
    pos: (usize, usize),
}

fn to_coords(pos: usize, width: usize) -> (usize, usize) {
    if pos == 0 {
        return (0, 0);
    }
    let x = pos % width;
    let y = pos / width;
    (x, y)
}

struct Symbol {
    sym: u8,
    visits: usize,
    gear_ratio: i32,
}

fn parse(input: &str) -> (Vec<Number>, HashMap<(usize, usize), Symbol>, usize) {
    let first_line = input
        .lines()
        .next()
        .expect("missing line in input")
        .as_bytes();
    let width = first_line.len();

    let mut symbols = HashMap::new();
    let mut tokens = Vec::default();
    let mut pending_num_token: Option<Number> = None;
    let mut i = 0;
    for byte in input.bytes() {
        if b'0' <= byte && byte <= b'9' {
            let num = (byte - b'0') as i32;

            if let Some(num_token) = &mut pending_num_token {
                num_token.number = (num_token.number * 10) + num;
            } else {
                pending_num_token = Some(Number {
                    number: num,
                    pos: to_coords(i, width),
                });
            }
        } else {
            if let Some(num_token) = pending_num_token.take() {
                tokens.push(num_token);
            }

            if byte != b'.' && byte != b'\n' {
                symbols.insert(
                    to_coords(i, width),
                    Symbol {
                        sym: byte,
                        visits: 0,
                        gear_ratio: 1,
                    },
                );
            }
        }
        if byte != b'\n' {
            i += 1;
        }
    }
    (tokens, symbols, width)
}

fn get_num_width(num: i32) -> usize {
    let mut width = 0;
    let mut tmp = num;
    while tmp != 0 {
        tmp /= 10;
        width += 1;
    }
    width
}

fn valid_number(num: Number, symbols: &HashMap<(usize, usize), Symbol>, width: usize) -> bool {
    let num_width = get_num_width(num.number);
    let pos = num.pos;

    // Left
    if pos.0 > 0 && symbols.contains_key(&(pos.0 - 1, pos.1)) {
        return true;
    }

    // Right
    if pos.0 < width && symbols.contains_key(&(pos.0 + num_width, pos.1)) {
        return true;
    }

    // Above
    let start_pos = if pos.0 == 0 { 0 } else { pos.0 - 1 };
    if pos.1 > 0 {
        for i in start_pos..=(pos.0 + num_width).min(width) {
            if symbols.contains_key(&(i, pos.1 - 1)) {
                return true;
            }
        }
    }
    // Below
    for i in start_pos..=(pos.0 + num_width).min(width) {
        if symbols.contains_key(&(i, pos.1 + 1)) {
            return true;
        }
    }

    false
}

fn solve_part1(input: &str) -> i32 {
    let (numbers, symbols, width) = parse(input);
    //println!("{:?}", numbers);
    //println!("{:?}", symbols);
    numbers
        .iter()
        .filter(|n| valid_number(**n, &symbols, width))
        .map(|n| n.number)
        .sum()
}

fn gear_ratio(num: Number, symbols: &mut HashMap<(usize, usize), Symbol>, width: usize) {
    let num_width = get_num_width(num.number);
    let pos = num.pos;

    // Left
    if pos.0 > 0 {
        if let Some(sym) = symbols.get_mut(&(pos.0 - 1, pos.1)) {
            sym.visits += 1;
            sym.gear_ratio *= num.number;
        }
    }

    // Right
    if pos.0 < width {
        if let Some(sym) = symbols.get_mut(&(pos.0 + num_width, pos.1)) {
            sym.visits += 1;
            sym.gear_ratio *= num.number;
        }
    }

    // Above
    let start_pos = if pos.0 == 0 { 0 } else { pos.0 - 1 };
    if pos.1 > 0 {
        for i in start_pos..=(pos.0 + num_width).min(width) {
            if let Some(sym) = symbols.get_mut(&(i, pos.1 - 1)) {
                sym.visits += 1;
                sym.gear_ratio *= num.number;
            }
        }
    }
    // Below
    for i in start_pos..=(pos.0 + num_width).min(width) {
        if let Some(sym) = symbols.get_mut(&(i, pos.1 + 1)) {
            sym.visits += 1;
            sym.gear_ratio *= num.number;
        }
    }
}

fn solve_part2(input: &str) -> i32 {
    let (numbers, mut symbols, width) = parse(input);
    numbers
        .iter()
        .for_each(|n| gear_ratio(*n, &mut symbols, width));
    symbols
        .iter()
        .filter(|(_, s)| s.sym == b'*' && s.visits == 2)
        .map(|(_, s)| s.gear_ratio)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn test_num_width() {
        assert_eq!(get_num_width(123), 3);
        assert_eq!(get_num_width(12), 2);
        assert_eq!(get_num_width(1), 1);
    }

    #[test]
    fn day3_part1_test() {
        let res = solve_part1(EXAMPLE);
        assert_eq!(res, 4361);
    }

    #[test]
    fn day3_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 3)?;
        let res = solve_part1(&input);
        println!("Day3 Part1 Result: {res}");
        Ok(())
    }
    #[test]
    fn day3_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 467835);
    }

    #[test]
    fn day3_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 3)?;
        let res = solve_part2(&input);
        println!("Day3 Part2 Result: {res}");
        Ok(())
    }
}
