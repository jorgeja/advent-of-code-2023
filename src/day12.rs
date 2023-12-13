use std::collections::HashMap;
use std::sync::atomic::AtomicI16;
use rayon::prelude::*;

const EXAMPLE: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u32>)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let record = parts
                .next()
                .unwrap()
                .as_bytes()
                .iter()
                .copied()
                .collect();
            let numbers = parts
                .next()
                .unwrap()
                .split(',')
                .map(str::parse::<u32>)
                .filter_map(|r| r.ok())
                .collect();
            (record, numbers)
        })
        .collect()
}

fn parse_part2(input: &str) -> Vec<(usize, Vec<u8>, usize, Vec<u32>)> {
    input
        .lines()
        .filter_map(|l| {
            let mut parts = l.split(' ');
            let mut record: Vec<u8> = parts
                .next()?
                .as_bytes()
                .iter()
                .copied()
                .collect();
            let mut numbers: Vec<u32> = parts
                .next()?
                .split(',')
                .map(str::parse::<u32>)
                .filter_map(|r| r.ok())
                .collect();

            let rec_filler = record.clone();
            record.extend((0..4).map(|_| [b'?'].iter().chain(rec_filler.iter())).flatten());
            let num_filler = numbers.clone();
            numbers.extend((0..4).map(|_| num_filler.iter()).flatten());
            Some((rec_filler.len(), record, num_filler.len(), numbers))
        })
        .collect()
}


fn find_permutations(start: usize, record: &[u8], num_index:usize, numbers: &[u32]/*s: String */) -> u64 {
    let mut valid_permutations = 0;
    
    let length = numbers[num_index] as usize;
    
    'start_pos: for i in start..record.len() {
        
        let window = i + length;
        if window > record.len() { break }

        for j in i..window {
            if record[j] == b'.' {
                continue 'start_pos;
            }
        }

        if (i > 0 && record[i - 1] == b'#') || (window < record.len() && record[window] == b'#') {
            continue 'start_pos;
        }

        for skipped in start..i {
            if record[skipped] == b'#' {
                continue 'start_pos;
            }
        }

        if num_index + 1 < numbers.len()  {
            if window < record.len() {
                //let mut next_s = s.clone();
                //next_s.extend((start.max(1)-1..i).map(|_| '.'));
                //next_s.extend((i..window).map(|_| '#'));
                valid_permutations += find_permutations(window + 1, &record, num_index + 1, &numbers/* , next_s*/)
            }
        } else {
            for skipped in window..record.len() {
                if record[skipped] == b'#' {
                    continue 'start_pos;
                }
            }

            //let mut next_s = s.clone();
            //next_s.extend((start.max(1)-1..i).map(|_| '.'));
            //next_s.extend((i..window).map(|_| '#'));
            //next_s.extend((window..record.len()).map(|_| '.'));
            valid_permutations += 1;
            //println!(" {}", next_s);
        }
    }

    valid_permutations
}

fn find_permutations_cached(start: usize, record: &[u8], num_index:usize, numbers: &[u32], r_size: usize, n_size: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(cached) = cache.get(&(start, num_index)) {
        return *cached;
    }

    let mut valid_permutations = 0;
    
    let length = numbers[num_index] as usize;
    
    'start_pos: for i in start..record.len() {
        
        let window = i + length;
        if window > record.len() { break }

        for j in i..window {
            if record[j] == b'.' {
                continue 'start_pos;
            }
        }

        if (i > 0 && record[i - 1] == b'#') || (window < record.len() && record[window] == b'#') {
            continue 'start_pos;
        }

        for skipped in start..i {
            if record[skipped] == b'#' {
                continue 'start_pos;
            }
        }

        if num_index + 1 < numbers.len()  {
            if window < record.len() {
                //let mut next_s = s.clone();
                //next_s.extend((start.max(1)-1..i).map(|_| '.'));
                //next_s.extend((i..window).map(|_| '#'));
                valid_permutations += find_permutations_cached(window + 1, &record, num_index + 1, &numbers, r_size, n_size, cache)
            }
        } else {
            for skipped in window..record.len() {
                if record[skipped] == b'#' {
                    continue 'start_pos;
                }
            }

            //let mut next_s = s.clone();
            //next_s.extend((start.max(1)-1..i).map(|_| '.'));
            //next_s.extend((i..window).map(|_| '#'));
            //next_s.extend((window..record.len()).map(|_| '.'));
            valid_permutations += 1;
            //println!(" {}", next_s);
        }
    }

    cache.insert((start, num_index), valid_permutations);
    valid_permutations
}

fn solve_part1(input: &str) -> u64 {
    let springs = parse(input);

    springs
        .iter()
        .map(|(record, numbers)| {
            //println!(":{} {:?} :", String::from_utf8(record.clone()).unwrap(), numbers);
            find_permutations(0, &record, 0, &numbers/*, String::default()*/)
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let springs = parse_part2(input);
    springs
        .par_iter()
        .map(|(rec_size, record, num_size, numbers)| {
            //println!(":{} {:?} :", String::from_utf8(record.clone()).unwrap(), numbers);
            let mut cache = HashMap::new();
            find_permutations_cached(0, &record, 0, &numbers, *rec_size, *num_size, &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;
    

    #[test]
    fn day12_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 21);
    }

    #[test]
    fn day12_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 12)?;
        let res = solve_part1(&input);
        println!("day12 Part1 Result: {res}");
        Ok(())
    }

    #[test]
    fn day12_part2_test() {
        let res = solve_part2(EXAMPLE);
        assert_eq!(res, 525152);
    }

    #[test]
    fn day12_part2_sol() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 12)?;
        let res = solve_part2(&input);
        println!("day12 Part2 Result: {res}");
        Ok(())
    }
}
