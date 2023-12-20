use std::{collections::HashMap, io::BufRead, str::FromStr};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

trait Rule {
    fn matches(&self, input: &HashMap<String, usize>) -> bool;
}

enum Op {
    LessThan {
        variable: String,
        value: usize,
        output: Id,
    },
    GreaterThan {
        variable: String,
        value: usize,
        output: Id,
    },
    Identity {
        output: Id,
    },
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Id {
    R,
    A,
    In,
    Intermediate(String),
}

impl FromStr for Id {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Id::R),
            "A" => Ok(Id::A),
            "in" => Ok(Id::In),
            s => Ok(Id::Intermediate(s.to_string())),
        }
    }
}

impl Op {
    fn output(&self) -> &Id {
        match self {
            Op::LessThan { output, .. } => output,
            Op::GreaterThan { output, .. } => output,
            Op::Identity { output } => output,
        }
    }
}

impl Rule for Op {
    fn matches(&self, input: &HashMap<String, usize>) -> bool {
        match self {
            Op::LessThan {
                variable, value, ..
            } => {
                let var_value = input.get(variable).unwrap();
                var_value < value
            }
            Op::GreaterThan {
                variable, value, ..
            } => {
                let var_value = input.get(variable).unwrap();
                var_value > value
            }
            Op::Identity { .. } => true,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(r"^(\w)([><])(\w+):(\w+)$").unwrap();
        if let Some(capture) = reg.captures(s) {
            let variable = capture[1].to_string();
            let op = capture[2].to_string();
            let value = capture[3]
                .parse::<usize>()
                .unwrap_or_else(|_e| panic!("Invalid value: {}", &capture[3]));
            let output = capture[4].parse().unwrap();

            match op.as_str() {
                "<" => Ok(Op::LessThan {
                    variable,
                    value,
                    output,
                }),
                ">" => Ok(Op::GreaterThan {
                    variable,
                    value,
                    output,
                }),
                _ => panic!("Unknown operator {}", op),
            }
        } else {
            let output = s.parse().unwrap();
            Ok(Op::Identity { output })
        }
    }
}

struct Workflow {
    rules: Vec<Op>,
}

struct Workflows {
    workflows: HashMap<Id, Workflow>,
}

impl Workflows {
    fn process(&self, input: &HashMap<String, usize>) -> &Id {
        let mut workflow = self.workflows.get(&Id::In).unwrap();
        while let Some(rule) = workflow.rules.iter().find(|rule| rule.matches(input)) {
            let id = rule.output();
            if id == &Id::R || id == &Id::A {
                return id;
            }
            workflow = self.workflows.get(id).unwrap();
        }

        panic!("Could not end");
    }

    /// We set the range of values to the maximum we expect as the input and for each rule we
    /// either split the range in two to satisfy both branches of the rules and push the new
    /// range that satisfies the rule to the stack and continue down the workflow with now
    /// split range. The exception to this is the identity rule which just pushes to the stack
    /// and we assume that is the last in the workflow.
    fn process_range(
        &self,
        min_value: usize,
        max_value: usize,
    ) -> Vec<HashMap<&str, (usize, usize)>> {
        let mut stack = vec![];
        let mut input = HashMap::new();

        input.insert("x", (min_value, max_value));
        input.insert("m", (min_value, max_value));
        input.insert("a", (min_value, max_value));
        input.insert("s", (min_value, max_value));

        stack.push((&Id::In, input));

        let mut accepted_ranges = vec![];

        while let Some((workflow_id, mut ranges)) = stack.pop() {
            if workflow_id == &Id::A {
                accepted_ranges.push(ranges);
            } else if workflow_id != &Id::R {
                let workflow = self.workflows.get(workflow_id).unwrap();

                for rule in &workflow.rules {
                    match rule {
                        Op::LessThan {
                            variable,
                            value,
                            output,
                        } => {
                            let (min, max) = ranges[variable.as_str()];

                            if (min..=max).contains(value) {
                                let mut new_ranges = ranges.clone();

                                new_ranges.insert(variable, (min, value - 1));
                                stack.push((output, new_ranges));

                                ranges.insert(variable, (*value, max));
                            }
                        }
                        Op::GreaterThan {
                            variable,
                            value,
                            output,
                        } => {
                            let (min, max) = ranges[variable.as_str()];

                            if (min..=max).contains(value) {
                                let mut new_ranges = ranges.clone();

                                new_ranges.insert(variable, (value + 1, max));
                                stack.push((output, new_ranges));

                                ranges.insert(variable, (min, *value));
                            }
                        }
                        Op::Identity { output } => {
                            stack.push((output, ranges.clone()));
                        }
                    }
                }
            }
        }
        accepted_ranges
    }
}

impl FromStr for Workflows {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(|line| {
                let (id, rest) = line.split_once('{').unwrap();
                let rest = rest.trim_end_matches('}');
                let rules = rest
                    .split(',')
                    .map(|s| Op::from_str(s).unwrap())
                    .collect::<Vec<_>>();

                (id.parse().unwrap(), Workflow { rules })
            })
            .collect::<HashMap<_, _>>();
        Ok(Workflows { workflows: rules })
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let (rules, parts) = str.split_once("\n\n").unwrap();

    let workflows: Workflows = rules.parse().unwrap();

    parts
        .lines()
        .map(|part| {
            part.trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|part| {
                    let (id, value) = part.split_once('=').unwrap();
                    (id.to_string(), value.parse::<usize>().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .filter(|part| workflows.process(part) == &Id::A)
        .map(|part| part.values().sum::<usize>())
        .sum::<usize>()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let (rules, _parts) = str.split_once("\n\n").unwrap();

    let workflows: Workflows = rules.parse().unwrap();

    let accepted_ranges = workflows.process_range(1, 4000);

    accepted_ranges
        .into_par_iter()
        .map(|part| {
            part.values()
                .map(|(min, max)| (*min..=*max))
                .multi_cartesian_product()
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"
            )),
            "19114"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"
            )),
            "167409079868000"
        );
    }
}
