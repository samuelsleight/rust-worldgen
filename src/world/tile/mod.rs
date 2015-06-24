pub use self::constraint::Constraint;

mod constraint;

/// Objects to generate in the world based on given constraints
#[derive(Clone)]
pub struct Tile<T> {
    value: T,
    constraints: Vec<Constraint>
}

impl<T: Clone> Tile<T> {
    /// Construct a new tile represented by the given object.
    pub fn new(value: T) -> Tile<T> {
        Tile {
            value: value,
            constraints: Vec::new()
        }
    }

    /// Adds a constraint to the tile.
    pub fn when(self, constraint: Constraint) -> Tile<T> {
        Tile {
            constraints: { let mut cs = self.constraints.clone(); cs.push(constraint); cs },
            ..self
        }
    }

    /// Returns the value this tile is represented by.
    pub fn value(&self) -> T {
        self.value.clone()
    }

    /// Returns true if the given value would satisfy all of this tile's 
    /// constraints.
    pub fn satisfied_by(&self, value: &f64) -> bool {
        self.constraints.iter().all(|constraint| constraint.satisfied_by(value))
    }
}
