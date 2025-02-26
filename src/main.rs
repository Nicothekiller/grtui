use crossterm::event::{self, Event, KeyCode};
use gtui::draw;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw).expect("failed to draw frame");

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => continue,
            },
            _ => continue,
        }
    }

    ratatui::restore();
    Ok(())
}
