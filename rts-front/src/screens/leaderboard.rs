use std::fmt::Display;

use regex::Regex;
use reqwasm::http::{Request, Response};
use serde::Deserialize;
use weblog::console_log;
use yew::prelude::*;

use crate::utils::alert_message;

const LEADER_BOARD_ROUTE: &str = "/api/leaderboard/{max}";

#[derive(Clone, PartialEq, Deserialize)]
struct LeaderBoardRow {
    username: String,
    wins: i32,
    looses: i32,
    score: i32,
}

impl Display for LeaderBoardRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {}. | {} | {} | w: {} - l: {}",
            1, self.username, self.score, self.wins, self.looses,
        )
    }
}

#[derive(Properties, PartialEq)]
struct LeaderBoardListProps {
    matchs: Vec<LeaderBoardRow>,
}

#[function_component(LeaderBoardList)]
fn leader_board(LeaderBoardListProps { matchs }: &LeaderBoardListProps) -> Html {
    matchs
        .iter()
        .map(|row| {
            html! {
                <li><p>{format!("{}", row)}</p></li>
            }
        })
        .collect()
}

#[function_component(Leaderboard)]
pub fn leaderboard() -> Html {
    let leader_board: UseStateHandle<Vec<LeaderBoardRow>> = use_state(|| Vec::new());

    {
        let leader_board = leader_board.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_board = featch_leader_board().await;
                    leader_board.set(fetched_board);
                });

                || ()
            },
            (),
        );
    }

    html! {
        <ul>
            <LeaderBoardList matchs={ (*leader_board).clone() } />
        </ul>
    }
}

async fn featch_leader_board() -> Vec<LeaderBoardRow> {
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

async fn parse_leader_board(response: Response) -> Vec<LeaderBoardRow> {
    match response.json::<Vec<LeaderBoardRow>>().await {
        Ok(rows) => rows,
        Err(_) => {
            alert_message("Failed to parse leader board json response !");
            Vec::new()
        }
    }
}
