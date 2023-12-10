use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Debug, Write},
    num,
    slice::SliceIndex,
    str::FromStr, default,
};
const EXAMPLE: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

const EXAMPLE_2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

const EXAMPLE_PART2_LARGE: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

const EXAMPLE_PART2: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

const EXAMPLE_PART2_SMALL : &str = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

const EXAMPLE_PART2_SMALLEST : &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;


struct Field {
    field: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Field {
    fn new(field: Vec<Vec<u8>>) -> Self {
        let height = field.len();
        let width = field[0].len();
        Self {
            field,
            height,
            width,
        }
    }
    fn index(&self, pos: Pos) -> Option<u8> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 as usize >= self.width || pos.1 as usize >= self.height {
            return None;
        }
        Some(self.field[pos.1 as usize][pos.0 as usize])
    }
}
impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.field {
            for c in l {
                f.write_char(*c as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

type Pos = (i32, i32);

fn parse(input: &str) -> (Pos, Field) {
    let mut start_pos = None;
    let field = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            if start_pos.is_none() {
                if let Some(col) = l.find('S') {
                    start_pos = Some((col as i32, row as i32));
                }
            }
            l.as_bytes().iter().copied().collect::<Vec<u8>>()
        })
        .collect();
    (start_pos.unwrap(), Field::new(field))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    #[default]
    Left,
    Up,
    Right,
    Down,
}

fn next_pos(pos: Pos, dir: Dir, pipe: u8) -> Option<(Pos, Dir)> {
    let next = match pipe {
        b'|' => match dir {
            Dir::Up => ((pos.0, pos.1 - 1), dir),
            Dir::Down => ((pos.0, pos.1 + 1), dir),
            _ => return None,
        },
        b'-' => match dir {
            Dir::Left => ((pos.0 - 1, pos.1), dir),
            Dir::Right => ((pos.0 + 1, pos.1), dir),
            _ => return None,
        },
        b'L' => match dir {
            Dir::Down => ((pos.0 + 1, pos.1), Dir::Right),
            Dir::Left => ((pos.0, pos.1 - 1), Dir::Up),
            _ => return None,
        },
        b'J' => match dir {
            Dir::Down => ((pos.0 - 1, pos.1), Dir::Left),
            Dir::Right => ((pos.0, pos.1 - 1), Dir::Up),
            _ => return None,
        },
        b'7' => match dir {
            Dir::Up => ((pos.0 - 1, pos.1), Dir::Left),
            Dir::Right => ((pos.0, pos.1 + 1), Dir::Down),
            _ => return None,
        },
        b'F' => match dir {
            Dir::Up => ((pos.0 + 1, pos.1), Dir::Right),
            Dir::Left => ((pos.0, pos.1 + 1), Dir::Down),
            _ => return None,
        },
        _ => return None,
    };
    Some(next)
}

#[derive(Debug, Default)]
struct Loop {
    length: u32,
    start_dir: Dir,
    end_dir: Dir,
    pipes: Vec<(i32, i32)>,
}

fn traverse(start_pos: Pos, field: &Field) -> Loop {
    let mut current_loop = Loop::default();

    for (d_pos, start_dir) in [
        ((-1, 0), Dir::Left),
        ((1, 0), Dir::Right),
        ((0, -1), Dir::Up),
        ((0, 1), Dir::Down),
    ] {
        let mut pos = (start_pos.0 + d_pos.0, start_pos.1 + d_pos.1);
        
        if let Some(mut pipe) = field.index(pos) {
            let mut dir = start_dir;
            let mut steps = 1;
            let mut pipes = vec![start_pos, pos];
            while let Some((next_pos, next_dir)) = next_pos(pos, dir, pipe) {
                pos = next_pos;
                dir = next_dir;

                if let Some(next_pipe) = field.index(pos) {
                    steps += 1;
                    pipes.push(pos);
                    pipe = next_pipe;
                    if pipe == b'S' {
                        println!("Found a loop with length {steps} ");
                        break;
                    }
                } else {
                    break;
                }
            }
            if steps > current_loop.length {
                current_loop.start_dir = start_dir;
                current_loop.end_dir = dir;
                current_loop.length = steps;
                current_loop.pipes = pipes;
            }
        }
    }
    current_loop
}

fn substitute_start_pipe(looop: &Loop, field: &mut Field) {
    let start_pos = looop.pipes[0];
    let start_char = match (looop.start_dir, looop.end_dir) {
        (Dir::Down, Dir::Down) => b'|',
        (Dir::Up, Dir::Up) => b'|',
        (Dir::Left, Dir::Left) => b'-',
        (Dir::Right, Dir::Right) => b'-',
        (Dir::Down, Dir::Left) => b'F',
        (Dir::Down, Dir::Right) => b'7',
        (Dir::Up, Dir::Left) => b'L',
        (Dir::Up, Dir::Right) => b'J',
        (Dir::Left, Dir::Down) => b'J',
        (Dir::Left, Dir::Up) => b'7',
        (Dir::Right, Dir::Down) => b'L',
        (Dir::Right, Dir::Up) => b'F',
        _ => unreachable!() 
    };
    println!("Replacing startpos with {}", start_char as char);
    field.field[start_pos.1 as usize][start_pos.0 as usize] = start_char;
}

fn solve_part1(input: &str) -> u32 {
    let (start_pos, mut field) = parse(input);
    let found_loop = traverse(start_pos, &mut field);

    found_loop.length / 2
}

fn find_pools(looop: &Loop, field: &Field) -> u32 {
    let loop_coords = looop.pipes.iter().copied().collect::<HashSet<_>>();

    let mut pools = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for height in 0..field.height {
        for width in 0..field.width {
            let start_pos = (width as i32, height as i32);
            if visited.contains(&start_pos) || loop_coords.contains(&start_pos) {
                continue;
            };

            stack.clear();
            stack.push(start_pos);
            visited.insert(start_pos);
            let mut pool = Vec::new();
            let mut touches_outside = false;
            while let Some(pos) = stack.pop() {
                let mut loop_neighbours = Vec::new();
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let new_pos = (pos.0 + dx, pos.1 + dy);
                        if field.index(new_pos).is_none() {
                            touches_outside = true;
                            continue;
                        }

                        if loop_coords.contains(&new_pos) {
                            loop_neighbours.push(new_pos);
                            continue;
                        };

                        if !visited.contains(&new_pos) {
                            visited.insert(new_pos);
                            stack.push(new_pos);
                        }
                    }
                }
                pool.push((pos, loop_neighbours));
            }
            pools.push((touches_outside, pool));
        }
    }

    let outside_nodes = pools
        .iter()
        .filter_map(|pool| {
            if pool.0 {
                Some(pool.1.iter().map(|(pos, _)| *pos))
            } else {
                None
            }
        })
        .flatten()
        .collect::<HashSet<_>>();

    let maybe_inside_pools: Vec<Vec<((i32, i32), Vec<(i32, i32)>)>> = pools.iter().filter_map(|pool| if !pool.0 {Some(pool.1.clone())} else {None}).collect::<Vec<_>>();
    maybe_inside_pools
        .iter()
        .filter(|pool| {
            //println!("Checking if pool is inside:");
            let res = is_inside(&outside_nodes, &loop_coords, field, pool); 
            //println!("This pool is {}", if res {"Inside"} else {"Outside"});
            res
        })
        .map(|pool| pool.len() as u32)
        .sum()
}

const edge_point_offsets: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1)
];

const direction: [Dir; 8] = [
    Dir::Up,
    Dir::Up,
    Dir::Right,
    Dir::Right,
    Dir::Down,
    Dir::Down,
    Dir::Left,
    Dir::Left
];

fn is_inside(outside_nodes: &HashSet<(i32, i32)>, loop_coords: &HashSet<(i32, i32)>, field: &Field, pool: &Vec<((i32, i32), Vec<(i32, i32)>)>) -> bool {
    let mut start_points = Vec::new();
    for (point, _) in pool.iter().filter(|p| !p.1.is_empty()) {
        for (i, window) in edge_point_offsets.windows(2).enumerate() {
            let pos1 = (point.0 + window[0].0, point.1 + window[0].1);
            let pos2 = (point.0 + window[1].0, point.1 + window[1].1);
            if loop_coords.contains(&pos1) && loop_coords.contains(&pos2) {
                start_points.push((pos1, pos2, direction[i]));
                //println!("Possible crack in loop wall at {:?}", (pos1, pos2, direction[i]))
            }
        } 
    }

    for (pos1, pos2, dir) in start_points {
        if path_find(pos1, pos2, dir, outside_nodes, loop_coords, field) {
            return false;
        }
    }

    true
}
// Search for a way out, depth first
fn path_find(pos1: Pos, pos2: Pos, dir: Dir, outside_nodes: &HashSet<(i32, i32)>, loop_coords: &HashSet<(i32, i32)>, field: &Field) -> bool {
    let mut stack = vec![(pos1, pos2, dir)];
    let mut visited = HashSet::new();
    while let Some((pos1, pos2, dir)) = stack.pop() {
        if outside_nodes.contains(&pos1) || outside_nodes.contains(&pos2) {
            println!("nodes {:?} {:?} are part of outside!", pos1, pos2);
            return true;
        } 
        if visited.contains(&(pos1, pos2, dir)) {
            continue;
        } else {
            visited.insert((pos1, pos2, dir));
        }

        if let (Some(pipe1), Some(pipe2)) = (field.index(pos1), field.index(pos2)) {
            if (loop_coords.contains(&pos1) && !is_opening(dir, pipe1, Order::First)) || (loop_coords.contains(&pos1) && !is_opening(dir, pipe2, Order::Second)) {
                //println!("No opening between: {:?} {:?} dir {:?}", pipe1 as char, pipe2 as char, dir);
                continue
            } else {
                //println!("Squeezing between: {:?} {:?} dir {:?}", pipe1 as char, pipe2 as char, dir);
            }
            
            let (pos_n1, pos_n2) = next_nodes(pos1, dir);
            let next_dirs = next_dirs(dir);
            stack.extend(&[
                (pos1, pos_n1, next_dirs[0]),
                (pos_n1, pos_n2, next_dirs[1]),
                (pos_n2, pos2, next_dirs[2]),
            ])
        } else {
            println!("One of the nodes {:?} {:?} are outside the field", pos1, pos2);
            return true;
        }
    }
    
    //println!("Could not find a path out from start {:?} {:?} dir {:?}", pos1, pos2, dir);
    false
}

fn next_nodes(pos1: Pos, dir: Dir) -> (Pos, Pos) {
    match dir {
        Dir::Left => ((pos1.0 - 1, pos1.1), (pos1.0 - 1, pos1.1 - 1)), 
        Dir::Up => ((pos1.0, pos1.1 - 1), (pos1.0 + 1, pos1.1 - 1)),
        Dir::Right => ((pos1.0 + 1, pos1.1), (pos1.0 + 1, pos1.1 + 1)),
        Dir::Down => ((pos1.0, pos1.1 + 1), (pos1.0 - 1, pos1.1 + 1)),
    }
}
fn next_dirs(dir: Dir) -> [Dir; 3] {
    match dir {
        Dir::Left => [Dir::Down, Dir::Left, Dir::Up],
        Dir::Up =>  [Dir::Left, Dir::Up, Dir::Right],
        Dir::Right => [Dir::Up, Dir::Right, Dir::Down],
        Dir::Down => [Dir::Right, Dir::Down, Dir::Left],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Order {
    First,
    Second
}
fn is_opening(dir: Dir, pipe:u8, order: Order) -> bool {
    use Order::*;
    match (pipe, order, dir) {
        (b'|', _, Dir::Down | Dir::Up ) => true,
        (b'-', _, Dir::Left | Dir::Right ) => true,

        (b'L', First, Dir::Down) => true,
        (b'L', Second, Dir::Down) => false,
        (b'L', First, Dir::Up) => false,
        (b'L', Second, Dir::Up) => true,
        
        (b'L', First, Dir::Left) => false,
        (b'L', Second, Dir::Left) => true,
        (b'L', First, Dir::Right) => true,
        (b'L', Second, Dir::Right) => false,

        (b'J', First, Dir::Down) => false,
        (b'J', Second, Dir::Down) => true,
        (b'J', First, Dir::Up) => true,
        (b'J', Second, Dir::Up) => false,

        (b'J', First, Dir::Left) => false,
        (b'J', Second, Dir::Left) => true,
        (b'J', First, Dir::Right) => true,
        (b'J', Second, Dir::Right) => false,

        (b'7', First, Dir::Down) => false,
        (b'7', Second, Dir::Down) => true,
        (b'7', First, Dir::Up) => true,
        (b'7', Second, Dir::Up) => false,

        (b'7', First, Dir::Left) => true,
        (b'7', Second, Dir::Left) => false,
        (b'7', First, Dir::Right) => false,
        (b'7', Second, Dir::Right) => true,

        (b'F', First, Dir::Down) => true,
        (b'F', Second, Dir::Down) => false,
        (b'F', First, Dir::Up) => false,
        (b'F', Second, Dir::Up) => true,

        (b'F', First, Dir::Left) => false,
        (b'F', Second, Dir::Left) => true,
        (b'F', First, Dir::Right) => true,
        (b'F', Second, Dir::Right) => false,

        _ => false
    }
}

fn solve_part2(input: &str) -> u32 {
    let (start_pos, mut field) = parse(input);
    let found_loop = traverse(start_pos, &field);
    substitute_start_pipe(&found_loop, &mut field);

    find_pools(&found_loop, &field)
}

#[cfg(test)]
mod tests {
    use std::{error::Error, f64::consts::E};

    use crate::get_input;

    use super::*;

    #[test]
    fn day10_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 4);

        let res = solve_part1(EXAMPLE_2);
        println!("{res}");
        assert_eq!(res, 8);
    }

    #[test]
    fn day10_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 10)?;
        let res = solve_part1(&input);
        println!("day10 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day10_part2_test() {
        let res = solve_part2(EXAMPLE_PART2_SMALL);
        assert_eq!(res, 4);
        let res = solve_part2(EXAMPLE_PART2_SMALLEST);
        assert_eq!(res, 4);
        let res = solve_part2(EXAMPLE_PART2);
        assert_eq!(res, 8);
        let res = solve_part2(EXAMPLE_PART2_LARGE);
        assert_eq!(res, 10);
    }

    #[test]
    fn day10_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 10)?;
        let res = solve_part2(&input);
        println!("day10 Part2 Result: {res}");
        Ok(())
    }
}
