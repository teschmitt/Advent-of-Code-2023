use std::collections::HashMap;

use anyhow::{Context, Result};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map},
    error::ErrorKind,
    multi::{many1, separated_list1},
};
use utils::get_input_file_as_string;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Coord {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(pos: (usize, usize)) -> Self {
        Coord {
            row: pos.0,
            col: pos.1,
        }
    }
}

impl From<Coord> for (usize, usize) {
    fn from(coord: Coord) -> Self {
        (coord.row, coord.col)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MapTile {
    NorthEast,
    NorthWest,
    NorthSouth,
    EastWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

impl<'a> From<&'a char> for MapTile {
    fn from(s: &'a char) -> Self {
        match s {
            'S' => MapTile::Start,
            '.' => MapTile::Ground,
            '|' => MapTile::NorthSouth,
            '-' => MapTile::EastWest,
            'L' => MapTile::NorthEast,
            'J' => MapTile::NorthWest,
            '7' => MapTile::SouthWest,
            'F' => MapTile::SouthEast,
            _ => panic!("Encountered unknown map tile"),
        }
    }
}

impl From<Vec<Vec<MapTile>>> for FieldMap {
    fn from(map_data: Vec<Vec<MapTile>>) -> Self {
        let mut start_tile = Coord::default();
        let mut res = HashMap::new();
        for (row_num, row) in map_data.iter().enumerate() {
            for (col_num, tile) in row.into_iter().enumerate() {
                res.insert((row_num, col_num).into(), tile.clone());
                if tile == &MapTile::Start {
                    start_tile = (row_num, col_num).into();
                }
            }
        }
        FieldMap {
            map: res,
            rows: map_data.len(),
            cols: map_data[0].len(),
            start_coord: start_tile,
        }
    }
}

impl FieldMap {
    fn get(&self, pos: &Coord) -> &MapTile {
        self.map
            .get(&pos)
            .context("No tile found at ({row}, {col})")
            .unwrap()
    }

    fn get_next(&self, prev: &Coord, cur: &Coord) -> Coord {
        self.get_adjecent(&cur)
            .iter()
            .filter(|&c| c != prev)
            .cloned()
            .last()
            .unwrap()
    }

    fn get_adjecent(&self, coord: &Coord) -> [Coord; 2] {
        use MapTile::*;
        let (row, col) = coord.clone().into();
        match self.get(&coord) {
            NorthEast => [(row, col + 1).into(), (row - 1, col).into()],
            NorthWest => [(row, col - 1).into(), (row - 1, col).into()],
            NorthSouth => [(row - 1, col).into(), (row + 1, col).into()],
            EastWest => [(row, col + 1).into(), (row, col - 1).into()],
            SouthEast => [(row, col + 1).into(), (row + 1, col).into()],
            SouthWest => [(row, col - 1).into(), (row + 1, col).into()],
            Ground | Start => {
                panic!("Find Start adjacents with start_adjacents(), Ground has no adjacents");
            }
        }
    }

    fn start_adjacent(&self) -> Coord {
        use MapTile::*;
        let start = &self.start_coord;
        if start.row > 0 {
            match self.get(&(start.row - 1, start.col).into()) {
                SouthEast | SouthWest | NorthSouth => return (start.row - 1, start.col).into(),
                _ => (),
            };
        };
        if start.row < self.rows - 1 {
            match self.get(&(start.row + 1, start.col).into()) {
                NorthEast | NorthWest | NorthSouth => return (start.row + 1, start.col).into(),
                _ => (),
            };
        };
        if start.col > 0 {
            match self.get(&(start.row, start.col - 1).into()) {
                SouthEast | NorthEast | EastWest => return (start.row, start.col - 1).into(),
                _ => (),
            };
        };
        if start.col < self.cols - 1 {
            match self.get(&(start.row, start.col + 1).into()) {
                NorthWest | SouthWest | EastWest => return (start.row, start.col + 1).into(),
                _ => (),
            };
        };
        panic!("No adjecent to Start node found!");
    }
}

#[derive(Debug)]
struct FieldMap {
    map: HashMap<Coord, MapTile>,
    rows: usize,
    cols: usize,
    start_coord: Coord,
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */
    let tile = map(one_of::<_, _, (&str, ErrorKind)>("S.|-LJ7F"), |t| {
        MapTile::from(&t)
    });
    let map_line = || many1(tile);
    let all_inputs = || all_consuming(separated_list1(line_ending, map_line()));
    /* ----------------------------------------------------------------------------------------- */

    let field_map = FieldMap::from(all_inputs()(&data.trim()).map_err(|err| err.to_owned())?.1);

    let mut jumps: usize = 1;
    let mut cur_coord = field_map.start_adjacent();
    let mut prev_coord = field_map.start_coord.clone();
    while cur_coord != field_map.start_coord {
        // println!(
        //     "J{jumps} from ({}, {}) to ({}, {}) {:?}",
        //     prev_coord.row,
        //     prev_coord.col,
        //     cur_coord.row,
        //     cur_coord.col,
        //     field_map.get(&cur_coord)
        // );
        // std::thread::sleep(std::time::Duration::from_millis(100));
        let new_coord = field_map.get_next(&prev_coord, &cur_coord);
        prev_coord = cur_coord;
        cur_coord = new_coord.clone();
        jumps += 1;
    }
    dbg!(jumps / 2);
    Ok(())
}
