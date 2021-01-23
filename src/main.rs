use serde_json::{Error, Value};

// TODO: Move to ENV variables
const RIOT_API_KEY: &'static str = "RGAPI-8296bf16-098e-44d8-a901-c42e9664b952";
const BASE_URL: &'static str = "https://la2.api.riotgames.com/lol/";

// Move parser to diff crate

// Using default values to prevent overuse of Option | Result
const DEFAULT_STRING_VALUE: &str = "";
const DEFAULT_INTEGER_VALUE: u64 = 0;

struct JsonParser {
    json_data: Value,
}

impl JsonParser {
    fn new(json_data: Value) -> Self {
        JsonParser { json_data }
    }

    fn safe_read_str(&self, key: &str) -> String {
        let value: Option<&Value> = self.json_data.get(key);

        match value {
            Some(value) => {
                let inner_value: Option<&str> = value.as_str();
                match inner_value {
                    Some(value) => value.to_owned(),
                    None => DEFAULT_STRING_VALUE.to_owned(),
                }
            }
            None => DEFAULT_STRING_VALUE.to_owned(),
        }
    }

    fn safe_read_int(&self, key: &str) -> u64 {
        let value: Option<&Value> = self.json_data.get(key);

        match value {
            Some(value) => {
                let inner_value: Option<u64> = value.as_u64();
                match inner_value {
                    Some(value) => value,
                    None => DEFAULT_INTEGER_VALUE,
                }
            }
            None => DEFAULT_INTEGER_VALUE,
        }
    }

    fn safe_read_array(&self, key: &str) -> Option<&Vec<Value>> {
        let value: Option<&Value> = self.json_data.get(key);

        match value {
            Some(value) => {
                let inner_value: Option<&Vec<Value>> = value.as_array();
                match inner_value {
                    Some(value) => Some(value),
                    None => None,
                }
            }
            None => None,
        }
    }
}

trait FromJson<T> {
    fn from_parser(json: JsonParser) -> T {
        unimplemented!("Method not implemented")
    }
    fn from_json_string(json: &str) -> Result<T, Error> {
        unimplemented!("Method not implemented")
    }
    fn from_json_object(json: Value) -> Result<T, Error> {
        unimplemented!("Method not implemented")
    }
}

// TODO: Move profile icon id to u8, summoner levl to u16
// Add new methods to json parser to read those values (cast from u64)
struct LeagueSummoner {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: u64,
    revision_date: u64,
    summoner_level: u64,
}

impl FromJson<LeagueSummoner> for LeagueSummoner {
    fn from_parser(json_parser: JsonParser) -> LeagueSummoner {
        LeagueSummoner {
            id: json_parser.safe_read_str("id"),
            account_id: json_parser.safe_read_str("accountId"),
            puuid: json_parser.safe_read_str("puuid"),
            name: json_parser.safe_read_str("name"),
            profile_icon_id: json_parser.safe_read_int("profileIconId"),
            revision_date: json_parser.safe_read_int("revisionDate"),
            summoner_level: json_parser.safe_read_int("summonerLevel"),
        }
    }

    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        Ok(LeagueSummoner::from_parser(json_parser))
    }
}

struct LeagueMatch {
    platform_id: String,
    game_id: u64,
    champion: u64,
    queue: u64,
    season: u64,
    timestamp: u64,
    role: String,
    lane: String,
}

impl FromJson<LeagueMatch> for LeagueMatch {
    fn from_parser(json_parser: JsonParser) -> LeagueMatch {
        LeagueMatch {
            platform_id: json_parser.safe_read_str("platformId"),
            game_id: json_parser.safe_read_int("gameId"),
            champion: json_parser.safe_read_int("champion"),
            queue: json_parser.safe_read_int("queue"),
            season: json_parser.safe_read_int("season"),
            timestamp: json_parser.safe_read_int("timestamp"),
            role: json_parser.safe_read_str("role"),
            lane: json_parser.safe_read_str("lane"),
        }
    }

    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);
        Ok(LeagueMatch::from_parser(json_parser))
    }

    fn from_json_object(json: Value) -> Result<Self, Error> {
        let json_parser: JsonParser = JsonParser::new(json);
        Ok(LeagueMatch::from_parser(json_parser))
    }
}

struct LeagueMatches {
    matches: Vec<LeagueMatch>,
}

impl FromJson<LeagueMatches> for LeagueMatches {
    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        let mut matches: Vec<LeagueMatch> = Vec::new();

        let json_matches: &Vec<Value> = json_parser.safe_read_array("matches").unwrap();

        for mut _match in json_matches.to_owned() {
            matches.push(LeagueMatch::from_json_object(_match.take()).unwrap());
        }

        Ok(LeagueMatches { matches })
    }
}

struct LeagueService {}

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

    async fn fetch_summoner<'s>(summoner_name: &'s str) -> LeagueSummoner {
        let summoner_url: String = LeagueService::get_summoner_url(summoner_name);

        let json_string: String = LeagueService::fetch(summoner_url.as_str()).await;
        LeagueSummoner::from_json_string(&*json_string).unwrap()
    }

    async fn fetch_summoner_matches<'s>(summoner_name: &'s str) -> LeagueMatches {
        let league_summoner: LeagueSummoner = LeagueService::fetch_summoner(summoner_name).await;

        let matches_url = LeagueService::get_matches_url(league_summoner.account_id.as_str());

        let json_string = LeagueService::fetch(matches_url.as_str()).await;

        LeagueMatches::from_json_string(&*json_string).unwrap()
    }

    fn get_summoner_url<'s>(summoner_name: &'s str) -> String {
        let url_parts: [&'s str; 3] = [BASE_URL, "summoner/v4/summoners/by-name/", summoner_name];
        url_parts.concat()
    }

    fn get_matches_url<'s>(account_id: &'s str) -> String {
        let url_parts = [BASE_URL, "match/v4/matchlists/by-account/", account_id];
        url_parts.concat()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_matches: LeagueMatches = LeagueService::fetch_summoner_matches("Ricefields").await;
    // let league_summoner = LeagueService::fetch_summoner("Ricefields").await;

    for _match in league_matches.matches {
        println!("Role {} \n", _match.role);
    }


    Result::Ok(())
}
