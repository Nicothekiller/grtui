use std::io::Stdout;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::Major;

/// Struct that represents the TUI of the program. Handles the info to render based on the major.
pub struct Tui {
    major: Major,
    selected: usize,
}

impl Tui {
    /// Creates a new [`Tui`].
    pub fn new(major: Major) -> Self {
        Self { major, selected: 0 }
    }

    /// Function to render the TUI.
    ///
    /// # Panics
    ///
    /// Panics if [draw](ratatui::Terminal::draw) fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if [read](crossterm::event::read) fails.
    pub fn render(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
    ) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| {
                let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
                let [main_area, status_area] = vertical.areas(frame.area());

                let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]);
                let [semester_area, info_area] = horizontal.areas(main_area);

                let semester_block = Block::bordered().title(Span::from(" Semestres ").green());
                let info_block = Block::bordered().title(" Info ");

                let mut s_lines_str: Vec<String> = vec![];
                for i in 1..self.major.semesters().len() + 1 {
                    s_lines_str.push(format!("Semestre {}", i));
                }

                let s_lines: Vec<Line> = s_lines_str
                    .iter()
                    .map(|x| {
                        if *x == s_lines_str[self.selected] {
                            return Line::from(x.as_str()).green().on_light_blue();
                        }

                        Line::from(x.as_str()).green()
                    })
                    .collect();

                let semesters = Paragraph::new(s_lines).block(semester_block);

                let info = Paragraph::new(format!(
                    "{:#?}\n\nSemeter GPA: {}",
                    self.major.semesters()[self.selected].classes(),
                    self.major.semesters()[self.selected].calc_gpa()
                ))
                .block(info_block);

                let gpa_general = self.major.general_gpa();

                let status_bar = Line::from(format!("general GPA: {}", gpa_general));

                frame.render_widget(semesters, semester_area);
                frame.render_widget(info, info_area);
                frame.render_widget(status_bar, status_area);
            })?;

            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.selected + 1 < self.major.semesters().len() {
                            self.selected += 1
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.selected > 0 {
                            self.selected -= 1
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }

        Ok(())
    }
}
