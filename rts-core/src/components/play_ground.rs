use std::fmt::Display;
use std::sync::{Arc, Mutex};

use crate::exceptions::RtsException;

pub type Cell<T> = Arc<Mutex<UnitHolder<T>>>;

pub struct PlayGround<T>
where
    T: Display,
{
    cells: Vec<Vec<Cell<T>>>,
}

pub struct UnitHolder<T>
where
    T: Display,
{
    t: Option<T>,
}

impl<T> Default for UnitHolder<T>
where
    T: Display,
{
    fn default() -> Self {
        UnitHolder { t: None }
    }
}

impl<T> UnitHolder<T>
where
    T: Display,
{
    pub fn update_with(&mut self, content: T) {
        self.t = Some(content);
    }
}

impl<T> PlayGround<T>
where
    T: Display,
{
    /// Initialize the map with given capacities
    /// All vectors are empty
    pub fn new(number_of_columns: usize, number_of_rows: usize) -> Self {
        let mut cells = Vec::with_capacity(number_of_rows);
        for _ in 0..number_of_rows {
            cells.push(vec![
                Arc::new(Mutex::new(UnitHolder::default()));
                number_of_columns
            ]);
        }
        PlayGround { cells }
    }

    pub fn update_at(&mut self, (x, y): (usize, usize), content: T) -> Result<(), RtsException> {
        let cell = &self.cells[x][y];
        let cell = Arc::clone(&cell);
        let mut cell = cell.lock().map_err(|_| {
            RtsException::UpdatePlayGroundException("Failed to update map".to_string())
        })?;
        println!("{}", &cell);
        cell.update_with(content);
        Ok(())
    }

    pub fn get_cells(&self) -> &Vec<Vec<Cell<T>>> {
        &self.cells
    }
}

impl<T> Display for PlayGround<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            write!(f, "|")?;
            for cell in row {
                let cell = Arc::clone(cell);
                let cell = cell.lock().map_err(|_| std::fmt::Error)?;
                write!(f, " {} |", *cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Display for UnitHolder<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.t {
            Some(t) => write!(f, "{}", t),
            None => write!(f, "xx"),
        }
    }
}
