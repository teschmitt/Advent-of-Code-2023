use std::collections::HashMap;

use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    combinator::map,
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
};
use utils::get_input_file_as_string;

#[derive(Debug)]
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
struct Destination {
    left: String,
    right: String,
}

#[derive(Debug)]
struct CamelMap<'a>(HashMap<&'a str, Destination>);

impl<'a> From<Vec<(&'a str, Destination)>> for CamelMap<'a> {
    fn from(vec: Vec<(&'a str, Destination)>) -> Self {
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
        |(_, left, _, right, _): (_, &str, _, &str, _)| Destination {
            left: left.to_owned(),
            right: right.to_owned(),
        },
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

    let (_, (instrs, camel_map)) = all_inputs(&data).map_err(|err| err.to_owned())?;

    let mut current_loc = camel_map.get("AAA");
    let mut steps = 0;
    for instr in instrs.iter().cycle() {
        let current_node = match instr {
            Instruction::Left => &current_loc.left,
            Instruction::Right => &current_loc.right,
        };
        steps += 1;
        if current_node == "ZZZ" {
            break;
        }

        current_loc = camel_map.get(current_node);
    }
    dbg!(steps);
    Ok(())
}
