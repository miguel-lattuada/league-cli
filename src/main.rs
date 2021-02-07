mod models;
mod utils;
mod services;
mod cli;
mod json;

use services::LeagueService;
use models::LeagueMatchesDetails;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "s", long = "summoner-name")]
    summoner_name: String,

    #[structopt(short = "c", long = "matches-count", default_value = "2")]
    matches_count: usize
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: TEST THE FETCH MATCH DETAIL METHOD FROM LEAGUE SERVICE
    
    // let summoner = LeagueService::fetch_summoner("Ricefields");
    
    // let league_matches: LeagueMatches = LeagueService::fetch_summoner_matches("Ricefields").await;
    // let league_summoner = LeagueService::fetch_summoner("Ricefields").await;

    let opt: Opt = Opt::from_args();
    let league_matches: LeagueMatchesDetails = LeagueService::fetch_summoner_matches_details(&opt.summoner_name, opt.matches_count).await;
    print!("{}", league_matches.into_string());

    Result::Ok(())
}