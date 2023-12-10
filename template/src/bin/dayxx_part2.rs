use anyhow::Result;
use utils::get_input_file_as_string;

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    // all_inputs(&data).map_err(|err| err.to_owned())?;

    Ok(())
}
