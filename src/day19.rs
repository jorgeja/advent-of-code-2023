use std::{str::FromStr, collections::HashMap};

const EXAMPLE : &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize, 
    a: usize,
    s: usize,
}

impl Part {
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').ok_or(format!("Could not remove prefix '{{': {:?}", s))?;
        let s = s.strip_suffix('}').ok_or(format!("Could not remove prefix '}}': {:?}", s))?;

        let mut part =  Part { x:0, m:0, a:0, s:0 };
        for elem in s.split(',') {
            let mut elem_split = elem.split('=');
            let index = elem_split.next().unwrap();
            let num = elem_split.next().unwrap().parse::<usize>().map_err(|_| format!("Could not parse num from {elem}"))?;
            match index {
                "x" => part.x = num,
                "m" => part.m = num,
                "a" => part.a = num,
                "s" => part.s = num,
                _ => unreachable!()
            }
        }
        Ok(part)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    elem: char,
    op: char,
    threshold: usize,
    res: String
}

impl Rule {
    fn process<'a>(&'a self, part: Part) -> Option<&'a str> {
        let value = match self.elem {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!()
        };

        let is_true = match self.op {
            '>' => value > self.threshold,
            '<' => value < self.threshold,
            _ => unreachable!()
        };

        if is_true {Some(self.res.as_str())} else {None}
    }
}

impl FromStr for Rule {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let params = parts.next().unwrap();
        let res = parts.next().unwrap();

        let mut rule = Rule{elem: ' ', op: ' ', threshold: 0, res: res.to_owned()};
        rule.elem = params[0..1].chars().next().unwrap();
        rule.op = params[1..2].chars().next().unwrap();
        rule.threshold = params[2..].parse::<usize>().unwrap();

        Ok(rule)
    }
}

#[derive(Debug)]
struct Machine {
    name: String,
    rules: Vec<Rule>,
    default: String,
}

impl Machine {
    fn process<'a>(&'a self, part: Part) -> &'a str {
        for rule in self.rules.iter() {
            if let Some(res) = rule.process(part) {
                return res;
            }
        }
        self.default.as_str()
    }
}

impl FromStr for Machine {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('{');
        let name = parts.next().ok_or(format!("could not find start '{{' in {s}"))?;
        let rest = parts.next().ok_or(format!("could not find rest of the rules in {s}"))?.split(',');

        let mut machine = Machine{name: name.to_owned(), rules: Vec::default(), default: String::default()};

        for r in rest {
            if r.contains('}') {
                let r = r.strip_suffix('}').unwrap();
                machine.default = r.to_owned();
            } else {
                let rule = Rule::from_str(r)?;
                machine.rules.push(rule)
            }
        }

        Ok(machine)
    }
}

fn parse(input: &str) -> (HashMap<String, Machine>, Vec<Part>) {
    let mut machines = HashMap::new();

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() { break }

        match Machine::from_str(line) {
            Ok(machine) => {machines.insert(machine.name.clone(), machine);},
            Err(e) => {println!("Could not parse line: {e}");}
        }
    }

    let mut parts = Vec::new();
    for line in lines {
        match Part::from_str(line) {
            Ok(part) => parts.push(part),
            Err(e) => println!("Could not parse part from line: {e}"),
        }
    }

    (machines, parts)
}

fn solve_part1(input: &str) -> usize {
    let (machines, parts) = parse(input);
    
    let mut rating = 0;
    for part in parts.iter() {
        let mut res = "in";
        while res != "A" && res != "R" {
            if let Some(machine) = machines.get(res) {
                res = machine.process(*part);
            } else {
                println!("Could not find machine: {res}");
                break;
            }
        }

        if res == "A" {
            rating += part.rating();
        }
    }
    rating
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day19_part1_test() {
        let res = solve_part1(EXAMPLE);
        println!("{res}");
        assert_eq!(res, 19114)
    }

    #[test]
    fn day19_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 19)?;
        let res = solve_part1(&input);
        println!("day19 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day19_part2_test() {
    //     let res = solve_part2(EXAMPLE);
    //     assert_eq!(res, 62);
    // }

    // #[test]
    // fn day19_part2_test() {
    //     let res = solve_part2(EXAMPLE);
    //     assert_eq!(res, 51);
    // }

    // #[test]
    // fn day19_part2() -> Result<(), Box<dyn Error>> {
    //     let input = get_input(2023, 19)?;
    //     let res = solve_part2(&input);
    //     println!("day19 Part2 Result: {res}");
    //     Ok(())
    // }
}
