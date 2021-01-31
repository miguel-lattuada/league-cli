#[derive(Clone)]
pub struct ParticipantStats {
    pub deaths: u64,
    pub kills: u64,
    pub assists: u64,
}

#[derive(Clone)]
pub struct LeagueMatchParticipant {
    pub participant_id: u64,
    pub account_id: String,
    pub champion_id: u64,
    pub team_id: u64,
    pub spell_one_id: u64,
    pub spell_two_id: u64,
    pub highest_rank: String,
    pub stats: ParticipantStats,
}

impl LeagueMatchParticipant {
    // TODO: move this into a trait that every model should implement
    // Consider CliDisplay { into_formatted_string() -> String }
    #[allow(dead_code)]
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
