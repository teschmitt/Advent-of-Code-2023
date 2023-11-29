use anyhow::Result;
use utils::get_input_file;

fn main() -> Result<()> {
    let data = get_input_file()?;
    let x: Vec<u8> = data
        .filter_map(|s| {
            if let Ok(s) = s {
                if let Some((_, n)) = s.rsplit_once('r') {
                    if let Ok(u) = n.parse::<u8>() {
                        return Some(u);
                    }
                }
            }
            None
        })
        .collect();
    println!("{:?}", x);
    Ok(())
}
