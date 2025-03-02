#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    grade: f64,
    hours: isize,
}

impl Class {
    pub fn new(name: String, grade: f64, hours: isize) -> Self {
        Self { name, grade, hours }
    }
}
