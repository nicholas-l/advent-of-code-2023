use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use num_integer::Integer;

#[derive(Debug)]
enum Module {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Circuit<'a> {
    modules: HashMap<&'a str, Module>,
    outputs: HashMap<&'a str, Vec<&'a str>>,
    inverse: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Circuit<'a> {
    fn from_str(s: &'a str) -> Result<Self, String> {
        let mut outputs = HashMap::new();
        let mut inverse: HashMap<&str, Vec<&str>> = HashMap::new();

        let modules = s
            .lines()
            .map(|line| {
                let (input, output) = line.split_once(" -> ").unwrap();

                let output = output.split(',').map(|s| s.trim()).collect::<Vec<_>>();

                let module = match input.chars().next().unwrap() {
                    '%' => (&input[1..], Module::FlipFlop),
                    '&' => (&input[1..], Module::Conjunction),
                    'b' if input == "broadcaster" => (input, Module::Broadcast),
                    _ => return Err(format!("Invalid input {}", input)),
                };

                outputs.insert(module.0, output.clone());

                output.iter().for_each(|&output| {
                    inverse.entry(output).or_default().push(module.0);
                });

                Ok(module)
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            modules,
            outputs,
            inverse,
        })
    }
}

struct CircuitState<'a> {
    flipflops: HashMap<&'a str, bool>,
    conjunctions: HashMap<&'a str, HashMap<&'a str, bool>>,
}

impl<'a> CircuitState<'a> {
    fn construct(circuit: &'a Circuit) -> Self {
        let conjunction_inputs = circuit
            .modules
            .iter()
            .filter_map(|(name, module)| match module {
                Module::Conjunction => Some(name),
                _ => None,
            })
            .flat_map(|to| {
                circuit
                    .inverse
                    .get(to)
                    .unwrap()
                    .iter()
                    .map(move |from| (to, from))
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<&str, HashMap<&str, bool>>, (to, from)| {
                    acc.entry(to).or_default().insert(from, false);
                    acc
                },
            );

        Self {
            flipflops: HashMap::new(),
            conjunctions: conjunction_inputs,
        }
    }
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let circuit = Circuit::from_str(&str).unwrap();

    let mut circuit_state = CircuitState::construct(&circuit);
    let mut low_count = 0;
    let mut high_count = 0;

    let mut queue = VecDeque::new();

    for _i in 0..1000 {
        queue.push_back(("button", "broadcaster", false));

        while let Some(pulse) = queue.pop_front() {
            let (from_node, node, pulse) = pulse;

            // println!("{} -> {} ({})", from_node, node, pulse);

            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }

            if !circuit.modules.contains_key(node) {
                // println!("Unable to find {}", node);
                continue;
            }

            match circuit
                .modules
                .get(node)
                .unwrap_or_else(|| panic!("Unable to find {}", node))
            {
                Module::Broadcast => {
                    let outputs = circuit.outputs.get(node).unwrap();
                    for output in outputs {
                        queue.push_back((node, output, pulse));
                    }
                }
                Module::FlipFlop => {
                    let outputs = circuit.outputs.get(node).unwrap();
                    let is_on = circuit_state.flipflops.entry(node).or_insert(false);

                    if pulse {
                        // We received a high pulse so don't send anything
                        continue;
                    }

                    let pulse_to_send = if *is_on {
                        *is_on = false;
                        false
                    } else {
                        *is_on = true;
                        true
                    };
                    for output in outputs {
                        queue.push_back((node, output, pulse_to_send));
                    }
                }
                Module::Conjunction => {
                    let outputs = circuit.outputs.get(node).unwrap();
                    let conjunction = circuit_state.conjunctions.entry(node).or_default();

                    let previous_pulse: &mut bool = conjunction
                        .get_mut(from_node)
                        .unwrap_or_else(|| panic!("Unable to find {} in", from_node));
                    *previous_pulse = pulse;

                    let pulse_to_send = !conjunction.values().all(|pulse| *pulse);

                    for output in outputs {
                        queue.push_back((node, output, pulse_to_send));
                    }
                }
            }
        }
    }

    (high_count * low_count).to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let circuit = Circuit::from_str(&str).unwrap();

    let mut circuit_state = CircuitState::construct(&circuit);

    let mut queue = VecDeque::new();

    // This is slighlty hardcoded the input into the code. We have looked back from the 'rx' node
    // and found the nodes that are connected to it. These nodes are Conjuction nodes which mean we
    // have to find the lowest common multiple of the button presses it takes for each of these
    // nodes to receive a low pulse.
    let rx_inputs = circuit.inverse.get("rx").unwrap();
    let lcm_inputs = rx_inputs
        .iter()
        .flat_map(|id| circuit.inverse.get(id).unwrap().iter())
        .collect::<HashSet<_>>();

    let mut first_low = HashMap::new();
    let mut second_low = HashMap::new();

    for i in 0.. {
        queue.push_back(("button", "broadcaster", false));

        while let Some(pulse) = queue.pop_front() {
            let (from_node, node, is_high) = pulse;

            if lcm_inputs.contains(&node) && !is_high {
                if first_low.contains_key(node) && !second_low.contains_key(node) {
                    second_low.insert(node, i);
                } else if !first_low.contains_key(node) {
                    first_low.insert(node, i);
                }
            }
            if second_low.len() == lcm_inputs.len() {
                return lcm_inputs
                    .into_iter()
                    .map(|id| (second_low.get(id).unwrap() - first_low.get(id).unwrap()) as usize)
                    .reduce(|a, b| a.lcm(&b))
                    .unwrap()
                    .to_string();
            }

            // This is unlikely to be triggered but included for completeness
            if node == "rx" && !is_high {
                return i.to_string();
            }

            match circuit.modules.get(node) {
                Some(Module::Broadcast) => {
                    for output in circuit.outputs.get(node).unwrap() {
                        queue.push_back((node, output, is_high));
                    }
                }
                Some(Module::FlipFlop) => {
                    // We received a low pulse so process the pulse
                    if !is_high {
                        let is_on = circuit_state.flipflops.entry(node).or_insert(false);
                        *is_on = !*is_on;

                        for output in circuit.outputs.get(node).unwrap() {
                            queue.push_back((node, output, *is_on));
                        }
                    }
                }
                Some(Module::Conjunction) => {
                    let conjunction = circuit_state.conjunctions.get_mut(node).unwrap();
                    conjunction.insert(from_node, is_high);

                    let pulse_to_send = !conjunction.values().all(|pulse| *pulse);

                    for output in circuit.outputs.get(node).unwrap() {
                        queue.push_back((node, output, pulse_to_send));
                    }
                }
                None => {}
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            )),
            "32000000"
        );

        assert_eq!(
            star_one(Cursor::new(
                b"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            )),
            "11687500"
        );
    }

    // #[test]
    // fn test_star_two() {
    //     assert_eq!(star_two(Cursor::new(b"")), "167409079868000");
    // }
}
