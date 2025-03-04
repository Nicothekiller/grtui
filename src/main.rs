use std::{fs::File, io::Read};

use grtui::Major;

/// Main function for the program. Starts execution.
///
/// # Panics
///
/// Panics if render function failed.
///
/// # Errors
///
/// This function will return an error if event::read fails.
fn main() -> std::io::Result<()> {
    let terminal = ratatui::init();

    let mut data_file = File::open("data.json").unwrap();
    let mut data_file_content = String::new();
    data_file.read_to_string(&mut data_file_content).unwrap();

    let mut major: Major = serde_json::from_str(data_file_content.as_str()).unwrap();

    major.render(terminal)?;

    ratatui::restore();
    Ok(())
}
