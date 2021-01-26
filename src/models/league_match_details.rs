use crate::models::{FromJson, LeagueMatch, LeagueMatchParticipant};
use crate::utils::JsonParser;
use serde_json::{Error, Value};

pub struct LeagueMatchDetail {
    // TODO check on league match data and how to set it
    // league_match: LeagueMatch,
    pub participants: Vec<LeagueMatchParticipant>,
}

impl LeagueMatchDetail {
    // TODO: move this into a trait that every model should implement
    // Consider CliDisplay { into_formatted_string() -> String }
    pub fn into_string(self) -> String {
        let mut result = String::from("");
        for participant in self.participants {
            result.push_str(&participant.into_string());
            result.push_str("\n");
        }
        result
    }
}

impl FromJson<LeagueMatchDetail> for LeagueMatchDetail {
    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        let mut participants: Vec<LeagueMatchParticipant> = Vec::new();

        let json_participants: &Vec<Value> = json_parser.safe_read_array("participants").unwrap();

        for mut participant in json_participants.to_owned() {
            participants.push(LeagueMatchParticipant::from_json_object(participant.take()).unwrap());
        }

        Ok(LeagueMatchDetail { participants })
    }
}