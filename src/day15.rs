const EXAMPLE : &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn hash(mut val: u32, chr: u32) -> u32 {
    val += chr;
    val *= 17;
    let res = val % 256;
    res
}

fn hash_str(data: &str) -> u32 {
    data.as_bytes().iter().fold(0, |acc, byte| hash(acc, *byte as u32))
}

fn solve_part1(input: &str) -> u32 {
    input.split(',').map(|elem| {
        hash_str(elem)
      }).sum()
}

struct Map<T, 'a> {
    storage: Vec<Vec<(&str, T)>>
}

impl<T, 'a> Map<T, 'a> {
    fn new() -> Self {
        Self {
            storage: Vec::from_iter((0..256).map(|_| Vec::default()))
        }
    }

    fn insert(&self, key: &'a str, val: T) {
        let index = hash_str(key);
    }
}

fn solve_part2(input: &str) -> u32 {
    let mut map = [Vec::new(); 256];
    for operation in input.split(',') {
        if let Some(label_end_index) = operation.find('-') {
            let label = &operation[0..label_end_index];
            let hash 
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day15_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 1320)
    }


    #[test]
    fn day15_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 15)?;
        let res = solve_part1(&input);
        println!("day15 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day15_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 64);
    }

    #[test]
    fn day15_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 15)?;
        let res = solve_part2(&input);
        println!("day15 Part2 Result: {res}");
        Ok(())
    }
}
