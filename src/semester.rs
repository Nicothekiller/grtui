use crate::Class;

#[derive(Debug, Clone)]
pub struct Semester {
    classes: Vec<Class>,
}

impl Semester {
    pub fn new(classes: Vec<Class>) -> Self {
        Self { classes }
    }
}
