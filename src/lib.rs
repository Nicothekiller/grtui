use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

/// Draw fn to use with ratatui. Draws the events happening on the screen.
pub fn draw(frame: &mut Frame) {
    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]);
    let [semester_area, info_area] = horizontal.areas(frame.area());

    let semester_block = Block::bordered().title(Span::from("Semestres").green());
    let info_block = Block::bordered().title("Info");

    let s_lines: Vec<Line> = vec![
        Line::from("Semestre 1").green(),
        Line::from("Semestre 2").red(),
        Line::from("Semestre 3").yellow(),
    ];
    let semesters = Paragraph::new(s_lines).block(semester_block);

    frame.render_widget(semesters, semester_area);
    frame.render_widget(info_block, info_area);
}
