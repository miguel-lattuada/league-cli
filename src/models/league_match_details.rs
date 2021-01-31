use crate::models::{LeagueMatchParticipant, LeagueMatchParticipantIdentity, LeagueMatchTeam};
use crate::utils::{format_game_duration, CHAMPIONS, QUEUES};

pub struct LeagueMatchDetail {
    // TODO check on league match data and how to set it
    // league_match: LeagueMatch,
    pub participants: Vec<LeagueMatchParticipant>,
    pub participant_identities: Vec<LeagueMatchParticipantIdentity>,
    pub game_duration: u64,
    pub queue_id: u64,
    pub teams: Vec<LeagueMatchTeam>,
}

impl LeagueMatchDetail {
    // TODO: move this into a trait that every model should implementservice
    // Consider CliDisplay { into_formatted_string() -> String }
    pub fn into_string(self) -> String {
        let mut result = String::from("");
        let participant_info = &self.get_participant_info("Ricefields");
        let team = &self.get_participant_team(participant_info.team_id);

        result.push_str(&format!("{}", if team.win { "Victory" } else { "Defeat" }));
        result.push_str(" @ ");
        result.push_str(&QUEUES.get(&self.queue_id).unwrap());
        result.push_str("\n");
        result.push_str(&format!(
            "{} / {} / {}",
            participant_info.stats.kills,
            participant_info.stats.deaths,
            participant_info.stats.assists
        ));
        result.push_str("\n");
        result.push_str(&format!(
            "> Duration: {}",
            format_game_duration(self.game_duration)
        ));
        result.push_str("\n");
        result.push_str(&format!(
            "> Champion: {}",
            CHAMPIONS.get(&participant_info.champion_id).unwrap()
        ));

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

    pub fn get_participant_team(&self, team_id: u64) -> LeagueMatchTeam {
        let team = &self
            .teams
            .to_owned()
            .into_iter()
            .find(|team| team.team_id == team_id)
            .unwrap();
        team.clone()
    }
}
