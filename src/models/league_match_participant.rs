use crate::models::FromJson;
use crate::utils::JsonParser;
use serde_json::{Error, Value};

#[derive(Clone)]
pub struct ParticipantStats {
    pub deaths: u64,
    pub kills: u64,
    pub assists: u64,
}

#[derive(Clone)]
pub struct LeagueMatchParticipant {
    pub participant_id: u64,
    account_id: String,
    pub champion_id: u64,
    pub team_id: u64,
    spell_one_id: u64,
    spell_two_id: u64,
    highest_rank: String,
    pub stats: ParticipantStats,
}

impl LeagueMatchParticipant {
    // TODO: move this into a trait that every model should implement
    // Consider CliDisplay { into_formatted_string() -> String }
    pub fn into_string(self) -> String {
        let mut result = String::from("");

        result.push_str("Champion: ");
        result.push_str(&self.champion_id.to_string());
        result.push_str(" -> ");
        result.push_str("(");
        result.push_str(&self.team_id.to_string());
        result.push_str(")");

        result
    }
}

impl FromJson<LeagueMatchParticipant> for LeagueMatchParticipant {
    fn from_parser(json_parser: JsonParser) -> LeagueMatchParticipant {
        let stats: &Value = json_parser.json_data.get("stats").unwrap();

        LeagueMatchParticipant {
            stats: ParticipantStats {
                kills: stats.get("kills").unwrap().as_u64().unwrap(),
                assists: stats.get("assists").unwrap().as_u64().unwrap(),
                deaths: stats.get("deaths").unwrap().as_u64().unwrap(),
            },
            participant_id: json_parser.safe_read_int("participantId"),
            account_id: json_parser.safe_read_str("accountid"),
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
