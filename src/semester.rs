use serde::{Deserialize, Serialize};

use crate::Class;

/// [`Semester`] struct, represents a single semester.
///
/// A semester is a vec filled with [`Class`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Semester {
    classes: Vec<Class>,
}

impl Semester {
    /// Creates a new [`Semester`].
    pub fn new(classes: Vec<Class>) -> Self {
        Self { classes }
    }

    /// Returns a reference to the classes of this [`Semester`].
    pub fn classes(&self) -> &[Class] {
        &self.classes
    }

    /// Returns the GPA of this [`Semester`].
    pub fn calc_gpa(&self) -> f64 {
        let mut hours: f64 = 0.0;
        let mut grade_nd: f64 = 0.0;

        for class in self.classes() {
            let c_hour: f64 = class.hours() as f64;
            hours += c_hour;

            if class.grade() >= 91.0 {
                grade_nd += 4.0 * c_hour;
            } else if class.grade() >= 81.0 && class.grade() < 91.0 {
                grade_nd += 3.0 * c_hour;
            } else if class.grade() >= 71.0 && class.grade() < 81.0 {
                grade_nd += 2.0 * c_hour;
            } else if class.grade() >= 61.0 && class.grade() < 71.0 {
                grade_nd += 1.0 * c_hour;
            } else if class.grade() < 61.0 {
                grade_nd += 0.0;
            }
        }

        grade_nd / hours
    }

    /// Returns the total coursed hours of this [`Semester`].
    pub fn total_hours(&self) -> f64 {
        let mut t_hours = 0.0;
        for class in self.classes() {
            t_hours += class.hours() as f64;
        }

        t_hours
    }

    /// Returns the total grade of this [`Semester`] by turning the grade into number based on the
    /// result (A->4,B->3,C->2,D->1,F->0) and multiplying that number by the amount of hours of the
    /// class to then sum everything.
    ///
    /// This function does not divide the result by the total amount of hours coursed, which is
    /// needed to calculate the final grade.
    pub fn total_grade_nd(&self) -> f64 {
        let mut grade_nd: f64 = 0.0;

        for class in self.classes() {
            let c_hour: f64 = class.hours() as f64;

            if class.grade() >= 91.0 {
                grade_nd += 4.0 * c_hour;
            } else if class.grade() >= 81.0 && class.grade() < 91.0 {
                grade_nd += 3.0 * c_hour;
            } else if class.grade() >= 71.0 && class.grade() < 81.0 {
                grade_nd += 2.0 * c_hour;
            } else if class.grade() >= 61.0 && class.grade() < 71.0 {
                grade_nd += 1.0 * c_hour;
            } else if class.grade() < 61.0 {
                grade_nd += 0.0 * c_hour;
            }
        }

        grade_nd
    }
}
