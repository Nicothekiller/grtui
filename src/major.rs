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

/// [`Major`] struct, represents a group of [`Semester`]
///
/// The struct also handles the state of the program and renders the TUI, but multiple instances
/// can be created as long as only one of them calls [render](Major::render).
#[derive(Debug, Clone)]
pub struct Major {
    semesters: Vec<Semester>,
    selected: usize,
}

impl Major {
    /// Creates a new [`Major`].
    pub fn new(semesters: Vec<Semester>) -> Self {
        Self {
            semesters,
            selected: 0,
        }
    }

    /// Function to render the TUI because the struct also handles the state of the TUI.
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

                let info = Paragraph::new(format!(
                    "{:#?}\n\nSemeter GPA: {}",
                    self.semesters[self.selected].classes(),
                    self.semesters[self.selected].calc_gpa()
                ))
                .block(info_block);

                let gpa_general = self.general_gpa();

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

    fn general_gpa(&mut self) -> f64 {
        let mut gpa_general = 0.0;
        let mut total_hours = 0.0;

        for semester in self.semesters() {
            gpa_general += semester.total_grade_nd();
            total_hours += semester.total_hours();
        }

        gpa_general /= total_hours;
        gpa_general
    }

    /// Returns a reference to the semesters of this [`Major`].
    pub fn semesters(&self) -> &[Semester] {
        &self.semesters
    }
}
