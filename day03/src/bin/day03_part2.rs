use std::collections::HashMap;

use anyhow::{Context, Result};
use utils::{get_input_file, Lines};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug)]
enum Position {
    Number(u8),
    Empty,
    Gear,
}

#[derive(Debug)]
struct Schematic {
    positions: HashMap<Coord, Position>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct Hit {
    value: u64,
    gear_coord: Coord,
}

impl Schematic {
    fn get_position(&self, row: usize, col: usize) -> &Position {
        let pos = Coord { row, col };
        self.positions
            .get(&pos)
            .context("Could not find position {pos}")
            .unwrap()
    }

    fn new(data: Lines) -> Result<Schematic> {
        let lines = data.collect::<Result<Vec<String>, _>>()?;
        let rows = lines.len();
        let cols = lines[0].len();
        let mut positions: HashMap<Coord, Position> = HashMap::new();
        let mut row = 0;
        for line in lines {
            let mut col = 0;
            for c in line.chars() {
                let pos = Coord { row, col };
                positions.insert(
                    pos,
                    if c.is_numeric() {
                        Position::Number(
                            c.to_digit(10).context("Char not a decimal digit").unwrap() as u8,
                        )
                    } else if c == '*' {
                        Position::Gear
                    } else {
                        Position::Empty
                    },
                );
                col += 1;
            }
            row += 1;
        }
        Ok(Schematic {
            positions,
            rows,
            cols,
        })
    }

    fn is_gear_part(&self, row: usize, col: usize) -> Option<Coord> {
        let t = row.saturating_sub(1);
        let l = col.saturating_sub(1);
        let b = if row + 1 >= self.rows { row } else { row + 1 };
        let r = if col + 1 >= self.cols { col } else { col + 1 };
        for row in t..=b {
            for col in l..=r {
                match self.get_position(row, col) {
                    Position::Gear => {
                        return Some(Coord { row, col });
                    }
                    _ => (),
                }
            }
        }
        return None;
    }

    fn get_gears(&self) -> Vec<Hit> {
        // save hits and gear position
        let mut res = vec![];
        for row in 0..self.rows {
            let mut is_part_nr = false;
            let mut nr: u64 = 0;
            let mut gear_part = None;
            for col in 0..self.cols {
                match self.get_position(row, col) {
                    Position::Number(x) => {
                        nr = (10 * nr) + (*x as u64);
                        if !is_part_nr {
                            gear_part = self.is_gear_part(row, col);
                            is_part_nr = gear_part.is_some();
                        }
                    }
                    _ => {
                        if is_part_nr {
                            res.push(Hit {
                                value: nr,
                                gear_coord: gear_part.clone().unwrap(),
                            });
                        }
                        nr = 0;
                        is_part_nr = false;
                    }
                }
            }
            if is_part_nr {
                res.push(Hit {
                    value: nr,
                    gear_coord: gear_part.unwrap(),
                });
            }
        }
        res
    }

    fn get_sum_gear_ratios(&self) -> u64 {
        let gears = self.get_gears();
        let mut sum = 0;
        let mut skip = 1;
        for gear in &gears {
            let part_two = gears
                .iter()
                .skip(skip)
                .filter(|g| g.gear_coord == gear.gear_coord)
                .last()
                .unwrap_or(&Hit {
                    value: 0,
                    gear_coord: Coord { row: 0, col: 0 },
                });
            sum += gear.value * part_two.value;
            skip += 1;
        }
        sum
    }
}

fn main() -> Result<()> {
    let data = get_input_file()?;
    let s = Schematic::new(data).unwrap();
    dbg!(s.get_sum_gear_ratios());
    Ok(())
}
