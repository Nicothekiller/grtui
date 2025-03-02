use std::io::Stdout;

use crate::Semester;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Terminal,
};

#[derive(Debug, Clone)]
pub struct Major {
    semesters: Vec<Semester>,
    selected: usize,
}

impl Major {
    pub fn new(semesters: Vec<Semester>) -> Self {
        Self {
            semesters,
            selected: 0,
        }
    }

    /// Draw fn to use with ratatui. Draws the events happening on the screen.
    pub fn render(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
    ) -> std::io::Result<()> {
        loop {
            terminal
                .draw(|frame| {
                    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]);
                    let [semester_area, info_area] = horizontal.areas(frame.area());

                    let semester_block = Block::bordered().title(Span::from(" Semestres ").green());
                    let info_block = Block::bordered().title(" Info ");

                    let mut s_lines_str: Vec<String> = vec![];
                    for i in 1..self.semesters().len() + 1 {
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

                    let info = Paragraph::new(format!("{:#?}", self.semesters[self.selected]))
                        .block(info_block);

                    frame.render_widget(semesters, semester_area);
                    frame.render_widget(info, info_area);
                })
                .expect("failed to draw frame");

            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.selected + 1 < self.semesters().len() {
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

    pub fn semesters(&self) -> &[Semester] {
        &self.semesters
    }
}
