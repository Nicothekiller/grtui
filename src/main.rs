use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::Stylize,
    widgets::{Block, Paragraph},
    Frame,
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw).expect("failed to draw frame");

        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                break;
            }
            _ => {
                continue;
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let widget = Block::bordered();

    let text = Paragraph::new("BATTLER SOY UN TUI").red().block(widget);
    frame.render_widget(text, frame.area());
}
