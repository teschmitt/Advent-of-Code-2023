use anyhow::Result;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    error::ErrorKind,
    multi::{many1, separated_list1},
};
use utils::get_input_file_as_string;

const EXPAND: usize = 1000000;

#[derive(PartialEq)]
enum MapTile {
    Empty,
    Galaxy,
}

impl From<&char> for MapTile {
    fn from(c: &char) -> Self {
        match c {
            '.' => MapTile::Empty,
            '#' => MapTile::Galaxy,
            c => panic!("Unknown map symbol: '{c}'"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord [{} {}]", self.row_idx, self.col_idx)
    }
}

fn expand(mut chart: Vec<Coord>) -> Vec<Coord> {
    chart.sort();
    // expand rows
    let mut scan_idx = 1;
    while scan_idx < chart.len() {
        let row_diff = chart[scan_idx - 1]
            .row_idx
            .abs_diff(chart[scan_idx].row_idx);
        if row_diff > 1 {
            for idx in scan_idx..chart.len() {
                chart[idx].row_idx += (EXPAND - 1) * (row_diff - 1);
            }
        }
        scan_idx += 1;
    }
    chart.sort_by_key(
        |&Coord {
             row_idx: _,
             col_idx,
         }| col_idx,
    );

    scan_idx = 1;
    while scan_idx < chart.len() {
        let col_diff = chart[scan_idx - 1]
            .col_idx
            .abs_diff(chart[scan_idx].col_idx);
        if col_diff > 1 {
            for idx in scan_idx..chart.len() {
                chart[idx].col_idx += (EXPAND - 1) * (col_diff - 1);
            }
        }
        scan_idx += 1;
    }
    chart
}

fn chart(galaxy_map: &Vec<Vec<MapTile>>) -> Vec<Coord> {
    let mut galaxy_pos = vec![];
    for (row_idx, row) in galaxy_map.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if col == &MapTile::Galaxy {
                galaxy_pos.push(Coord { row_idx, col_idx });
            }
        }
    }
    galaxy_pos
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */
    let map_tile = map(one_of::<_, _, (&str, ErrorKind)>(".#"), |t| {
        MapTile::from(&t)
    });
    let map_line = || many1(map_tile);
    let all_inputs = || separated_list1(line_ending, map_line());
    /* ----------------------------------------------------------------------------------------- */

    let (_, map_image) = all_inputs()(&data).map_err(|err| err.to_owned())?;

    let chart = expand(chart(&map_image));

    let res: usize = chart
        .iter()
        .enumerate()
        .flat_map(
            |(
                i,
                &Coord {
                    row_idx: row_orig,
                    col_idx: col_orig,
                },
            )| {
                chart.iter().skip(i + 1).map(
                    move |&Coord {
                              row_idx: row_dest,
                              col_idx: col_dest,
                          }| {
                        let diff = row_dest.abs_diff(row_orig) + col_dest.abs_diff(col_orig);
                        // println!("{row_orig}, {col_orig} -> {row_dest}, {col_dest} == {diff}");
                        diff
                    },
                )
            },
        )
        .sum();
    dbg!(res);

    Ok(())
}
