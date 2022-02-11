use std::fmt::Display;

use crate::exceptions::RtsException;

use super::play_ground::PlayGround;

pub trait Displayer<T>
where
    T: Display,
{
    fn display(play_ground: &PlayGround<T>) -> Result<(), RtsException>;
}

pub struct ConsoleDisplayer;

impl<T> Displayer<T> for ConsoleDisplayer
where
    T: Display,
{
    fn display(play_ground: &PlayGround<T>) -> Result<(), RtsException> {
        println!("{}", play_ground); 
        Ok(())
    }
}
