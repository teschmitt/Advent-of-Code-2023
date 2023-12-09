use anyhow::Result;
use nom::{
    character::complete::{line_ending, space1},
    combinator::map,
    multi::separated_list1,
};
use utils::{get_input_file_as_string, get_num};

fn extrapolate(values: Vec<i64>) -> Vec<i64> {
    if values.iter().all(|&v| v == 0) {
        vec![0].into_iter().chain(values).collect()
    } else {
        let x = get_first_or_zero(&extrapolate(get_differences(&values)));
        let v = get_first_or_zero(&values);
        vec![v - x].into_iter().chain(values).collect()
    }
}

fn get_differences(values: &Vec<i64>) -> Vec<i64> {
    values
        .windows(2)
        .map(|w| w.last().unwrap_or(&0) - get_first_or_zero(w))
        .collect()
}

fn get_first_or_zero(values: &[i64]) -> i64 {
    *values.first().unwrap_or(&0)
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    let get_i64 = get_num::<i64>;
    let one_line = map(separated_list1(space1, get_i64), |v| v);
    let mut all_inputs = map(separated_list1(line_ending, one_line), |l| l);

    let (_, series) = all_inputs(&data).map_err(|err| err.to_owned())?;
    let res: i64 = series
        .iter()
        .map(|hist| get_first_or_zero(&extrapolate(hist.clone())))
        .sum();
    dbg!(res);
    Ok(())
}
