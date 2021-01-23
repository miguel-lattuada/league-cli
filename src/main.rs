use serde_json::{Error, Value};

// TODO: Move to ENV variables
const RIOT_API_KEY: &'static str = "RGAPI-a9305292-3faf-4da8-b74a-5b63712c69a0";
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

impl LeagueSummoner {
    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        Ok(LeagueSummoner {
            id: json_parser.safe_read_str("id"),
            account_id: json_parser.safe_read_str("accountId"),
            puuid: json_parser.safe_read_str("puuid"),
            name: json_parser.safe_read_str("name"),
            profile_icon_id: json_parser.safe_read_int("profileIconId"),
            revision_date: json_parser.safe_read_int("revisionDate"),
            summoner_level: json_parser.safe_read_int("summonerLevel"),
        })
    }
}

struct LeagueMatch {}

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

    async fn fetch_summoner(summoner_name: &'static str) -> LeagueSummoner {
        let summoner_url: String = LeagueService::get_summoner_url(summoner_name);

        let json_string: String = LeagueService::fetch(summoner_url.as_str()).await;
        LeagueSummoner::from_json_string(&*json_string).unwrap()
    }

    fn get_summoner_url(summoner_name: &'static str) -> String {
        let url_parts: [&'static str; 3] =
            [BASE_URL, "summoner/v4/summoners/by-name/", summoner_name];
        url_parts.concat()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_summoner = LeagueService::fetch_summoner("Ricefields").await;

    println!("{}", league_summoner.account_id);

    Result::Ok(())
}
