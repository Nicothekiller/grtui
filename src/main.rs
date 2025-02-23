use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

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

fn draw(frame: &mut Frame) {
    let text = Line::raw("Noooo la magia no es real no existe");
    let magia = Line::raw("Te falta amor").red();

    let par = Paragraph::new(vec![text, magia]).block(Block::bordered());

    frame.render_widget(par, frame.area());
}
