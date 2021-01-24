use crate::models::LeagueSummoner;
use crate::models::LeagueMatches;
use crate::models::FromJson;

// TODO: Move to ENV variables
const RIOT_API_KEY: &'static str = "RGAPI-8296bf16-098e-44d8-a901-c42e9664b952";
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

    pub fn get_summoner_url<'s>(summoner_name: &'s str) -> String {
        let url_parts: [&'s str; 3] = [BASE_URL, "summoner/v4/summoners/by-name/", summoner_name];
        url_parts.concat()
    }

    pub fn get_matches_url<'s>(account_id: &'s str) -> String {
        let url_parts = [BASE_URL, "match/v4/matchlists/by-account/", account_id];
        url_parts.concat()
    }
}