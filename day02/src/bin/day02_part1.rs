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
    bag: Grab,
    games: Vec<Game>,
}

impl GameSet {
    fn get_possible_games(&self) -> Vec<u64> {
        let mut res: Vec<u64> = vec![];
        for game in &self.games {
            if game.all_grabs_possible(&self.bag) {
                res.push(game.nr)
            }
        }
        res
    }
}

#[derive(Debug)]
struct Game {
    nr: u64,
    grabs: Vec<Grab>,
}

impl Game {
    fn all_grabs_possible(&self, bag: &Grab) -> bool {
        for g in &self.grabs {
            if g.blue > bag.blue || g.red > bag.red || g.green > bag.green {
                return false;
            }
        }
        true
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
    let bag = Grab {
        red: 12,
        green: 13,
        blue: 14,
    };

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
        |(nr, _, grabs)| Game { nr, grabs },
    );

    let mut get_game_set = map(separated_list1(char('\n'), get_game), |games| GameSet {
        bag: bag.clone(),
        games,
    });

    /* ----------------------------------------------------------------------------------------- */

    let (_, game_set) = get_game_set(data.as_str()).unwrap();
    let x: u64 = game_set.get_possible_games().iter().sum();
    dbg!(x);
    Ok(())
}

fn get_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}
