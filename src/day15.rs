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

struct Map {
    storage: Vec<Vec<(String, u32)>>
}

impl Map {
    fn new() -> Self {
        Self {
            storage: Vec::from_iter((0..256).map(|_| Vec::default()))
        }
    }

    fn insert(&mut self, key: &str, val: u32) {
        let index = hash_str(key);
        if let Some((_, stored_val)) = self.storage[index as usize].iter_mut().find(|elem| &elem.0 == key) {
            *stored_val = val;
        } else {
            self.storage[index as usize].push((key.to_owned(), val));
        }
    }

    fn remove(&mut self, key: &str) {
        let index = hash_str(key);
        self.storage[index as usize].retain(|elem| &elem.0 != key)
    }

    fn focusing_power(&self) -> u32 {
        self.storage.iter().enumerate().map(|(num, stored_box)| -> u32 {
            stored_box.iter().enumerate().map(|(i, (_, lens))| (num as u32 + 1) * (i as u32 + 1) * *lens).sum()
        }).sum()
    }
}

fn solve_part2(input: &str) -> u32 {
    let mut map = Map::new();
    
    for operation in input.split(',') {
        if let Some(label_end_index) = operation.find('-') {
            let label = &operation[0..label_end_index];
            map.remove(label);
        } else if let Some(label_end_index) = operation.find('=') {
            let label = &operation[0..label_end_index];

            match operation[label_end_index+1..label_end_index+2].parse::<u32>() {
                Ok(val) => map.insert(label, val),
                Err(err) => println!("Cant parse {} to int, because {err}", &operation[label_end_index+1..])
            }   
        }
    }

    map.focusing_power()
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
        assert_eq!(res, 145);
    }

    #[test]
    fn day15_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 15)?;
        let res = solve_part2(&input);
        println!("day15 Part2 Result: {res}");
        Ok(())
    }
}
