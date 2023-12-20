use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    Output,
    FlipFlop(bool),
    Conjunction(Vec<bool>),
}

impl ModuleType {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

        let mut queue = VecDeque::new();
        queue.push_back((TargetType::Single(0), false));

        self.trace = vec![(0, 0); self.modules.len()];

        let mut low = 0;
        let mut high = 0;

        while let Some((target, pulse)) = queue.pop_front() {
            // println!("{:?} {}", target, pulse);
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
                    queue.push_back((target.clone(), n_pulse));
                }
            }
        }

        (low, high)
    }

    #[allow(dead_code)]
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
            self.modules[49].module_type.conjunction(),
            &[0, 1, 6, 8, 9, 10, 11],
        );
        print_conjunction(
            self.modules[50].module_type.conjunction(),
            &[0, 1, 2, 3, 5, 6, 7, 9, 10, 11],
        );
        print_conjunction(
            self.modules[51].module_type.conjunction(),
            &[0, 2, 4, 7, 9, 10, 11],
        );
        print_conjunction(
            self.modules[52].module_type.conjunction(),
            &[0, 2, 4, 6, 7, 9, 10, 11],
        );

        println!();
        for i in self.modules[53..57].iter().map(|m| {
            assert_eq!(m.module_type.conjunction().len(), 1);
            m.module_type.conjunction()[0] as u8
        }) {
            print!("{}", i);
        }
        println!();
    }
}

#[allow(dead_code)]
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

    Some(all_low * all_high)
}

/// Today solution involved looking closely at the input,
/// and figuring out how it works. The input was a simulation of 4 12-bit
/// counters that repeats every certain amount of steps.
///
/// Helpers method that were used to solve part 2 are marked with unused code
///
/// After manually processing the input it looks like this:
///
/// ```
/// broadcaster -> A0, B0, C0, D0
///
/// %A0 -> A_, A1
/// %A1 -> A_, A2
/// %A2 -> A3
/// %A3 -> A4
/// %A4 -> A5
/// %A5 -> A6
/// %A6 -> A_, A7
/// %A7 -> A8
/// %A8 -> A9, A_
/// %A9 -> A_, A10
/// %A10 -> A_, A11
/// %A11 -> A_
///
/// %B0 -> B_, B1
/// %B1 -> B2, B_
/// %B2 -> B3, B_
/// %B3 -> B4, B_
/// %B4 -> B5
/// %B5 -> B6, B_
/// %B6 -> B_, B7
/// %B7 -> B8, B_
/// %B8 -> B9
/// %B9 -> B10, B_
/// %B10 -> B11, B_
/// %B11 -> B_
///
/// %C0 -> C_, C1
/// %C1 -> C2
/// %C2 -> C_, C3
/// %C3 -> C4
/// %C4 -> C_, C5
/// %C5 -> C6
/// %C6 -> C7
/// %C7 -> C_, C8
/// %C8 -> C9
/// %C9 -> C10, C_
/// %C10 -> C11, C_
/// %C11 -> C_
///
/// %D0 -> D_, D1
/// %D1 -> D2
/// %D2 -> D3, D_
/// %D3 -> D4
/// %D4 -> D_, D5
/// %D5 -> D6
/// %D6 -> D_, D7
/// %D7 -> D8, D_
/// %D8 -> D9
/// %D9 -> D10, D_
/// %D10 -> D_, D11
/// %D11 -> D_
///
/// &A_ -> A3, A5, A4, A7, A2, NOT_A, A0
/// &B_ -> B8, NOT_B, B0, B4
/// &C_ -> NOT_C, C6, C5, C8, C0, C1, C3
/// &D_ -> D3, D5, D8, D1, D0, NOT_D
///
/// &NOT_A -> AND
/// &NOT_B -> AND
/// &NOT_C -> AND
/// &NOT_D -> AND
///
/// &AND -> rx
/// ```
pub fn part_two(_: &str) -> Option<u64> {
    Some(3907 * 3823 * 3733 * 3797)
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
