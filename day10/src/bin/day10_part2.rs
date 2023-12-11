use anyhow::{Context, Result};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map},
    error::ErrorKind,
    multi::{many1, separated_list1},
};
use std::collections::HashMap;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    /// approximate the start tile. It's enough to know if its NorthSouth or not so we can count it
    /// correctly later
    fn get_start_maptile(&self) -> MapTile {
        use MapTile::*;
        let mut dir = self.get_start_adjacents();
        dir.sort();
        // N, E, W, S sorted order
        match dir {
            [Direction::North, Direction::South] => NorthSouth,
            [Direction::North, Direction::East] => NorthEast,
            [Direction::North, Direction::West] => NorthWest,
            [Direction::East, Direction::South] => SouthEast,
            [Direction::East, Direction::West] => EastWest,
            [Direction::West, Direction::South] => SouthWest,
            d => panic!("Big problem, weird directions: {d:?}"),
        }
    }

    /// fetches the direction the adjacents are located at and their coords
    fn get_start_adjacents(&self) -> [Direction; 2] {
        use MapTile::*;
        let start = &self.start_coord;
        let mut dir: Vec<Direction> = vec![];
        if start.row > 0 {
            match self.get(&(start.row - 1, start.col).into()) {
                SouthEast | SouthWest | NorthSouth => {
                    dir.push(Direction::North);
                }
                _ => (),
            };
        };
        if start.row < self.rows - 1 {
            match self.get(&(start.row + 1, start.col).into()) {
                NorthEast | NorthWest | NorthSouth => {
                    dir.push(Direction::South);
                }
                _ => (),
            };
        };
        if start.col > 0 {
            match self.get(&(start.row, start.col - 1).into()) {
                SouthEast | NorthEast | EastWest => {
                    dir.push(Direction::West);
                }
                _ => (),
            };
        };
        if start.col < self.cols - 1 {
            match self.get(&(start.row, start.col + 1).into()) {
                NorthWest | SouthWest | EastWest => {
                    dir.push(Direction::East);
                }
                _ => (),
            };
        };
        assert_eq!(
            dir.len(),
            2,
            "{} adjecents to the start node found this is very concerning, since there should be exactly 2!", dir.len()
        );

        let mut dir_iter = dir.into_iter();
        [dir_iter.next().unwrap(), dir_iter.next().unwrap()]
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

    let mut clean_map = FieldMap {
        map: HashMap::new(),
        rows: field_map.rows,
        cols: field_map.cols,
        start_coord: field_map.start_coord.clone(),
    };

    // build a clean map in which we can later on count the vertical paths
    let mut prev_coord = field_map.start_coord.clone();
    let mut cur_coord: Coord = match field_map.get_start_adjacents().first().unwrap() {
        Direction::North => (prev_coord.row - 1, prev_coord.col).into(),
        Direction::East => (prev_coord.row, prev_coord.col - 1).into(),
        Direction::West => (prev_coord.row, prev_coord.col + 1).into(),
        Direction::South => (prev_coord.row + 1, prev_coord.col).into(),
    };

    while cur_coord != field_map.start_coord {
        clean_map
            .map
            .insert(cur_coord.clone(), field_map.get(&cur_coord).clone());
        let new_coord = field_map.get_next(&prev_coord, &cur_coord);
        prev_coord = cur_coord;
        cur_coord = new_coord.clone();
    }
    // fill in start node
    clean_map
        .map
        .insert(cur_coord, field_map.get_start_maptile());

    let mut area = 0;
    let mut inside = false;
    let (rows, cols) = (clean_map.rows, clean_map.cols);

    // get ready for diagonal scanning!
    for sum in 0..(rows + cols - 1) {
        let mut row = sum;
        if row >= rows {
            row = rows - 1;
        }

        let mut col = sum - row;
        if col >= cols {
            col = cols - 1;
        }

        while row > 0 && col < cols - 1 {
            if let Some(tile) = clean_map.map.get(&(row, col).into()) {
                match tile {
                    MapTile::NorthWest | MapTile::SouthEast => (),
                    MapTile::NorthEast
                    | MapTile::NorthSouth
                    | MapTile::EastWest
                    | MapTile::SouthWest => {
                        inside = !inside;
                    }
                    t => panic!("Encountered unexpected tile type '{t:?}'"),
                };
            } else {
                if inside {
                    area += 1;
                }
            }
            row -= 1;
            col += 1;
        }
        if let Some(tile) = clean_map.map.get(&(row, col).into()) {
            match tile {
                MapTile::NorthWest | MapTile::SouthEast => (),
                MapTile::NorthEast
                | MapTile::NorthSouth
                | MapTile::EastWest
                | MapTile::SouthWest => {
                    inside = !inside;
                }
                t => panic!("Encountered unexpected tile type '{t:?}'"),
            };
        } else {
            if inside {
                area += 1;
            }
        };
    }

    dbg!(area);
    Ok(())
}
