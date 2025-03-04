use crate::Semester;

use serde::{Deserialize, Serialize};

/// [`Major`] struct, represents a group of [`Semester`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Major {
    semesters: Vec<Semester>,
}

impl Major {
    /// Creates a new [`Major`].
    pub fn new(semesters: Vec<Semester>) -> Self {
        Self { semesters }
    }

    /// Returns the general gpa of this [`Major`].
    pub fn general_gpa(&mut self) -> f64 {
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
