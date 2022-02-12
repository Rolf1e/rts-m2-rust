use rts_core::components::game::Game;
use rts_core::entity::game_actions::Action;
use rts_core::entity::player::Player;

async fn start(game: &Game) {
    println!("Launch game");
    match game.start().await {
        Ok(_) => println!("GS"),
        Err(e) => println!("{}", e),
    }
}

async fn play(game: &Game) {
    println!("Play a turn ");
    match game.play_with_async(0, Action::EndGame).await {
        Ok(_) => println!("Play"),
        Err(e) => println!("{}", e),
    }
}

#[tokio::main]
async fn main() {
    let p1 = Player::new("Tigran".to_string());
    let p2 = Player::new("Emma".to_string());
    let game = Game::new(vec![p1, p2]);

    let t1 = start(&game);
    let t2 = play(&game);

    tokio::select! {
        _ = t2 => println!("Game is over"),
        _ = t1 => println!("Game started"),
    };
}
