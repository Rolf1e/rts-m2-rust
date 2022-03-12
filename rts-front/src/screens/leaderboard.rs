use std::fmt::Display;

use regex::Regex;
use reqwasm::http::{Request, Response};
use serde::Deserialize;
use weblog::console_log;
use yew::prelude::*;

use crate::utils::alert_message;

const LEADER_BOARD_ROUTE: &str = "/api/leaderboard/{max}";

#[derive(Clone, PartialEq, Deserialize)]
struct MatchRow {
    rank: i32,
    player: String,
    score: i32,
    wins: i32,
    losses: i32,
}

impl Display for MatchRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. | {} | {} | w: {} - l: {}",
            self.rank, self.player, self.score, self.wins, self.losses
        )
    }
}

#[derive(Properties, PartialEq)]
struct MatchRowListProps {
    matchs: Vec<MatchRow>,
}

#[function_component(LeaderBoard)]
fn leader_board(MatchRowListProps { matchs }: &MatchRowListProps) -> Html {
    matchs
        .iter()
        .map(|m| {
            html! {
                <li><p>{format!("{}", m)}</p></li>
            }
        })
        .collect()
}

#[function_component(Leaderboard)]
pub fn leaderboard() -> Html {
    let leader_board: UseStateHandle<Vec<MatchRow>> = use_state(|| Vec::new());

    {
        let leader_board = leader_board.clone();

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_board = featch_leader_board().await;
                leader_board.set(fetched_board);
            });

            || ()
        });
    }

    html! {
        <ul>
            <LeaderBoard matchs={ (*leader_board).clone() } />
        </ul>
    }
}

async fn featch_leader_board() -> Vec<MatchRow> {
    let possible_regex = Regex::new(r"\{max\}");
    if let Err(_error) = possible_regex {
        alert_message("Failed to compile regex !");
        return Vec::new();
    }

    let route = possible_regex
        .unwrap()
        .replace_all(LEADER_BOARD_ROUTE, "10");

    let response = Request::get(&route)
        .header("Content-Type", "application/json")
        .send();

    if let Ok(leader_board) = response.await {
        parse_leader_board(leader_board).await
    } else {
        alert_message("Failed to retrieve leader board from api !");
        Vec::new()
    }
}

async fn parse_leader_board(response: Response) -> Vec<MatchRow> {
    match response.json::<Vec<MatchRow>>().await {
        Ok(rows) => rows,
        Err(_) => {
            alert_message("Failed to parse leader board json response !");
            Vec::new()
        }
    }
}
