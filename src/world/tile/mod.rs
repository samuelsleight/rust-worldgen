pub use self::constraint::Constraint;

mod constraint;

#[derive(Clone)]
pub struct Tile<T> {
    value: T,
    constraints: Vec<Constraint>
}

impl<T: Clone> Tile<T> {
    pub fn new(value: T) -> Tile<T> {
        Tile {
            value: value,
            constraints: Vec::new()
        }
    }

    pub fn when(self, constraint: Constraint) -> Tile<T> {
        Tile {
            constraints: { let mut cs = self.constraints.clone(); cs.push(constraint); cs },
            ..self
        }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn satisfied_by(&self, value: &f64) -> bool {
        self.constraints.iter().all(|constraint| constraint.satisfied_by(value))
    }
}
