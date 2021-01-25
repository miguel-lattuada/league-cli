use crate::models::FromJson;
use crate::utils::JsonParser;
use serde_json::{Error, Value};

pub struct LeagueMatchParticipant {
    participant_id: String,
    champion_id: u64,
    team_id: u64,
    spell_one_id: u64,
    spell_two_id: u64,
    highest_rank: String,
}

impl FromJson<LeagueMatchParticipant> for LeagueMatchParticipant {
    fn from_parser(json_parser: JsonParser) -> LeagueMatchParticipant {
        LeagueMatchParticipant {
            participant_id: json_parser.safe_read_str("participantId"),
            champion_id: json_parser.safe_read_int("championId"),
            team_id: json_parser.safe_read_int("teamId"),
            spell_one_id: json_parser.safe_read_int("spell1Id"),
            spell_two_id: json_parser.safe_read_int("spell2Id"),
            highest_rank: json_parser.safe_read_str("highestAchievedSeasonTier"),
        }
    }

    fn from_json_string(json: &str) -> Result<Self, Error> {
        unimplemented!()
    }

    fn from_json_object(json: Value) -> Result<Self, Error> {
        let json_parser: JsonParser = JsonParser::new(json);
        Ok(LeagueMatchParticipant::from_parser(json_parser))
    }
}
