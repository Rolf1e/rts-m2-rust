use rts_core::components::displayer::{ConsoleDisplayer, Displayer};
use rts_core::components::play_ground::PlayGround;
use rts_core::components::unit_factory::UnitFactory;
use rts_core::entity::unit::{Unit, UnitType};
use rts_core::exceptions::RtsException;

fn main() -> Result<(), RtsException> {
    let mut play_ground: PlayGround<Unit> = PlayGround::new(10, 10);
    let unit_factory = UnitFactory::default();
    ConsoleDisplayer::display(&play_ground)?;

    let unit = unit_factory.build_unit(UnitType::Classic);
    play_ground.update_at((1, 1), unit)?;
    ConsoleDisplayer::display(&play_ground)?;

    Ok(())
}
