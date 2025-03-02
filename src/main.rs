use gtui::{Class, Major, Semester};

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

    let semesters = vec![
        Semester::new(vec![
            Class::new("Calculo vectorial".to_string(), 92.0, 3),
            Class::new("Matematicas discretas".to_string(), 98.0, 3),
        ]),
        Semester::new(vec![
            Class::new("Calculo integral".to_string(), 87.2, 4),
            Class::new("quimica".to_string(), 59.0, 1),
        ]),
    ];

    let mut major = Major::new(semesters);

    major.render(terminal)?;

    ratatui::restore();
    Ok(())
}
