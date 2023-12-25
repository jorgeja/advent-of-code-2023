use core::{num, panic};
use std::{
    collections::{BinaryHeap, HashMap},
    default,
    fmt::Write,
    str::FromStr,
};

use lcmx::lcmx;

const EXAMPLE1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

const EXAMPLE2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Low,
    High,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct FlipFlop {
    output: State,
}

impl FlipFlop {
    fn set_state(&mut self, new_state: State) -> Option<State> {
        if new_state == State::Low {
            self.output = if self.output == State::Low {
                State::High
            } else {
                State::Low
            };
            return Some(self.output);
        }
        None
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Conjunc<'a> {
    state: Vec<(&'a str, State)>,
}

impl<'a> Conjunc<'a> {
    fn set_state(&mut self, input: &str, new_state: State) -> State {
        let mut all_on = true;
        for (inp_name, state) in self.state.iter_mut() {
            if *inp_name == input {
                *state = new_state;
            }

            if *state == State::Low {
                all_on = false;
            }
        }

        if all_on {
            State::Low
        } else {
            State::High
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum NodeType<'a> {
    #[default]
    Unknown,
    BroadCaster,
    FlipFlop(FlipFlop),
    Conjunc(Conjunc<'a>),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node<'a> {
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    node_type: NodeType<'a>,
}

type System<'a> = HashMap<&'a str, Node<'a>>;

fn parse<'a>(input: &'a str) -> System<'a> {
    let mut system = System::new();

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let node_identifier = parts.next().unwrap();
        let outputs = parts.next().unwrap().split(", ").collect::<Vec<&str>>();

        if node_identifier.starts_with("broadcaster") {
            system.entry(node_identifier).or_insert(Node {
                inputs: Vec::default(),
                outputs: outputs.clone(),
                node_type: NodeType::BroadCaster,
            });
            for output in outputs {
                let node = system.entry(output).or_insert(Node::default());
                node.inputs.push(node_identifier)
            }
        } else {
            let (kind, name) = (&node_identifier[0..1], &node_identifier[1..]);

            let node = system.entry(name).or_insert(Node {
                inputs: Vec::default(),
                outputs: Vec::default(),
                node_type: NodeType::Unknown,
            });
            node.outputs = outputs.clone();

            match kind {
                "%" => {
                    node.node_type = NodeType::FlipFlop(FlipFlop::default());
                }
                "&" => {
                    if node.inputs.is_empty() {
                        node.node_type = NodeType::Conjunc(Conjunc::default());
                    } else {
                        let state = node
                            .inputs
                            .iter()
                            .copied()
                            .map(|node_id| (node_id, State::Low))
                            .collect::<Vec<_>>();
                        node.node_type = NodeType::Conjunc(Conjunc { state })
                    }
                }
                _ => {}
            };

            for output in outputs {
                let out_node = system.entry(output).or_insert(Node::default());
                out_node.inputs.push(name);

                if let NodeType::Conjunc(conjunc) = &mut out_node.node_type {
                    conjunc.state.push((name, State::Low));
                }
            }
        }
    }

    system
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Signal<'a> {
    iter: usize,
    sender: &'a str,
    receiver: &'a str,
    pulse: State,
}

impl<'a> PartialOrd for Signal<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.iter.cmp(&other.iter))
    }
}

impl<'a> Ord for Signal<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn solve_part1(loops: usize, input: &str) -> u32 {
    let mut system = parse(input);

    // for (node_name, node) in system.iter() {
    //     println!("{node_name} {node:?}");
    // }

    let mut num_low = 0;
    let mut num_high = 0;
    let mut stack = BinaryHeap::new();
    for _ in 0..loops {
        stack.push(Signal {
            iter: usize::MAX,
            sender: "button",
            receiver: "broadcaster",
            pulse: State::Low,
        });
        //println!("\nNew Loop");
        while let Some(Signal {
            iter,
            sender,
            receiver,
            pulse,
        }) = stack.pop()
        {
            match pulse {
                State::Low => num_low += 1,
                State::High => num_high += 1,
            }

            let node = system.get_mut(receiver).unwrap();
            //println!("{sender} -{pulse:?}-> {receiver}");
            if let Some(out_pulse) = match &mut node.node_type {
                NodeType::Unknown => None,
                NodeType::BroadCaster => Some(pulse),
                NodeType::FlipFlop(flip_flop) => flip_flop.set_state(pulse),
                NodeType::Conjunc(conjunc) => Some(conjunc.set_state(sender, pulse)),
            } {
                if iter == 0 {
                    panic!("Iter is ZERO..")
                }

                let next_iter = iter - 1;

                for output in node.outputs.iter().rev() {
                    stack.push(Signal {
                        iter: next_iter,
                        sender: receiver,
                        receiver: output,
                        pulse: out_pulse,
                    });
                }
            }
        }
    }
    num_high * num_low
}

fn solve_part2(input: &str) -> u32 {
    let mut system = parse(input);

    let mut conj_loops = HashMap::new();
    let mut conjs = 0;
    for (_, node) in system.iter() {
        if let NodeType::Conjunc(_) = node.node_type {
            conjs += 1;
        }
    }

    let mut stack = BinaryHeap::new();
    let mut button_pushes = 0;
    loop {
        button_pushes += 1;
        stack.push(Signal {
            iter: usize::MAX,
            sender: "button",
            receiver: "broadcaster",
            pulse: State::Low,
        });
        //println!("\nNew Loop");

        while let Some(Signal {
            iter,
            sender,
            receiver,
            pulse,
        }) = stack.pop()
        {
            let node = system.get_mut(receiver).unwrap();
            //println!("{sender} -{pulse:?}-> {receiver}");
            if let Some(out_pulse) = match &mut node.node_type {
                NodeType::Unknown => None,
                NodeType::BroadCaster => Some(pulse),
                NodeType::FlipFlop(flip_flop) => flip_flop.set_state(pulse),
                NodeType::Conjunc(conjunc) => {
                    let p = conjunc.set_state(sender, pulse);
                    if p == State::Low {
                        let (last_button_push, delta) =
                            conj_loops.entry(receiver).or_insert((button_pushes, 0));
                        if *last_button_push != button_pushes {
                            let new_delta = button_pushes - *last_button_push;
                            if *delta > 0 && *delta != new_delta {
                                panic!(
                                    "Nah.. doesnt work.. delta: {delta}, new_delta: {new_delta}"
                                );
                            }
                            //println!("Loop for {receiver} at {button_pushes} delta: {delta}");
                            *last_button_push = button_pushes;
                            *delta = new_delta;
                        }
                    }
                    Some(p)
                }
            } {
                if iter == 0 {
                    panic!("Iter is ZERO..")
                }

                let next_iter = iter - 1;

                for output in node.outputs.iter().rev() {
                    stack.push(Signal {
                        iter: next_iter,
                        sender: receiver,
                        receiver: output,
                        pulse: out_pulse,
                    });
                }
            }
        }

        if button_pushes % 1000000 == 0 {
            println!("Still pushing after {button_pushes}..");
            let mut num_loops = 0;
            for (_, (_, delta)) in conj_loops.iter() {
                if *delta > 0 {
                    num_loops += 1;
                }
            }
            println!(
                "Conj's looping: {num_loops}/{} tot: {conjs}",
                conj_loops.len()
            );

            if conjs - num_loops == 1 {
                break;
            }
        }
    }

    let mut loop_vals = Vec::new();
    for elem in conj_loops.iter() {
        println!("{elem:?}");
        loop_vals.push(elem.1 .1);
    }
    println!("Found all except one loop after {button_pushes}");

    lcmx(&loop_vals).unwrap()
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::get_input;

    use super::*;

    #[test]
    fn day20_part1_test() {
        let res = solve_part1(1, EXAMPLE1);
        println!("{res}");
        assert_eq!(res, 32);

        let res = solve_part1(1000, EXAMPLE1);
        println!("{res}");
        assert_eq!(res, 32000000);

        let res = solve_part1(1000, EXAMPLE2);
        println!("{res}");
        assert_eq!(res, 11687500);
    }

    #[test]
    fn day20_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 20)?;
        let res = solve_part1(1000, &input);
        println!("day20 Part1 Result: {res}");
        Ok(())
    }

    // #[test]
    // fn day20_part1_recursive() {
    //     let res = solve_part1(&input);
    //     assert_eq!(res, 102);
    // }

    // #[test]
    // fn day20_part2_test() {
    //     let res = solve_part2(EXAMPLE);
    //     assert_eq!(res, 51);
    // }

    #[test]
    fn day20_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(2023, 20)?;
        let res = solve_part2(&input);
        println!("day20 Part2 Result: {res}");
        Ok(())
    }
}
