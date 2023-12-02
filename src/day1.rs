fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
            if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                let first = (*first as usize - 48) as i32 * 10;
                let last = (*last as usize - 48) as i32;
                first + last
            } else {
                0
            }
        })
        .sum()
}

fn find_digits_in_line(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let mut last_found_index = 0;
    let mut current_index = 0;
    let mut digits = Vec::default();
    while current_index < bytes.len() {
        if b'0' <= bytes[current_index] && bytes[current_index] <= b'9' {
            let digit = (bytes[current_index] - 48) as i32;
            digits.push(digit);
            last_found_index = current_index + 1;
        } else {
            let mut digit = 0;
            for i in last_found_index..current_index {
                digit = match &bytes[i..current_index + 1] {
                    b"one" => 1,
                    b"two" => 2,
                    b"three" => 3,
                    b"four" => 4,
                    b"five" => 5,
                    b"six" => 6,
                    b"seven" => 7,
                    b"eight" => 8,
                    b"nine" => 9,
                    _ => 0,
                };

                if digit > 0 {
                    break;
                }
            }
            if digit > 0 {
                digits.push(digit);
                last_found_index = current_index;
            }
        }
        current_index += 1;
    }

    if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
        let res = first * 10 + last;
        res
    } else {
        0
    }
}

fn solve_part2(input: &str) -> i32 {
    input.lines().map(find_digits_in_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;

    #[test]
    fn day1_part1_test() {
        let test_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let res = solve_part1(test_input);
        assert_eq!(res, 142);
    }
    #[test]
    fn day1_part1() {
        let input = get_input(2023, 1).unwrap();
        let res = solve_part1(&input);
        println!("Day1 Part1 Result: {res}");
    }
    #[test]
    fn day1_part2_test() {
        let test_input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let res = solve_part2(test_input);
        assert_eq!(res, 281);
    }

    #[test]
    fn day1_part2() {
        let test_input = get_input(2023, 1).unwrap();
        let res = solve_part2(&test_input);
        println!("Day1 Part2 Result: {res}");
    }
}
