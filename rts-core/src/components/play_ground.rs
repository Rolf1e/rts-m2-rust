use std::fmt::Display;
use std::sync::{Arc, Mutex};

pub type Cell<T> = Arc<Mutex<UnitHolder<T>>>;
pub type Coordinate = (f32, f32);

pub type Identifier = i16;

pub trait HasIdentifier {
    fn get_identifier(&self) -> Identifier;
    fn is(&self, identifier: &Identifier) -> bool;
}

pub struct PlayGround<T>
where
    T: Display + HasIdentifier,
{
    cells: Vec<Cell<T>>,
}

pub struct UnitHolder<T>
where
    T: Display + HasIdentifier,
{
    t: Option<T>,
    coordinate: Coordinate,
}

impl<T> UnitHolder<T>
where
    T: Display + HasIdentifier,
{
    fn new(coordinate: Coordinate) -> Self {
        UnitHolder {
            t: None,
            coordinate,
        }
    }

    fn is(&self, identifier: &Identifier) -> bool {
        todo!()
    }
}

impl<T> UnitHolder<T>
where
    T: Display + HasIdentifier,
{
    pub fn update_with(&mut self, content: T) {
        self.t = Some(content);
    }
}

impl<T> PlayGround<T>
where
    T: Display + HasIdentifier,
{
    /// Initialize the map with given capacities
    /// All vectors are empty
    pub fn new(number_of_columns: usize, number_of_rows: usize) -> Self {
        todo!()
    }

    pub fn get_cells(&self) -> &[Cell<T>] {
        &self.cells
    }
}

impl<T> Display for PlayGround<T>
where
    T: Display + HasIdentifier,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            write!(f, "|")?;
            let cell = Arc::clone(cell);
            let cell = cell.lock().map_err(|_| std::fmt::Error)?;
            write!(f, " {} |", *cell)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Display for UnitHolder<T>
where
    T: Display + HasIdentifier,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.t {
            Some(t) => write!(f, "{}", t),
            None => write!(f, "xx"),
        }
    }
}
