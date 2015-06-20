#[derive(Clone, Copy)]
pub enum Constraint {
    LT(f64),
    GT(f64)
}

impl Constraint {
    pub fn satisfied_by(&self, value: &f64) -> bool {
        match self {
            &Constraint::LT(ref threshold) => value < threshold,
            &Constraint::GT(ref threshold) => value > threshold
        }
    }
}
