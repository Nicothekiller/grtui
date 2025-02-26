use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame) {
    let text = Line::raw("Linea 1");
    let magia = Line::raw("Linea 2").red();

    let par = Paragraph::new(vec![text, magia]).block(Block::bordered());

    frame.render_widget(par, frame.area());
}
