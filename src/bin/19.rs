use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, Clone)]
enum Target<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

impl<'a> From<&'a str> for Target<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Target::Accept,
            "R" => Target::Reject,
            _ => Target::Workflow(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Section {
    X,
    M,
    A,
    S,
}

impl From<&str> for Section {
    fn from(value: &str) -> Self {
        match value {
            "x" => Section::X,
            "m" => Section::M,
            "a" => Section::A,
            "s" => Section::S,
            _ => panic!("Invalid section"),
        }
    }
}

#[derive(Debug)]
struct Condition {
    section: Section,
    op: std::cmp::Ordering,
    value: u64,
}

impl Condition {
    fn accept(&self, part: &Part) -> bool {
        match self.section {
            Section::X => self.op == part.x.cmp(&self.value),
            Section::M => self.op == part.m.cmp(&self.value),
            Section::A => self.op == part.a.cmp(&self.value),
            Section::S => self.op == part.s.cmp(&self.value),
        }
    }

    fn filter(&self, range_part: &RangePart) -> (Option<RangePart>, Option<RangePart>) {
        let (lo, hi) = range_part.get_range(self.section);
        match self.op {
            std::cmp::Ordering::Less => {
                if hi < self.value {
                    (Some(range_part.clone()), None)
                } else if self.value <= lo {
                    (None, Some(range_part.clone()))
                } else {
                    let mut accept = range_part.clone();
                    let mut reject = range_part.clone();
                    accept.update_range(self.section, (lo, self.value - 1));
                    reject.update_range(self.section, (self.value, hi));
                    (Some(accept), Some(reject))
                }
            }
            std::cmp::Ordering::Greater => {
                if lo > self.value {
                    (Some(range_part.clone()), None)
                } else if self.value >= hi {
                    (None, Some(range_part.clone()))
                } else {
                    let mut accept = range_part.clone();
                    let mut reject = range_part.clone();
                    accept.update_range(self.section, (self.value + 1, hi));
                    reject.update_range(self.section, (lo, self.value));
                    (Some(accept), Some(reject))
                }
            }
            std::cmp::Ordering::Equal => unreachable!(),
        }
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let (section, op, value) = if value.contains('<') {
            let (section, value) = value.split_once('<').unwrap();
            (section, std::cmp::Ordering::Less, value)
        } else {
            let (section, value) = value.split_once('>').unwrap();
            (section, std::cmp::Ordering::Greater, value)
        };

        let section = section.into();
        let value = value.parse().unwrap();

        Condition { section, op, value }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    target: Target<'a>,
}

impl<'a> Rule<'a> {
    fn accept(&self, part: &Part) -> bool {
        self.condition
            .as_ref()
            .map(|condition| condition.accept(part))
            .unwrap_or(true)
    }

    fn filter(&self, range_part: &RangePart) -> (Option<RangePart>, Option<RangePart>) {
        self.condition
            .as_ref()
            .map(|condition| condition.filter(range_part))
            .unwrap_or_else(|| (Some(range_part.clone()), None))
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let (condition, target) = s
            .split_once(':')
            .map(|(condition, target)| (Some(condition), target))
            .unwrap_or((None, s));

        let condition = condition.map(Into::into);
        let target = target.into();
        Rule { condition, target }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(s: &'a str) -> Self {
        Workflow {
            rules: s.split(',').map(Into::into).collect(),
        }
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let mut parts = s
            .split(',')
            .map(|s| s.split_once('=').unwrap().1.parse().unwrap());

        Part {
            x: parts.next().unwrap(),
            m: parts.next().unwrap(),
            a: parts.next().unwrap(),
            s: parts.next().unwrap(),
        }
    }
}

#[derive(Clone)]
struct RangePart {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl RangePart {
    fn new() -> Self {
        RangePart {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn count(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn get_range(&self, section: Section) -> (u64, u64) {
        match section {
            Section::X => self.x,
            Section::M => self.m,
            Section::A => self.a,
            Section::S => self.s,
        }
    }

    fn update_range(&mut self, section: Section, range: (u64, u64)) {
        match section {
            Section::X => self.x = range,
            Section::M => self.m = range,
            Section::A => self.a = range,
            Section::S => self.s = range,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (system, parts) = input.split_once("\n\n").unwrap();

    let system = system
        .lines()
        .map(|line| {
            let (name, description) = line.split_once('{').unwrap();
            let description = description.trim_end_matches('}');
            (name, Into::<Workflow>::into(description))
        })
        .collect::<HashMap<_, _>>();

    Some(
        parts
            .lines()
            .map(|line| line.trim_start_matches('{').trim_end_matches('}').into())
            .filter_map(|part: Part| -> Option<u64> {
                let mut next_workflow = Target::Workflow("in");

                while let Target::Workflow(workflow_id) = next_workflow {
                    let workflow = system.get(workflow_id).unwrap();

                    for rule in &workflow.rules {
                        if rule.accept(&part) {
                            next_workflow = rule.target.clone();
                            break;
                        }
                    }
                }

                match next_workflow {
                    Target::Accept => Some(part.value()),
                    Target::Reject => None,
                    _ => unreachable!(),
                }
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (system, _) = input.split_once("\n\n").unwrap();

    let system = system
        .lines()
        .map(|line| {
            let (name, description) = line.split_once('{').unwrap();
            let description = description.trim_end_matches('}');
            (name, Into::<Workflow>::into(description))
        })
        .collect::<HashMap<_, _>>();

    let mut parts = vec![("in", RangePart::new())];
    let mut answer = 0;

    while let Some((workflow_id, mut range_part)) = parts.pop() {
        let workflow = system.get(workflow_id).unwrap();

        for rule in &workflow.rules {
            let (accept, reject) = rule.filter(&range_part);

            if let Some(accept) = accept {
                match rule.target {
                    Target::Accept => answer += accept.count(),
                    Target::Reject => {}
                    Target::Workflow(next) => {
                        parts.push((next, accept));
                    }
                }
            }

            if let Some(reject) = reject {
                range_part = reject;
            } else {
                break;
            }
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
