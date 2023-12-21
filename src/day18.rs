use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Write,
    str::FromStr,
};

const EXAMPLE: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd)]
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

    fn map(&mut self, f: impl Fn(&mut u8)) {
        self.data.iter_mut().for_each(f);
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("W: {} H: {}\n", self.width, self.height))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let data = self.get(x, y);
                f.write_char((data + b'0') as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct Color{
    r: u8,
    g: u8,
    b: u8
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("(#").ok_or(format!("Could not remove prefix '(': {:?}", s))?;
        let s = s.strip_suffix(')').ok_or(format!("Could not remove prefix ')': {:?}", s))?;
        
        Ok(Color { 
            r: u8::from_str_radix(&s[0..2], 16).map_err(|e| format!("{e} : {:?}", &s[0..2]))?, 
            g: u8::from_str_radix(&s[2..4], 16).map_err(|e| format!("{e} : {:?}", &s[2..4]))?, 
            b: u8::from_str_radix(&s[4..6], 16).map_err(|e| format!("{e} : {:?}", &s[4..6]))?
        })
    }
}

type DigPlan = Vec<(char, i32, Color)>;

fn parse(input: &str) -> DigPlan {
    let mut dig_plan = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(' ');
        let command = parts.next().unwrap().chars().next().unwrap();

        let amount = parts.next().unwrap().parse::<i32>().unwrap();
        match Color::from_str(parts.next().unwrap()) {
            Ok(color) => dig_plan.push((command, amount, color)),
            Err(e) => eprintln!("Could not parse color: {e}"),
        }
    }
    dig_plan
}

type Dir = (isize, isize);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const RIGHT: Dir = (1, 0);
const LEFT: Dir = (-1, 0);
const NONE: Dir = (0, 0);

fn format_dir(dir: Dir) -> char {
    match dir {
        UP => '^',
        DOWN => 'v',
        RIGHT => '>',
        LEFT => '<',
        _ => '#',
    }
}

fn dig_out(dig_plan: &DigPlan) -> u32 {
    // let mut hole = Vec::new();

    
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut x = 0;
    let mut y = 0;

    for (command, amount, _color) in dig_plan.iter() {
        match *command {
            'U' => {
                y -= *amount;
                min_y = min_y.min(y);
            },
            'L' => {
                x -= *amount;
                min_x = min_x.min(x);
            },
            'D' => {
                y += *amount;
                max_y = max_y.max(y);
            },
            'R' => {
                x += *amount;
                max_x = max_x.max(x);
            },
            _ => {}
        }
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    println!("X: [{min_x}:{max_x}], Y: [{min_y}:{max_y}]");
    println!("W: {width}, H: {height}");
    let mut hole = Vec::from_iter((0..(height+3)).map(|_| Vec::<(bool, Dir)>::from_iter((0..(width+3)).map(|_| (false, NONE)))));
    
    let mut x = if min_x < 0 {-min_x} else {0} as usize + 1;
    let mut y = if min_y < 0 {-min_y} else {0} as usize + 1;

    println!("Start [{x}, {y}] ");
    let mut dir = DOWN;
    let mut last_move = ' ';
    for (command, amount, _color) in dig_plan.iter() {
        //let last_dir = dir;
        match *command {
            'U' => {
                dir = match (last_move, dir) {
                    ('R', UP) => LEFT,
                    ('R', DOWN) => RIGHT,
                    ('L', UP) => RIGHT,
                    ('L', DOWN) => LEFT,
                    _ => unreachable!(),
                };
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    y -= 1
                }
            },
            'L' => {
                dir = match (last_move, dir) {
                    ('D', LEFT) => UP,
                    ('D', RIGHT) => DOWN,
                    ('U', LEFT) => DOWN,
                    ('U', RIGHT) => UP,
                    _ => unreachable!(),
                };
                
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    x -= 1
                }
            },
            'D' => {
                dir = match (last_move, dir) {
                    ('R', UP) => RIGHT,
                    ('R', DOWN) => LEFT,
                    ('L', UP) => LEFT,
                    ('L', DOWN) => RIGHT,
                    _ => unreachable!(),
                };
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    y += 1
                }
            },
            'R' => {
                dir = match (last_move, dir) {
                    ('D', LEFT) => DOWN,
                    ('D', RIGHT) => UP,
                    ('U', LEFT) => UP,
                    ('U', RIGHT) => DOWN,
                    _ => dir,
                };
                
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    x += 1
                }
            },
            _ => {}
        }
        last_move = *command;
    }

    //format(&hole);
    measure_hole(&hole)
}

use image::*;
fn measure_hole(hole: &Vec<Vec<(bool, Dir)>>) -> u32 {
    // let mut image: RgbImage  = ImageBuffer::new(hole[0].len() as u32, hole.len() as u32);

    // image.put_pixel(0, 0, Rgb([255, 255, 255]));
    // image.put_pixel(hole[0].len() as u32 - 1, hole.len() as u32 - 1, Rgb([0, 255, 0]));
    // image.put_pixel(hole[0].len() as u32 - 1, 0 as u32, Rgb([0, 0, 255]));
    // image.put_pixel(0, hole.len() as u32 - 1, Rgb([255, 255, 0]));

    let mut filled_holes = 0;
    for (y, row) in hole.iter().enumerate() {
        let mut last_was_edge = false;
        let mut last_dir = NONE;
        let mut rising_edge = 0;
        let mut falling_edge = 0;
        let mut fell_inside = false;

        for (x, (is_edge, inside_dir)) in row.iter().enumerate() {
            // if *is_edge {
            //     image.put_pixel(x as u32, y as u32, Rgb([0, 0, 255]));
            // }
            
            if *is_edge && !last_was_edge {
                rising_edge = x;

                if falling_edge > 0 && fell_inside {
                    let delta = rising_edge - falling_edge;
                    filled_holes += delta;

                    // for dx in falling_edge..rising_edge {
                    //     //print!("F");
                    //     image.put_pixel(dx as u32, y as u32, Rgb([255, 0, 0]));
                    // }

                    falling_edge = 0;
                    fell_inside = false;
                }
            }

            if !*is_edge && last_was_edge {
                falling_edge = x;

                if rising_edge > 0 {
                    let delta = falling_edge - rising_edge;
                    filled_holes += delta;

                    // for dx in rising_edge..falling_edge {
                    //     //image.put_pixel(dx as u32, y as u32, Rgb([0, 255, 0]));
                    // }

                    rising_edge = 0;
                }

                if last_dir == RIGHT || hole[y - 1][x - 1].1 == RIGHT || hole[y + 1][x - 1].1 == RIGHT {
                    fell_inside = true;
                }
            }

            last_dir = *inside_dir;
            last_was_edge = *is_edge;
        }
    }
    
    //image.save("test.png").unwrap();
    filled_holes as u32
}

fn format(hole: &Vec<Vec<bool>>) {
    for row in hole.iter() {
        for col in row.iter() {
            if *col {print!("#")} else {print!(".")}
        }
        println!("");
    }
}

fn solve_part1(input: &str) -> u32 {
    let dig_plan = parse(input);
    dig_out(&dig_plan)
}

type Pos = (i64, i64);
struct Edge {
    start: (i64, i64),
    end: (i64, i64)
}

fn dig_big(dig_plan: &DigPlan) -> u32 {


    let mut dir = DOWN;
    let mut last_move = ' ';

    let mut left_edges = Vec::new();
    let mut right_edges = Vec::new();

    for (command, amount, _color) in dig_plan.iter() {
        //let last_dir = dir;
        match *command {
            'U' => {
                dir = match (last_move, dir) {
                    ('R', UP) => LEFT,
                    ('R', DOWN) => RIGHT,
                    ('L', UP) => RIGHT,
                    ('L', DOWN) => LEFT,
                    _ => unreachable!(),
                };
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    y -= 1
                }
            },
            'L' => {
                dir = match (last_move, dir) {
                    ('D', LEFT) => UP,
                    ('D', RIGHT) => DOWN,
                    ('U', LEFT) => DOWN,
                    ('U', RIGHT) => UP,
                    _ => unreachable!(),
                };
                
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    x -= 1
                }
            },
            'D' => {
                dir = match (last_move, dir) {
                    ('R', UP) => RIGHT,
                    ('R', DOWN) => LEFT,
                    ('L', UP) => LEFT,
                    ('L', DOWN) => RIGHT,
                    _ => unreachable!(),
                };
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    y += 1
                }
            },
            'R' => {
                dir = match (last_move, dir) {
                    ('D', LEFT) => DOWN,
                    ('D', RIGHT) => UP,
                    ('U', LEFT) => UP,
                    ('U', RIGHT) => DOWN,
                    _ => dir,
                };
                
                //println!("{last_move} > {command}, {} => {}", format_dir(last_dir), format_dir(dir));
                for _ in 0..*amount {
                    hole[y][x] = (true, dir);
                    x += 1
                }
            },
            _ => {}
        }
        last_move = *command;
    }

    //format(&hole);
    measure_hole(&hole)
}

fn solve_part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day18_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 62)
    }

    #[test]
    fn day18_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 18)?;
        let res = solve_part1(&input);
        println!("day18 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day18_part1_recursive() {
    //     let res = solve_part1(&input);
    //     assert_eq!(res, 102);
    // }

    // #[test]
    // fn day18_part2_test() {
    //     let res = solve_part2(EXAMPLE);
    //     assert_eq!(res, 51);
    // }

    // #[test]
    // fn day18_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 18)?;
    //     let res = solve_part2(&input);
    //     println!("day18 Part2 Result: {res}");
    //     Ok(())
    // }
}
