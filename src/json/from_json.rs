use crate::models::{
    LeagueMatch, LeagueMatchDetail, LeagueMatchParticipant, LeagueMatchParticipantIdentity,
    LeagueMatchTeam, ParticipantStats, Player, LeagueMatches, LeagueSummoner
};
use crate::utils::JsonParser;
use serde_json::{Error, Value};

pub trait FromJson<T> {
    fn from_parser(json: JsonParser) -> T {
        unimplemented!("Method not implemented")
    }
    fn from_json_string(json: &str) -> Result<T, Error> {
        unimplemented!("Method not implemented")
    }
    fn from_json_object(json: Value) -> Result<T, Error> {
        unimplemented!("Method not implemented")
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

        let teams: Vec<LeagueMatchTeam> = json_parser
            .safe_read_array("teams")
            .into_iter()
            .map(|team: Value| {
                let win: bool = match team.get("win").unwrap().as_str().unwrap() {
                    "Win" => true,
                    _ => false,
                };

                let team_id = team.get("teamId").unwrap().as_u64().unwrap();

                LeagueMatchTeam { win, team_id }
            })
            .collect();

        let game_duration: u64 = json_parser.safe_read_int("gameDuration");
        let queue_id: u64 = json_parser.safe_read_int("queueId");

        Ok(LeagueMatchDetail {
            participants,
            participant_identities,
            game_duration,
            queue_id,
            teams,
        })
    }
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

impl FromJson<LeagueMatch> for LeagueMatch {
    fn from_parser(json_parser: JsonParser) -> LeagueMatch {
        LeagueMatch {
            platform_id: json_parser.safe_read_str("platformId"),
            game_id: json_parser.safe_read_int("gameId"),
            champion: json_parser.safe_read_int("champion"),
            queue: json_parser.safe_read_int("queue"),
            season: json_parser.safe_read_int("season"),
            timestamp: json_parser.safe_read_int("timestamp"),
            role: json_parser.safe_read_str("role"),
            lane: json_parser.safe_read_str("lane"),
        }
    }

    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);
        Ok(LeagueMatch::from_parser(json_parser))
    }

    fn from_json_object(json: Value) -> Result<Self, Error> {
        let json_parser: JsonParser = JsonParser::new(json);
        Ok(LeagueMatch::from_parser(json_parser))
    }
}

impl FromJson<LeagueMatches> for LeagueMatches {
    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        let mut matches: Vec<LeagueMatch> = Vec::new();

        let json_matches: Vec<Value> = json_parser.safe_read_array("matches");

        for mut _match in json_matches {
            matches.push(LeagueMatch::from_json_object(_match.take()).unwrap());
        }

        Ok(LeagueMatches { matches })
    }
}

impl FromJson<LeagueSummoner> for LeagueSummoner {
    fn from_parser(json_parser: JsonParser) -> LeagueSummoner {
        LeagueSummoner {
            id: json_parser.safe_read_str("id"),
            account_id: json_parser.safe_read_str("accountId"),
            puuid: json_parser.safe_read_str("puuid"),
            name: json_parser.safe_read_str("name"),
            profile_icon_id: json_parser.safe_read_int("profileIconId"),
            revision_date: json_parser.safe_read_int("revisionDate"),
            summoner_level: json_parser.safe_read_int("summonerLevel"),
        }
    }

    fn from_json_string(json: &str) -> Result<Self, Error> {
        let json_value: serde_json::Value = serde_json::from_str(json)?;
        let json_parser: JsonParser = JsonParser::new(json_value);

        Ok(LeagueSummoner::from_parser(json_parser))
    }
}
