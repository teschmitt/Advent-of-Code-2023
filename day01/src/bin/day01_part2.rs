use anyhow::Result;
use std::{cmp::min, collections::HashMap};
use utils::get_input_file;

fn replace_str_with_num(s: String, table: &HashMap<&str, &str>) -> String {
    // basic idea: shift window across string, replace name of number with number in each window,
    // append the (modified) window to the result string
    let mut res = String::new();
    let mut start_idx = 0;

    loop {
        let end_idx = min(start_idx + 5, s.len());
        let win = s[start_idx..end_idx].to_owned();
        let mut rep = String::new();
        for key in table.keys() {
            rep = win.replace(key, table.get(key).unwrap());
            if rep.len() < win.len() {
                // we found a name so the replaced string is shorter
                break;
            }
        }
        // if no name was found, we just append the window since it may still contain numbers
        res = format!("{res}{rep}{win}");

        if start_idx >= s.len() {
            res = res.chars().filter(|c| c.is_digit(10)).collect();
            break res;
        }
        start_idx += 1;
    }
}

fn main() -> Result<()> {
    let num_table: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let data = get_input_file()?;
    let mut sum = 0;

    data.map(|s| replace_str_with_num(s.unwrap(), &num_table))
        .for_each(|calbr| {
            let nums = calbr
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            let val = (10 * nums.first().unwrap()) + nums.last().unwrap();
            sum += val;
        });
    dbg!(sum);
    Ok(())
}
