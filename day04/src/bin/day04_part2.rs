use std::collections::HashSet;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use utils::{get_input_file_as_string, get_u64};

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */

    let get_card = map(
        tuple((
            preceded(tuple((tag("Card"), multispace1)), get_u64),
            tuple((char(':'), multispace1)),
            separated_list1(multispace1, get_u64),
            tuple((multispace0, char('|'), multispace0)),
            separated_list1(multispace1, get_u64),
        )),
        |(_, _, winners, _, cards)| (winners, cards),
    );

    let mut get_pile = map(separated_list1(char('\n'), get_card), |cards| cards);

    /* ----------------------------------------------------------------------------------------- */

    let (_, pile): (_, Vec<(Vec<u64>, Vec<u64>)>) =
        get_pile(data.as_str()).map_err(|err| err.to_owned())?;
    let values: Vec<u32> = pile
        .iter()
        .map(|(winners, draw)| {
            winners
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&draw.iter().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .len() as u32
        })
        .collect();

    let mut cards = vec![1; values.len()];
    for idx in 0..values.len() {
        for offs in 1..=values[idx] {
            cards[idx + offs as usize] += cards[idx];
        }
    }

    dbg!(cards.iter().sum::<u64>());
    Ok(())
}
