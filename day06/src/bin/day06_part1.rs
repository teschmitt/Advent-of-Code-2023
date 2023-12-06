use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use utils::{get_input_file_as_string, get_u64};

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    let mut get_races = map(
        all_consuming(tuple((
            preceded(
                tuple((tag("Time:"), multispace1)),
                separated_list1(multispace1, get_u64),
            ),
            preceded(
                tuple((char('\n'), tag("Distance:"), multispace1)),
                separated_list1(multispace1, get_u64),
            ),
        ))),
        |(times, distances)| (times, distances),
    );
    let (_, (times, distances)) = get_races(&data).map_err(|err| err.to_owned())?;

    let mut res = 1;
    for race in 0..times.len() {
        let time = times[race];
        let mut wins = 0;
        for speed in 1..time {
            let time_left = time - speed;
            if time_left * speed > distances[race] {
                wins += 1;
            }
        }
        res *= wins;
    }
    dbg!(res);
    Ok(())
}
