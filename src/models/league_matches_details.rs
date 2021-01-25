use crate::models::{LeagueMatchDetail};

pub struct LeagueMatchesDetails {
    pub matches_details: Vec<LeagueMatchDetail>,
}

impl LeagueMatchesDetails {
    pub fn new() -> Self {
        LeagueMatchesDetails {
            matches_details: Vec::new()
        }
    }
    
    pub fn append_match(mut self, league_match_detail: LeagueMatchDetail) {
        self.matches_details.push(league_match_detail);
    }
}