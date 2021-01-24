use crate::models::from_json::FromJson;
use crate::utils::JsonParser;
use serde_json::Error;

// TODO: Move profile icon id to u8, summoner levl to u16
// Add new methods to json parser to read those values (cast from u64)
pub struct LeagueSummoner {
    id: String,
    pub account_id: String,
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
