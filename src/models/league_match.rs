use crate::models::from_json::FromJson;
use crate::utils::JsonParser;
use serde_json::{Value, Error};

/**
 * LeagueMatch is an item of the LeagueMatches
 */
pub struct LeagueMatch {
    pub platform_id: String,
    pub game_id: u64,
    pub champion: u64,
    pub queue: u64,
    pub season: u64,
    pub timestamp: u64,
    pub role: String,
    pub lane: String,
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
