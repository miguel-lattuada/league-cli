use crate::models::{
    FromJson, LeagueMatch, LeagueMatchParticipant, LeagueMatchParticipantIdentity, Player,
};
use crate::utils::{format_game_duration, Champions, JsonParser, Queues};
use serde_json::{Error, Value};

pub struct LeagueMatchDetail {
    // TODO check on league match data and how to set it
    // league_match: LeagueMatch,
    pub participants: Vec<LeagueMatchParticipant>,
    pub participant_identities: Vec<LeagueMatchParticipantIdentity>,
    pub game_duration: u64,
    pub queue_id: u64,
}

impl LeagueMatchDetail {
    // TODO: move this into a trait that every model should implementservice
    // Consider CliDisplay { into_formatted_string() -> String }
    pub fn into_string(self) -> String {
        let mut result = String::from("");

        result.push_str(&Queues.get(&self.queue_id).unwrap());
        result.push_str("\n");
        result.push_str(&format!(
            "> Duration: {}",
            format_game_duration(self.game_duration)
        ));
        result.push_str("\n");
        result.push_str(&format!(
            "> Champion: {}",
            Champions
                .get(&self.get_participant_info("Ricefields").champion_id)
                .unwrap()
        ));
        // for participant in self.participants {
        //     result.push_str(&participant.into_string());
        //     result.push_str("\n");
        // }

        result
    }

    pub fn get_participant_id(&self, summoner_name: &str) -> LeagueMatchParticipantIdentity {
        let participant_identity: &LeagueMatchParticipantIdentity = &self
            .participant_identities
            .to_owned()
            .into_iter()
            .find(|identity| identity.player.summoner_name == summoner_name)
            .unwrap();

        participant_identity.clone()
    }

    pub fn get_participant_info(&self, summoner_name: &str) -> LeagueMatchParticipant {
        let participant_id = &self.get_participant_id(summoner_name);

        let participant_info: &LeagueMatchParticipant = &self
            .participants
            .to_owned()
            .into_iter()
            .find(|participant| participant.participant_id == participant_id.participant_id)
            .unwrap();

        participant_info.clone()
    }
}

impl FromJson<LeagueMatchDetail> for LeagueMatchDetail {
    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        let participants: Vec<LeagueMatchParticipant> = json_parser
            .safe_read_array("participants")
            .into_iter()
            .map(|mut participant| {
                LeagueMatchParticipant::from_json_object(participant.take()).unwrap()
            })
            .collect();

        let participant_identities: Vec<LeagueMatchParticipantIdentity> = json_parser
            .safe_read_array("participantIdentities")
            .into_iter()
            .map(|mut identity| {
                LeagueMatchParticipantIdentity::from_json_object(identity.take()).unwrap()
            })
            .collect();

        let game_duration: u64 = json_parser.safe_read_int("gameDuration");
        let queue_id: u64 = json_parser.safe_read_int("queueId");

        Ok(LeagueMatchDetail {
            participants,
            participant_identities,
            game_duration,
            queue_id,
        })
    }
}
