mod models;
mod utils;
mod services;

use models::{LeagueMatches};
use services::{LeagueService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: TEST THE FETCH MATCH DETAIL METHOD FROM LEAGUE SERVICE
    
    // let summoner = LeagueService::fetch_summoner("Ricefields");

    let league_matches: LeagueMatches = LeagueService::fetch_summoner_matches("Ricefields").await;
    // let league_summoner = LeagueService::fetch_summoner("Ricefields").await;

    for _match in league_matches.matches {
        println!("Role {}", _match.game_id);
    }


    Result::Ok(())
}
