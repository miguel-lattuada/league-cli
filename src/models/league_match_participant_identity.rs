use crate::models::FromJson;
use crate::utils::JsonParser;
use serde_json::{Error, Value};

#[derive(Clone)]
pub struct Player {
    pub account_id: String,
    pub summoner_name: String,
}

#[derive(Clone)]
pub struct LeagueMatchParticipantIdentity {
    pub participant_id: u64,
    pub player: Player,
}

impl FromJson<LeagueMatchParticipantIdentity> for LeagueMatchParticipantIdentity {
    fn from_parser(json_parser: JsonParser) -> LeagueMatchParticipantIdentity {
        let player: &Value = json_parser.json_data.get("player").unwrap();

        LeagueMatchParticipantIdentity {
            participant_id: json_parser.safe_read_int("participantId"),
            player: Player {
                account_id: player
                    .get("accountId")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned(),
                summoner_name: player
                    .get("summonerName")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned(),
            },
        }
    }

    fn from_json_string(json: &str) -> Result<Self, Error> {
        unimplemented!()
    }

    fn from_json_object(json: Value) -> Result<Self, Error> {
        let json_parser: JsonParser = JsonParser::new(json);
        Ok(LeagueMatchParticipantIdentity::from_parser(json_parser))
    }
}
