use std::fmt::Display;

use crate::exceptions::RtsException;

use super::play_ground::{HasIdentifier, PlayGround};

pub trait Displayer<T>
where
    T: Display + HasIdentifier,
{
    fn display(play_ground: &PlayGround<T>) -> Result<(), RtsException>;
}

pub struct ConsoleDisplayer;

impl<T> Displayer<T> for ConsoleDisplayer
where
    T: Display + HasIdentifier,
{
    fn display(play_ground: &PlayGround<T>) -> Result<(), RtsException> {
        println!("=== PlayGround === \n{}\n ======", play_ground); 
        Ok(())
    }
}
