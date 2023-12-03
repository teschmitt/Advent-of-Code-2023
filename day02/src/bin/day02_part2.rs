use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    combinator::{map, map_res, recognize},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashMap;
use utils::get_input_file_as_string;

#[derive(Debug)]
struct GameSet {
    games: Vec<Game>,
}

impl GameSet {
    fn sum_of_powers(&self) -> u64 {
        self.games.iter().map(|g| g.power()).sum()
    }
}

#[derive(Debug)]
struct Game {
    grabs: Vec<Grab>,
}

impl Game {
    fn power(&self) -> u64 {
        let g = self.fewest_cubes_possible();
        g.red * g.green * g.blue
    }
    fn fewest_cubes_possible(&self) -> Grab {
        self.grabs
            .iter()
            .cloned()
            .reduce(|acc, e| Grab {
                red: acc.red.max(e.red),
                green: acc.green.max(e.green),
                blue: acc.blue.max(e.blue),
            })
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn new(color: &str) -> Self {
        match color {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("No valid color supplied"),
        }
    }
}

#[derive(Clone, Debug)]
struct Grab {
    red: u64,
    green: u64,
    blue: u64,
}

impl Grab {
    pub fn new(cubes: Vec<(Color, u64)>) -> Self {
        let h: HashMap<_, _> = cubes.into_iter().collect();
        Grab {
            red: *h.get(&Color::Red).unwrap_or(&0),
            green: *h.get(&Color::Green).unwrap_or(&0),
            blue: *h.get(&Color::Blue).unwrap_or(&0),
        }
    }
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */

    let get_cube = map(
        tuple((
            get_u64,
            multispace1,
            alt((tag("red"), tag("green"), tag("blue"))),
        )),
        |(n, _, color)| (Color::new(color), n),
    );

    let get_grab = map(separated_list1(tag(", "), get_cube), |cubes| {
        Grab::new(cubes)
    });

    let get_game = map(
        tuple((
            preceded(tag("Game "), get_u64),
            tuple((char(':'), multispace1)),
            separated_list1(tuple((char(';'), multispace1)), get_grab),
        )),
        |(_, _, grabs)| Game { grabs },
    );

    let mut get_game_set = map(separated_list1(char('\n'), get_game), |games| GameSet {
        games,
    });

    /* ----------------------------------------------------------------------------------------- */

    let (_, game_set) = get_game_set(data.as_str()).unwrap();
    dbg!(game_set.sum_of_powers());
    Ok(())
}

fn get_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}
