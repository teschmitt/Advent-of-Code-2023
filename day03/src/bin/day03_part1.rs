use std::collections::HashMap;

use anyhow::{Context, Result};
use utils::{get_input_file, Lines};

#[derive(Debug, Hash, Eq, PartialEq)]
struct RowCol {
    row: usize,
    col: usize,
}

#[derive(Debug)]
enum Position {
    Number(u8),
    Dot,
    Symbol,
}

#[derive(Debug)]
struct Schematic {
    positions: HashMap<RowCol, Position>,
    rows: usize,
    cols: usize,
}

impl Schematic {
    fn get_position(&self, row: usize, col: usize) -> &Position {
        let pos = RowCol { row, col };
        self.positions
            .get(&pos)
            .context("Could not find position {pos}")
            .unwrap()
    }

    fn new(data: Lines) -> Result<Schematic> {
        let lines = data.collect::<Result<Vec<String>, _>>()?;
        let rows = lines.len();
        let cols = lines[0].len();
        let mut positions: HashMap<RowCol, Position> = HashMap::new();
        let mut row = 0;
        for line in lines {
            let mut col = 0;
            for c in line.chars() {
                let pos = RowCol { row, col };
                positions.insert(
                    pos,
                    if c.is_numeric() {
                        Position::Number(
                            c.to_digit(10).context("Char not a decimal digit").unwrap() as u8,
                        )
                    } else if c == '.' {
                        Position::Dot
                    } else {
                        Position::Symbol
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

    fn is_part_number(&self, row: usize, col: usize) -> bool {
        let t = row.saturating_sub(1);
        let l = col.saturating_sub(1);
        let b = if row + 1 >= self.rows { row } else { row + 1 };
        let r = if col + 1 >= self.cols { col } else { col + 1 };
        for row in t..=b {
            for col in l..=r {
                match self.get_position(row, col) {
                    Position::Symbol => {
                        return true;
                    }
                    _ => (),
                }
            }
        }
        return false;
    }

    fn get_part_numbers(&self) -> Vec<u64> {
        let mut res = vec![];
        for row in 0..self.rows {
            let mut is_part_nr = false;
            let mut nr: u64 = 0;
            for col in 0..self.cols {
                match self.get_position(row, col) {
                    Position::Number(x) => {
                        nr = (10 * nr) + (*x as u64);
                        if !is_part_nr {
                            is_part_nr = self.is_part_number(row, col);
                        }
                    }
                    _ => {
                        if is_part_nr {
                            res.push(nr);
                        }
                        nr = 0;
                        is_part_nr = false;
                    }
                }
            }
            if is_part_nr {
                res.push(nr);
            }
        }
        res
    }
}

fn main() -> Result<()> {
    let data = get_input_file()?;
    let s = Schematic::new(data).unwrap();
    dbg!(s.get_part_numbers().iter().sum::<u64>());
    Ok(())
}
