mod models;
mod utils;
mod services;

use models::{LeagueMatches, LeagueMatchesDetails, LeagueMatchDetail};
use services::{LeagueService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: TEST THE FETCH MATCH DETAIL METHOD FROM LEAGUE SERVICE
    
    // let summoner = LeagueService::fetch_summoner("Ricefields");

    // let league_matches: LeagueMatches = LeagueService::fetch_summoner_matches("Ricefields").await;
    // let league_summoner = LeagueService::fetch_summoner("Ricefields").await;

    let league_matches: LeagueMatchesDetails = LeagueService::fetch_summoner_matches_details("Ricefields").await;

    print!("{}", league_matches.into_string());

    Result::Ok(())
}
