use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::exceptions::RtsException;

pub trait PlayGroundObserver<T>
where
    T: Display + HasIdentifier,
{
    fn update(&mut self, unit: T);

    fn update_cell(
        &self,
        identifier: Identifier,
        coordinate: Coordinate,
    ) -> Result<(), RtsException>;
}

pub type Cell<T> = Rc<RefCell<UnitHolder<T>>>;
pub type Coordinate = (f32, f32);

pub type Identifier = i128;

pub trait HasIdentifier {
    fn get_identifier(&self) -> Identifier;
    fn is(&self, identifier: &Identifier) -> bool;
}

/// Hold the state of the game
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

impl<T> PlayGroundObserver<T> for PlayGround<T>
where
    T: Display + HasIdentifier,
{
    fn update(&mut self, unit: T) {
        self.cells
            .push(Rc::new(RefCell::new(UnitHolder::new(unit, (0.0, 0.0)))));
    }

    fn update_cell(
        &self,
        identifier: Identifier,
        coordinate: Coordinate,
    ) -> Result<(), RtsException> {
        let cell = self.find_cell_by(&identifier);
        if cell.is_none() {
            return Err(RtsException::UpdatePlayGroundException(format!(
                "Failed to find cell with identifier {}",
                identifier
            )));
        }

        let cell_ptr = Rc::clone(cell.unwrap());
        let mut cell = cell_ptr.borrow_mut();
        cell.update(coordinate);
        Ok(())
    }
}

impl<T> UnitHolder<T>
where
    T: Display + HasIdentifier,
{
    fn new(t: T, coordinate: Coordinate) -> Self {
        UnitHolder {
            t: Some(t),
            coordinate,
        }
    }

    fn is(&self, identifier: &Identifier) -> bool {
        if let Some(t) = &self.t {
            t.is(identifier)
        } else {
            false
        }
    }
}

impl<T> UnitHolder<T>
where
    T: Display + HasIdentifier,
{
    pub fn update(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }
}

impl<T> Default for PlayGround<T>
where
    T: Display + HasIdentifier,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PlayGround<T>
where
    T: Display + HasIdentifier,
{
    /// Initialize map with given capacities to avoid resizing
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn add_unit(&mut self, content: T) {
        let holder = UnitHolder::new(content, (0.0, 0.0));
        self.cells.push(Rc::new(RefCell::new(holder)))
    }

    pub fn get_cells(&self) -> &[Cell<T>] {
        &self.cells
    }

    fn find_cell_by(&self, identifier: &Identifier) -> Option<&Cell<T>> {
        self.cells.iter().find(|cell| {
            let cell_ptr = Rc::clone(cell);
            let cell = cell_ptr.borrow();
            cell.is(identifier)
        })
    }
}

impl<T> Display for PlayGround<T>
where
    T: Display + HasIdentifier,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            let cell_ptr = Rc::clone(cell);
            let cell = cell_ptr.borrow();
            write!(f, "| {} |", *cell)?;
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
