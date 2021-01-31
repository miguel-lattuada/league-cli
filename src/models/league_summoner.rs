// TODO: Move profile icon id to u8, summoner levl to u16
// Add new methods to json parser to read those values (cast from u64)
pub struct LeagueSummoner {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub profile_icon_id: u64,
    pub revision_date: u64,
    pub summoner_level: u64,
}
