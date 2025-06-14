// Date Created: 05/06/2025.

use std::fs::read_to_string;

pub fn get_script_contents(scriptfile: &str) -> Result<String, std::io::Error> {
    let contents = read_to_string(scriptfile)?;

    Ok(contents)
}
