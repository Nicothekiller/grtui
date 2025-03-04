use std::{fs::File, io::Read};

use grtui::{Major, Tui};

/// Main function for the program. Starts execution.
///
/// # Panics
///
/// This function will panic if reading the json file fails.
///
/// # Errors
///
/// This function will return an error if [render](grtui::tui::Tui::render) fails.
fn main() -> std::io::Result<()> {
    let terminal = ratatui::init();

    let mut data_file = File::open("data.json").unwrap();
    let mut data_file_content = String::new();
    data_file.read_to_string(&mut data_file_content).unwrap();

    let major: Major = serde_json::from_str(data_file_content.as_str()).unwrap();

    let mut tui = Tui::new(major);

    tui.render(terminal)?;

    ratatui::restore();
    Ok(())
}
