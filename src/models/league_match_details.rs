use crate::models::{FromJson, LeagueMatch, LeagueMatchParticipant};
use crate::utils::{JsonParser, format_game_duration, Queues};
use serde_json::{Error, Value};

pub struct LeagueMatchDetail {
    // TODO check on league match data and how to set it
    // league_match: LeagueMatch,
    pub participants: Vec<LeagueMatchParticipant>,
    pub game_duration: u64,
    pub queue_id: u64
}

impl LeagueMatchDetail {
    // TODO: move this into a trait that every model should implementservice
    // Consider CliDisplay { into_formatted_string() -> String }
    pub fn into_string(self) -> String {
        let mut result = String::from("");

        result.push_str(&Queues.get(&self.queue_id).unwrap());
        result.push_str("\n");
        result.push_str(&format!("> Duration: {}", format_game_duration(self.game_duration)));
        
        // for participant in self.participants {
        //     result.push_str(&participant.into_string());
        //     result.push_str("\n");
        // }

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

        let game_duration: u64 = json_parser.safe_read_int("gameDuration");
        let queue_id: u64 = json_parser.safe_read_int("queueId");

        Ok(LeagueMatchDetail { participants, game_duration, queue_id })
    }
}