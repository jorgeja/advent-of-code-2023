use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    str::FromStr,
};

const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

#[derive(Debug, Default, Clone)]
struct Grid {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        if x > (self.width - 1) || y > (self.height - 1) {
            return 0;
        }

        let i = y * self.width + x;
        self.data[i]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        let i = y * self.width + x;
        if let Some(data) = self.data.get_mut(i) {
            *data = val;
        }
    }
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        let mut width = 0;
        for line in input.lines() {
            width = width.max(line.len());
            grid.data.extend(line.as_bytes().iter().copied());
            grid.height += 1;
        }
        grid.width = width;
        Ok(grid)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("W: {} H: {}\n", self.width, self.height))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let data = self.get(x, y);
                f.write_char(data as char)?;
            }

            f.write_char('\n')?;
        }
        Ok(())
    }
}

type Pos = (usize, usize);
type Dir = (isize, isize);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const RIGHT: Dir = (1, 0);
const LEFT: Dir = (-1, 0);
const NONE: Dir = (0, 0);

fn move_in_dir(pos: Pos, dir: Dir) -> (Pos, Dir) {
    (
        (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        ),
        dir,
    )
}

fn energize_grid(start: (Pos, Dir), grid: &Grid) -> u32 {
    let mut energized_cells = HashSet::<Pos>::new();
    let mut moves = HashSet::<(Pos, Dir)>::new();
    let mut stack = Vec::new();
    //let mut visited_grid = grid.clone();
    stack.push(start);
    while let Some((pos, dir)) = stack.pop() {
        if moves.contains(&(pos, dir)) {
            continue;
        } else {
            moves.insert((pos, dir));
        }

        let cell = grid.get(pos.0, pos.1);
        if cell > 0 {
            energized_cells.insert(pos);
        } else {
            continue;
        }

        let (first, second) = next_move(cell, dir);
        if first != NONE {
            stack.push(move_in_dir(pos, first))
        }
        if second != NONE {
            stack.push(move_in_dir(pos, second))
        }
    }

    energized_cells.len() as u32
}

fn next_move(cell: u8, dir: Dir) -> (Dir, Dir) {
    match cell {
        b'\\' => match dir {
            UP => (LEFT, NONE),
            DOWN => (RIGHT, NONE),
            LEFT => (UP, NONE),
            RIGHT => (DOWN, NONE),
            _ => unreachable!(),
        },
        b'/' => match dir {
            UP => (RIGHT, NONE),
            DOWN => (LEFT, NONE),
            LEFT => (DOWN, NONE),
            RIGHT => (UP, NONE),
            _ => unreachable!(),
        },
        b'|' => match dir {
            UP => (UP, NONE),
            DOWN => (DOWN, NONE),
            LEFT => (DOWN, UP),
            RIGHT => (DOWN, UP),
            _ => unreachable!(),
        },
        b'-' => match dir {
            UP => (LEFT, RIGHT),
            DOWN => (LEFT, RIGHT),
            LEFT => (LEFT, NONE),
            RIGHT => (RIGHT, NONE),
            _ => unreachable!(),
        },
        b'.' => (dir, NONE),
        _ => (NONE, NONE),
    }
}

fn recursive_energize(
    movement: (Pos, Dir),
    grid: &Grid,
    visited_grid: &mut Grid,
    visited: &mut HashSet<Pos>,
    cache: &mut HashMap<(Pos, Dir), u32>,
) -> u32 {
    if let Some(energized_tiles) = cache.get(&movement) {
        if *energized_tiles == 0 {
            println!("Loop! {movement:?}");
        }
        return *energized_tiles;
    }

    let (pos, dir) = movement;

    let cell = grid.get(pos.0, pos.1);
    if cell > 0 {
        cache.insert(movement, 0);
        let mut result = if visited.contains(&pos) {
            0
        } else {
            visited.insert(pos);
            1
        };

        visited_grid.set(pos.0, pos.1, b'X');
        println!("Visited:{result} {}, {visited_grid}", cell as char);
        visited_grid.set(pos.0, pos.1, b'#');

        let (first, second) = next_move(cell, dir);
        if first != NONE {
            result +=
                recursive_energize(move_in_dir(pos, first), grid, visited_grid, visited, cache);
        }
        if second != NONE {
            result +=
                recursive_energize(move_in_dir(pos, second), grid, visited_grid, visited, cache);
        }

        cache.insert(movement, result);
        result
    } else {
        0
    }
}

fn solve_part1(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    println!("{grid}");
    energize_grid(((0, 0), (1, 0)), &grid)
}

fn solve_part2(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    let mut max_visits = 0;
    println!("Checking From TOP");
    for x in 0..grid.width {
        max_visits = max_visits.max(energize_grid(((x, 0), DOWN), &grid));
    }
    println!("Checking From Bottom");
    for x in 0..grid.width {
        max_visits = max_visits.max(energize_grid(((x, grid.height - 1), UP), &grid));
    }

    println!("Checking From Left");
    for y in 0..grid.height {
        max_visits = max_visits.max(energize_grid(((0, y), RIGHT), &grid));
    }
    println!("Checking From Right");
    for y in 0..grid.height {
        max_visits = max_visits.max(energize_grid(((grid.width - 1, y), LEFT), &grid));
    }

    max_visits
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day16_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 46)
    }

    #[test]
    fn day16_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 16)?;
        let res = solve_part1(&input);
        println!("day16 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day16_part1_recursive() {
        let grid = Grid::from_str(EXAMPLE).unwrap();
        let mut v_grid = grid.clone();
        let mut visited = HashSet::new();
        let mut cache = HashMap::new();
        let start = ((0, 0), (1, 0));
        let res = recursive_energize(start, &grid, &mut v_grid, &mut visited, &mut cache);
        println!("{res} should be 46");
        //assert_eq!(res, 46);
        // 2
        let mut v_grid = grid.clone();
        let mut visited = HashSet::new();
        let mut cache = HashMap::new();
        let start = ((3, 0), (0, 1));
        let res = recursive_energize(start, &grid, &mut v_grid, &mut visited, &mut cache);
        println!("{res} should be 51");
        assert_eq!(res, 51);
    }

    #[test]
    fn day16_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 51);
    }

    #[test]
    fn day16_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 16)?;
        let res = solve_part2(&input);
        println!("day16 Part2 Result: {res}");
        Ok(())
    }
}
