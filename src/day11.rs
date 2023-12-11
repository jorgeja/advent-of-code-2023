use std::collections::{HashMap, HashSet};

const EXAMPLE : &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

fn parse(input: &str) -> (Vec<(i64, i64)>, HashSet<usize>, HashSet<usize>){
    let mut filled_rows = HashSet::new();
    let mut filled_columns = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    let galaxies = input.lines().enumerate().map(|(row, line)| {
        let mut galaxies = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((col as i64, row as i64));
                filled_rows.insert(row);
                filled_columns.insert(col);
            }
            width = width.max(col + 1);
        }
        height = height.max(row + 1);
        galaxies
    }).flatten().collect::<Vec<_>>();

    (galaxies, filled_columns, filled_rows)
}

fn find_galaxy_distance(empty_size: i64, mut galaxies: Vec<(i64, i64)>, filled_columns: &HashSet<usize>, filled_rows: &HashSet<usize>) -> i64 {
    let empty_size = empty_size - 1;
    for g in galaxies.iter_mut() {
        for c in 0..g.0 {
            if !filled_columns.contains(&(c as usize)) {
                g.0 += empty_size;
            }
        }
        for r in 0..g.1 {
            if !filled_rows.contains(&(r as usize)) {
                g.1 += empty_size;
            }
        }
    }

    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i == j { continue }

            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let dx = (g1.0 - g2.0).abs();
            let dy = (g1.1 - g2.1).abs();
            let distance = dx + dy;
            distances.push(distance);
        }
    }
    
    distances.iter().sum()
}

fn solve_part1(input: &str) -> i64 {
    let (galaxies, filled_columns, filled_rows) = parse(input);
    find_galaxy_distance(2, galaxies, &filled_columns, &filled_rows)
}

fn solve_part2(empty_distance: i64, input: &str) -> i64 {
    let (galaxies, filled_columns, filled_rows) = parse(input);
    find_galaxy_distance(empty_distance, galaxies, &filled_columns, &filled_rows)
}

#[cfg(test)]
mod tests {
    use std::{error::Error, f64::consts::E};

    use crate::get_input;

    use super::*;

    #[test]
    fn day11_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 374);
    }

    #[test]
    fn day11_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 11)?;
        let res = solve_part1(&input);
        println!("day11 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day11_part2_test() {
        let res = solve_part2(10, EXAMPLE);
        assert_eq!(res, 1030);
        let res = solve_part2(100, EXAMPLE);
        assert_eq!(res, 8410);
    }

    #[test]
    fn day11_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 11)?;
        let res = solve_part2(1_000_000, &input);
        println!("day11 Part2 Result: {res}");
        Ok(())
    }
}
