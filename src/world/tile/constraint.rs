/// A constraint that limits when a tile should be chosen for
/// the generated world.
#[derive(Clone, Copy)]
pub enum Constraint {
    /// This constraint is satisfied when the noise value is 
    /// lower than the given threshold.
    LT(f64),

    /// This constraint is satisfied when the noise value is 
    /// greater than the given threshold.
    GT(f64)
}

impl Constraint {
    /// Returns true is the given value would satisfy this constraint.
    pub fn satisfied_by(&self, value: &f64) -> bool {
        match self {
            &Constraint::LT(ref threshold) => value < threshold,
            &Constraint::GT(ref threshold) => value > threshold
        }
    }
}
