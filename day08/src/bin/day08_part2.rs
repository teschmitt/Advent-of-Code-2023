use std::collections::HashMap;

use anyhow::{Context, Result};
use gcd::Gcd;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    combinator::map,
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
};
use utils::get_input_file_as_string;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

impl From<&char> for Instruction {
    fn from(value: &char) -> Self {
        match value {
            'L' => Instruction::Left,
            _ => Instruction::Right,
        }
    }
}

#[derive(Debug)]
struct Destination<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
struct CamelMap<'a>(HashMap<&'a str, Destination<'a>>);

impl<'a> From<Vec<(&'a str, Destination<'a>)>> for CamelMap<'a> {
    fn from(vec: Vec<(&'a str, Destination<'a>)>) -> Self {
        CamelMap(vec.into_iter().collect())
    }
}

impl CamelMap<'_> {
    fn get(&self, key: &str) -> &Destination {
        self.0
            .get(key)
            .context(format!("Key '{key}' not found"))
            .unwrap()
    }
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */

    let get_instr = map(many1(one_of::<_, _, (&str, ErrorKind)>("LR")), |v| {
        v.iter()
            .map(|instr| instr.into())
            .collect::<Vec<Instruction>>()
    });

    let left_right = map(
        tuple((tag("("), alphanumeric1, tag(", "), alphanumeric1, tag(")"))),
        |(_, left, _, right, _): (_, &str, _, &str, _)| Destination { left, right },
    );

    let locations = map(
        separated_list1(
            line_ending,
            tuple((terminated(alphanumeric1, tag(" = ")), left_right)),
        ),
        |lines| lines,
    );

    let mut all_inputs = map(
        tuple((get_instr, many1(line_ending), locations)),
        |(instr, _, nodes)| {
            (
                instr,
                CamelMap::from(
                    nodes
                        .into_iter()
                        .map(|(key, value)| (key, value))
                        .collect::<Vec<_>>(),
                ),
            )
        },
    );

    /* ----------------------------------------------------------------------------------------- */

    let (_, (instrs, camel_map)): (_, (Vec<Instruction>, CamelMap)) =
        all_inputs(&data).map_err(|err| err.to_owned())?;

    let mut instr_cycle = instrs.iter().cycle();

    let mut total_cycles: u64 = 0;
    for &start_node in camel_map.0.keys().filter(|&node| node.ends_with("A")) {
        let mut steps = 0;
        let mut cur_node = start_node;
        let mut first_round: u64 = 0;
        while !cur_node.ends_with("Z") || first_round == 0 {
            if cur_node.ends_with("Z") && first_round == 0 {
                first_round = steps;
                steps = 0;
            } else if cur_node.ends_with("Z") {
                // loop from ..Z back to ..Z done
                break;
            }
            let dests = camel_map.get(cur_node);
            cur_node = match instr_cycle.next().context("No instruction found").unwrap() {
                Instruction::Left => dests.left,
                Instruction::Right => dests.right,
            };
            steps += 1;
        }
        if steps != first_round {
            // in this case, the calculation in the following lines will not work and the solution
            // will be a bit more involved.
            panic!("Getting to first destination took {first_round} steps, doing the loop took {steps}");
        }
        if total_cycles > 0 {
            total_cycles = total_cycles * steps / total_cycles.gcd(steps);
        } else {
            total_cycles = steps;
        }
    }
    dbg!(total_cycles);

    Ok(())
}
