use anyhow::Result;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    error::ErrorKind,
    multi::{many1, separated_list1},
};
use utils::get_input_file_as_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

#[derive(Debug)]
struct MapImage {
    // galaxy_map: Vec<Vec<MapTile>>,
    galaxy_pos: Vec<Coord>,
}

impl MapImage {
    fn new(m: Vec<Vec<MapTile>>) -> Self {
        let mut galaxy_map = vec![];
        for row in m {
            let mut r: Vec<MapTile> = vec![];
            for col in row {
                r.push(col);
            }
            galaxy_map.push(r);
        }
        expand(&mut galaxy_map);

        MapImage {
            // galaxy_map: galaxy_map.clone(),
            galaxy_pos: chart(&galaxy_map),
        }
    }
}

fn expand(galaxy_map: &mut Vec<Vec<MapTile>>) {
    let mut width = galaxy_map.len();
    let empty_row = vec![MapTile::Empty; width];

    let mut row_idx = 0;
    loop {
        if let Some(row) = galaxy_map.get(row_idx) {
            if row.iter().all(|c| c == &MapTile::Empty) {
                galaxy_map.insert(row_idx + 1, empty_row.clone());
                row_idx += 1;
            }
            row_idx += 1;
        } else {
            break;
        }
    }
    let mut col_idx = 0;
    loop {
        if col_idx >= width {
            break;
        }
        // we're just going to trust that all lines are equally long haha
        let col = galaxy_map.iter().map(|c| c[col_idx]).collect::<Vec<_>>();
        if col.iter().all(|c| c == &MapTile::Empty) {
            galaxy_map
                .iter_mut()
                .for_each(|row| row.insert(col_idx, MapTile::Empty));
            width += 1;
            col_idx += 1;
        }
        col_idx += 1;
    }
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
    let mut all_inputs = map(separated_list1(line_ending, map_line()), |m| {
        MapImage::new(m)
    });
    /* ----------------------------------------------------------------------------------------- */

    let (_, map_image) = all_inputs(&data).map_err(|err| err.to_owned())?;

    let res: usize = map_image
        .galaxy_pos
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
                map_image.galaxy_pos.iter().skip(i + 1).map(
                    move |&Coord {
                              row_idx: row_dest,
                              col_idx: col_dest,
                          }| {
                        let diff = row_dest.abs_diff(row_orig) + col_dest.abs_diff(col_orig);
                        // println!("{row_orig}, {col_orig} -> {row_dest}, {col_dest} == {d}");
                        diff
                    },
                )
            },
        )
        .sum();
    dbg!(res);

    Ok(())
}
