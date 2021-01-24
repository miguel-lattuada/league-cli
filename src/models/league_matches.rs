use crate::models::FromJson;
use crate::models::LeagueMatch;
use crate::utils::JsonParser;
use serde_json::{Error, Value};

pub struct LeagueMatches {
    pub matches: Vec<LeagueMatch>,
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
