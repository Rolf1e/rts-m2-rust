use crate::components::building::Building;
use crate::entity::player::Player;

pub type Players = (Player, Player);

pub struct PlayGround {
    barrack: Building,
    players: Players,
}

impl PlayGround {
    pub fn new(player_1: Player, player_2: Player) -> Self {
        PlayGround {
            barrack: Building::new(),
            players: (player_1, player_2),
        }
    }
}
