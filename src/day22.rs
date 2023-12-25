use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    fmt::Write,
    str::FromStr,
};

const EXAMPLE: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

type Pos = (isize, isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: Pos,
    end: Pos,
}

impl Brick {
    fn overlap_xy(&self, other: &Brick) -> bool {
        if other.start.0 > self.end.0
            || other.start.1 > self.end.1
            || other.end.0 < self.start.0
            || other.end.1 < self.start.1
        {
            false
        } else {
            true
        }
    }
    fn check_format(&self) -> bool {
        self.start.0 <= self.end.0 && self.start.1 <= self.end.1 && self.start.2 <= self.end.2
    }
}

impl FromStr for Brick {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('~');
        let mut start_nums = parts.next().ok_or(format!("Bad format"))?.split(',');
        let mut end_nums = parts.next().ok_or(format!("Bad format"))?.split(',');
        Ok(Brick {
            start: (
                start_nums.next().ok_or(format!("Bad format"))?.parse()?,
                start_nums.next().ok_or(format!("Bad format"))?.parse()?,
                start_nums.next().ok_or(format!("Bad format"))?.parse()?,
            ),
            end: (
                end_nums.next().ok_or(format!("Bad format"))?.parse()?,
                end_nums.next().ok_or(format!("Bad format"))?.parse()?,
                end_nums.next().ok_or(format!("Bad format"))?.parse()?,
            ),
        })
    }
}

fn solve_part1(input: &str) -> u32 {
    let mut bricks = input
        .lines()
        .filter_map(|l| match Brick::from_str(l) {
            Ok(b) => Some(b),
            Err(e) => {
                println!("Could not parse brick from {l}: {e}");
                None
            }
        })
        .collect::<Vec<_>>();

    if !bricks.iter().all(Brick::check_format) {
        panic!("Not all bricks have the correct format!!");
    }

    for i in 0..bricks.len() {
        let mut brick = bricks[i];
        let length = brick.end.2 - brick.start.2;
        let start_z = brick.start.2;
        for lower_brick in bricks[..i].iter().rev() {
            if brick.overlap_xy(lower_brick) {
                //print!("[{i}] {brick:?} overlaps {lower_brick:?} ");
                let lower_z = lower_brick.end.2;
                if brick.start.2 != start_z {
                    if lower_z > brick.start.2 + 1 {
                        brick.start.2 = lower_z + 1;
                        brick.end.2 = brick.start.2 + length;
                        //println!("moved to z {}", lower_z + 1);
                    } else {
                        //println!("");
                    }
                } else if lower_z + 1 < brick.start.2 {
                    brick.start.2 = lower_z + 1;
                    brick.end.2 = brick.start.2 + length;
                    //println!("moved to z {}", lower_z + 1);
                } else {
                    //println!("");
                }
            }
        }

        bricks[i] = brick;
    }

    let mut supports = HashMap::new();
    let mut supported_by = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        for (j, other_brick) in bricks[i + 1..].iter().enumerate() {
            let j = j + i + 1;
            if brick.overlap_xy(other_brick) {
                let this_top_z = brick.end.2;
                let other_bottom_z = other_brick.start.2;

                let this_bottom_z = brick.start.2;
                let other_top_z = other_brick.end.2;

                if this_top_z + 1 == other_bottom_z {
                    let sup = supported_by.entry(j).or_insert(Vec::default());
                    if !sup.contains(&i) {
                        sup.push(i);
                    }
                    let sup = supports.entry(i).or_insert(Vec::default());
                    if !sup.contains(&j) {
                        sup.push(j);
                    }
                } else if other_top_z + 1 == this_bottom_z {
                    let sup = supported_by.entry(i).or_insert(Vec::default());
                    if !sup.contains(&j) {
                        sup.push(j);
                    }
                    let sup = supports.entry(j).or_insert(Vec::default());
                    if !sup.contains(&i) {
                        sup.push(i);
                    }
                }
            }
        }
    }

    let mut can_be_removed = HashSet::new();

    for (supported, supporters) in supported_by.iter() {
        //println!("{supported:?} supported by {supporters:?}");
        if supporters.len() > 1 {
            for s in supporters {
                can_be_removed.insert(*s);
            }
        }

        if !supports.contains_key(supported) {
            can_be_removed.insert(*supported);
        }
    }

    for (_, supporters) in supported_by.iter() {
        //println!("{supported:?} supported by {supporters:?}");
        if supporters.len() == 1 {
            let _ = can_be_removed.remove(&supporters[0]);
        }
    }

    can_be_removed.len() as u32
}

fn solve_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day22_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        //assert_eq!(res, 16)
    }

    #[test]
    fn day22_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 22)?;
        let res = solve_part1(&input);
        println!("day22 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day22_part2_test() {
    //     let res = solve_part2(EXAMPLE);
    //     assert_eq!(res, 51);
    // }

    // #[test]
    // fn day22_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 22)?;
    //     let res = solve_part2(&input);
    //     println!("day22 Part2 Result: {res}");
    //     Ok(())
    // }
}
