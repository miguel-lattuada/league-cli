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
