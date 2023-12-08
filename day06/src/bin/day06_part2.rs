use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, one_of, space1},
    combinator::{all_consuming, map, map_res, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use utils::get_input_file_as_string;

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    let mut get_races = map(
        all_consuming(tuple((
            preceded(
                tuple((tag("Time:"), multispace1)),
                tuple((space_sep_decimal, newline)),
            ),
            preceded(tuple((tag("Distance:"), multispace1)), space_sep_decimal),
        ))),
        |((times, _), distances)| (times, distances),
    );
    let (_, (time, distance)) = get_races(&data).map_err(|err| err.to_owned())?;

    let mut res = 1;
    let mut wins = 0;
    for speed in 1..time {
        let time_left = time - speed;
        if time_left * speed > distance {
            wins += 1;
        }
    }
    res *= wins;

    dbg!(res);
    Ok(())
}

fn space_sep_decimal(input: &str) -> IResult<&str, u64> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(space1)))),
        |out: &str| str::replace(&out, " ", "").parse::<u64>(),
    )(input)
}
