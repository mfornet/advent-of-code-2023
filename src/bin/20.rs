use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    Output,
    FlipFlop(bool),
    Conjunction(Vec<bool>),
}

impl ModuleType {
    fn conjunction(&self) -> &[bool] {
        match self {
            Self::Conjunction(state) => state,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum TargetType {
    Single(usize),
    Conjunction(usize, usize),
}

impl TargetType {
    fn target(&self) -> usize {
        match self {
            Self::Single(index) => *index,
            Self::Conjunction(index, _) => *index,
        }
    }

    fn conjunction_index(&self) -> Option<usize> {
        match self {
            Self::Single(_) => None,
            Self::Conjunction(_, index) => Some(*index),
        }
    }
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    target: Vec<TargetType>,
}

impl Module {
    fn output() -> Self {
        Self {
            module_type: ModuleType::Output,
            target: vec![],
        }
    }
}

/*

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a

 */

fn parse_module<'a>(value: &'a str, cache: &mut HashMap<&'a str, usize>) -> (usize, Module) {
    let (left, right) = value.split_once(" -> ").unwrap();

    let target = right
        .split(", ")
        .map(|target| {
            let len = cache.len();
            TargetType::Single(*cache.entry(target).or_insert(len))
        })
        .collect_vec();

    let (name, module) = match left.chars().next().unwrap() {
        'b' => (
            left,
            Module {
                module_type: ModuleType::Broadcaster,
                target,
            },
        ),
        '%' => (
            &left[1..],
            Module {
                module_type: ModuleType::FlipFlop(false),
                target,
            },
        ),
        '&' => (
            &left[1..],
            Module {
                module_type: ModuleType::Conjunction(vec![]),
                target,
            },
        ),
        _ => unreachable!(),
    };

    let len = cache.len();
    let index = *cache.entry(name).or_insert(len);
    (index, module)
}

#[derive(Debug)]
struct Machine {
    modules: Vec<Module>,
    target: usize,
    trace: Vec<(usize, usize)>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut cache = HashMap::new();
        cache.insert("broadcaster", 0);

        for line in value
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            let (head, _) = line.split_once(" -> ").unwrap();
            if head.starts_with('%') || head.starts_with('&') {
                let name = &head[1..];
                let len = cache.len();
                cache.entry(name).or_insert(len);
            }
        }

        let mut modules = vec![];

        for line in value
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            let (index, module) = parse_module(line, &mut cache);
            if index >= modules.len() {
                modules.resize_with(index + 1, Module::output);
            }
            modules[index] = module;
        }

        modules.resize_with(cache.len(), Module::output);

        for i in 0..modules.len() {
            for j in 0..modules[i].target.len() {
                let t = modules[i].target[j].target();
                let index = if let ModuleType::Conjunction(target) = &mut modules[t].module_type {
                    target.push(false);
                    Some(target.len() - 1)
                } else {
                    None
                };

                if let Some(index) = index {
                    modules[i].target[j] = TargetType::Conjunction(t, index);
                }
            }
        }

        Self {
            modules,
            target: cache.get("rx").copied().unwrap_or_default(),
            trace: vec![(0, 0); cache.len()],
        }
    }
}

impl Machine {
    fn press_button(&mut self) -> (u64, u64) {
        // Send low pulse to the broadcaster
        let mut pending = vec![(TargetType::Single(0), false)];
        self.trace = vec![(0, 0); self.modules.len()];

        let mut low = 0;
        let mut high = 0;

        while let Some((target, pulse)) = pending.pop() {
            println!("{:?} {}", target, pulse);
            let index = target.target();

            if pulse {
                high += 1;
                self.trace[index].1 += 1;
            } else {
                low += 1;
                self.trace[index].0 += 1;
            }

            let receiver = &mut self.modules[index];

            let n_pulse = match &mut receiver.module_type {
                ModuleType::Broadcaster => Some(pulse),
                ModuleType::Output => None,
                ModuleType::FlipFlop(state) => (!pulse).then(|| {
                    *state = !*state;
                    *state
                }),
                ModuleType::Conjunction(state) => {
                    state[target.conjunction_index().unwrap()] = pulse;
                    Some(!state.iter().all(|&x| x))
                }
            };

            if let Some(n_pulse) = n_pulse {
                for target in &receiver.target {
                    pending.push((target.clone(), n_pulse));
                }
            }
        }

        (low, high)
    }

    fn print_state(&self) {
        for i in 0..4 {
            for j in 0..12 {
                let index = i * 12 + j + 1;
                match self.modules[index].module_type {
                    ModuleType::FlipFlop(state) => print!("{}", if state { 1 } else { 0 }),
                    _ => unreachable!(),
                }
            }
            println!();
        }

        println!();
        print_conjunction(
            self.modules[48 + 1 + 0].module_type.conjunction(),
            &[0, 1, 2, 3, 5, 6, 7, 9, 10, 11],
        );
        print_conjunction(
            self.modules[48 + 1 + 1].module_type.conjunction(),
            &[0, 2, 4, 7, 9, 10, 11],
        );
        print_conjunction(
            self.modules[48 + 1 + 2].module_type.conjunction(),
            &[0, 1, 6, 8, 9, 10, 11],
        );
        print_conjunction(
            self.modules[48 + 1 + 3].module_type.conjunction(),
            &[0, 2, 4, 6, 7, 9, 10, 11],
        );

        println!();
        for i in self.modules[48 + 5..48 + 5 + 4].iter().map(|m| {
            assert_eq!(m.module_type.conjunction().len(), 1);
            m.module_type.conjunction()[0] as u8
        }) {
            print!("{}", i);
        }
        println!();

        // println!();
        // println!("{:?}", self.modules[1 + 48 + 8].module_type);
        // println!("{:?}", self.modules[1 + 48 + 8 + 1].module_type);
    }
}

fn print_conjunction(state: &[bool], index: &[usize]) {
    for i in 0..12 {
        let pos = index.iter().find_position(|&&x| x == i);

        if let Some((pos, _)) = pos {
            print!("{}", if state[pos] { 1 } else { 0 });
        } else {
            print!("_")
        }
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut machine = Machine::from(input);

    let mut all_low = 0;
    let mut all_high = 0;

    for _ in 0..1000 {
        let (low, high) = machine.press_button();
        all_low += low;
        all_high += high;
    }

    let answer = all_low * all_high;
    assert_eq!(answer, 883726240);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    // return None;
    let mut machine = Machine::from(input);
    let target = machine.target;
    let mut pressed = 0;

    loop {
        machine.press_button();
        pressed += 1;

        // println!("{} {} {:?}", pressed, target, machine.trace[target]);
        // for (i, &(lo, hi)) in machine.trace.iter().enumerate() {
        //     print!("({:2}, {:2}) ", lo, hi);
        //     if i % 20 == 19 {
        //         println!();
        //     }
        // }
        // println!();

        println!("\npressed: {}\n", pressed);
        machine.print_state();

        if pressed == 2048 {
            break;
        }
        // std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();

        if machine.trace[target].0 == 1 {
            break;
        }
    }

    Some(pressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {}
}
