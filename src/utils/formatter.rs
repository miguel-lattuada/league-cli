pub fn format_game_duration(duration: u64) -> String {
    return format!("{}m {}s", duration / 60, duration % 60);
}