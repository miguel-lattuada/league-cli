mod models;
mod utils;
mod services;

use models::{LeagueMatches};
use services::{LeagueService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_matches: LeagueMatches = LeagueService::fetch_summoner_matches("Ricefields").await;
    // let league_summoner = LeagueService::fetch_summoner("Ricefields").await;

    for _match in league_matches.matches {
        println!("Role {} \n", _match.role);
    }


    Result::Ok(())
}
