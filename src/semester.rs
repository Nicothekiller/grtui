use crate::Class;

/// [`Semester`] struct, represents a single semester.
///
/// A semester is a vec filled with [`Class`].
#[derive(Debug, Clone)]
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
}
