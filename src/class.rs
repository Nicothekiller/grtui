use serde::{Deserialize, Serialize};

/// [`Class`] struct, represents an individual class in a semester.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    name: String,
    grade: f64,
    hours: isize,
}

impl Class {
    /// Creates a new [`Class`].
    ///
    /// Represents a class in a semester.
    pub fn new(name: String, grade: f64, hours: isize) -> Self {
        Self { name, grade, hours }
    }

    /// Returns a reference to the name of this [`Class`].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the grade of this [`Class`].
    pub fn grade(&self) -> f64 {
        self.grade
    }

    /// Returns the hours of this [`Class`].
    pub fn hours(&self) -> isize {
        self.hours
    }
}
