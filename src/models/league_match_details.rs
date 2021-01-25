use crate::models::{FromJson, LeagueMatch, LeagueMatchParticipant};
use crate::utils::JsonParser;
use serde_json::{Error, Value};

pub struct LeagueMatchDetail {
    // TODO check on league match data and how to set it
    // league_match: LeagueMatch,
    participants: Vec<LeagueMatchParticipant>,
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
