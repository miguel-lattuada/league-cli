use crate::models::{LeagueSummoner, LeagueMatches, FromJson, LeagueMatchesDetails, LeagueMatchDetail};
use futures_util::future::join_all;

// TODO: Move to ENV variables
const RIOT_API_KEY: &'static str = "RGAPI-4cb655c6-4251-4afa-ad5d-3afbad854ef3";
const BASE_URL: &'static str = "https://la2.api.riotgames.com/lol/";

pub struct LeagueService {}

impl LeagueService {
    async fn fetch(url: &str) -> String {
        let client = reqwest::Client::new();

        // TODO: handle errors on fetch
        client
            .get(url)
            .header("X-Riot-Token", RIOT_API_KEY)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn fetch_summoner<'s>(summoner_name: &'s str) -> LeagueSummoner {
        let summoner_url: String = LeagueService::get_summoner_url(summoner_name);

        let json_string: String = LeagueService::fetch(summoner_url.as_str()).await;
        LeagueSummoner::from_json_string(&*json_string).unwrap()
    }

    pub async fn fetch_summoner_matches<'s>(summoner_name: &'s str) -> LeagueMatches {
        let league_summoner: LeagueSummoner = LeagueService::fetch_summoner(summoner_name).await;

        let matches_url = LeagueService::get_matches_url(league_summoner.account_id.as_str());

        let json_string = LeagueService::fetch(matches_url.as_str()).await;

        LeagueMatches::from_json_string(&*json_string).unwrap()
    }

    pub async fn fetch_summoner_matches_details<'s>(summoner_name: &'s str) -> LeagueMatchesDetails {
        let league_summoner: LeagueSummoner = LeagueService::fetch_summoner(summoner_name).await;

        let matches_url = LeagueService::get_matches_url(league_summoner.account_id.as_str());

        let json_string = LeagueService::fetch(matches_url.as_str()).await;

        let league_matches = LeagueMatches::from_json_string(&*json_string).unwrap();

        let mut league_matches_details = LeagueMatchesDetails::new();

        let mut summoner_match_requests: Vec<_> = Vec::with_capacity(league_matches.matches.len());

        // TODO for each league_matches we have to request match detail and create response
        // append each detail to league_matches_details
        // for _match in league_matches.matches {
        //     summoner_match_requests.push(LeagueService::fetch_summoner_match_detail(_match.game_id));
        // }

        for index in 0..2 {
            let _match = league_matches.matches.get(index).unwrap();
            summoner_match_requests.push(LeagueService::fetch_summoner_match_detail(_match.game_id));
        }

        league_matches_details.matches_details = join_all(summoner_match_requests).await;

        league_matches_details
    }

    async fn fetch_summoner_match_detail(match_id: u64) -> LeagueMatchDetail {
        let match_detail_url = LeagueService::get_match_details_url(&*match_id.to_string());

        let json_string = LeagueService::fetch(match_detail_url.as_str()).await;

        LeagueMatchDetail::from_json_string(&*json_string).unwrap()
    }

    pub fn get_summoner_url<'s>(summoner_name: &'s str) -> String {
        let url_parts: [&'s str; 3] = [BASE_URL, "summoner/v4/summoners/by-name/", summoner_name];
        url_parts.concat()
    }

    pub fn get_matches_url<'s>(account_id: &'s str) -> String {
        let url_parts = [BASE_URL, "match/v4/matchlists/by-account/", account_id];
        url_parts.concat()
    }

    pub fn get_match_details_url<'s>(match_id: &'s str) -> String {
        let url_parts = [BASE_URL, "match/v4/matches/", match_id];
        url_parts.concat()
    }
}