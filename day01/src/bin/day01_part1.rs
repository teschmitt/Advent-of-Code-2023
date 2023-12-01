use anyhow::{Context, Result};
use utils::get_input_file;

fn main() -> Result<()> {
    let data = get_input_file()?;
    let mut sum = 0;
    data.for_each(|calbr| {
        let nums = calbr
            .context("Line is not a string")
            .unwrap()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        let val = (10 * nums.first().unwrap()) + nums.last().unwrap();
        sum += val;
    });
    dbg!(sum);
    Ok(())
}
