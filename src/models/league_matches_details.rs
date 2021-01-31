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

    // TODO: move this into a trait that every model should implement
    // Consider CliDisplay { into_formatted_string() -> String }
    pub fn into_string(self) -> String {
        let mut result = String::from("");
        for match_detail in self.matches_details {
            result.push_str(&match_detail.into_string());
            result.push_str("\n");
        }
        result
    }
}