use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Steam64 ID for a player
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Steam64Id(pub String);

impl From<String> for Steam64Id {
    fn from(value: String) -> Self {
        Steam64Id(value)
    }
}

impl From<&str> for Steam64Id {
    fn from(value: &str) -> Self {
        Steam64Id(value.to_string())
    }
}

impl AsRef<str> for Steam64Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Leetify User ID (UUID format)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LeetifyId(pub String);

impl From<String> for LeetifyId {
    fn from(value: String) -> Self {
        LeetifyId(value)
    }
}

impl From<&str> for LeetifyId {
    fn from(value: &str) -> Self {
        LeetifyId(value.to_string())
    }
}

impl AsRef<str> for LeetifyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Player id - either a Steam64 ID or Leetify ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Steam64(Steam64Id),
    Leetify(LeetifyId),
}

impl From<Steam64Id> for Id {
    fn from(id: Steam64Id) -> Self {
        Id::Steam64(id)
    }
}

impl From<LeetifyId> for Id {
    fn from(id: LeetifyId) -> Self {
        Id::Leetify(id)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        // Leetify IDs are UUIDs in format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        // Steam64 IDs are numeric strings (typically 17 digits)
        if is_uuid_format(value) {
            Id::Leetify(LeetifyId(value.to_string()))
        } else if value.chars().all(|c| c.is_ascii_digit()) && value.len() >= 15 {
            Id::Steam64(Steam64Id(value.to_string()))
        } else {
            // Default to Leetify ID if format is unclear
            Id::Leetify(LeetifyId(value.to_string()))
        }
    }
}

/// Check if a string matches UUID format (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
fn is_uuid_format(s: &str) -> bool {
    // UUID format: 8-4-4-4-12 hex digits separated by hyphens
    // Example: 5ea07280-2399-4c7e-88ab-f2f7db0c449f
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    // Check each part has correct length and contains only hex digits
    let lengths = [8, 4, 4, 4, 12];
    parts
        .iter()
        .zip(lengths.iter())
        .all(|(part, &len)| part.len() == len && part.chars().all(|c| c.is_ascii_hexdigit()))
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

/// Data source for matches
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum DataSource {
    FACEIT,
    Matchmaking,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for DataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "faceit" => DataSource::FACEIT,
            "matchmaking" => DataSource::Matchmaking,
            other => DataSource::Other(other.to_string()),
        })
    }
}

impl DataSource {
    pub fn as_str(&self) -> &str {
        match self {
            DataSource::FACEIT => "faceit",
            DataSource::Matchmaking => "matchmaking",
            DataSource::Other(s) => s.as_str(),
        }
    }
}

impl From<String> for DataSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "faceit" => DataSource::FACEIT,
            "matchmaking" => DataSource::Matchmaking,
            _ => DataSource::Other(value),
        }
    }
}

impl From<&str> for DataSource {
    fn from(value: &str) -> Self {
        match value {
            "faceit" => DataSource::FACEIT,
            "matchmaking" => DataSource::Matchmaking,
            _ => DataSource::Other(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub privacy_mode: String,
    pub winrate: f64,
    pub total_matches: u32,
    #[serde(default)]
    pub first_match_date: Option<DateTime<Utc>>,
    pub name: String,
    pub bans: Vec<PlatformBanInfo>,
    pub steam64_id: String,
    #[serde(default)]
    pub id: Option<String>,
    pub ranks: Ranks,
    pub rating: Rating,
    pub stats: Stats,
    pub recent_matches: Vec<RecentMatch>,
    pub recent_teammates: Vec<RecentTeammate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranks {
    #[serde(default)]
    pub leetify: Option<f64>,
    #[serde(default)]
    pub premier: Option<u32>,
    #[serde(default)]
    pub faceit: Option<u32>,
    #[serde(default)]
    pub faceit_elo: Option<u32>,
    #[serde(default)]
    pub wingman: Option<u32>,
    #[serde(default)]
    pub renown: Option<u32>,
    pub competitive: Vec<CompetitiveRank>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitiveRank {
    pub map_name: String,
    pub rank: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub aim: f64,
    pub positioning: f64,
    pub utility: f64,
    pub clutch: f64,
    pub opening: f64,
    pub ct_leetify: f64,
    pub t_leetify: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub accuracy_enemy_spotted: f64,
    pub accuracy_head: f64,
    pub counter_strafing_good_shots_ratio: f64,
    pub ct_opening_aggression_success_rate: f64,
    pub ct_opening_duel_success_percentage: f64,
    pub flashbang_hit_foe_avg_duration: f64,
    pub flashbang_hit_foe_per_flashbang: f64,
    pub flashbang_hit_friend_per_flashbang: f64,
    pub flashbang_leading_to_kill: f64,
    pub flashbang_thrown: f64,
    pub he_foes_damage_avg: f64,
    pub he_friends_damage_avg: f64,
    pub preaim: f64,
    pub reaction_time_ms: f64,
    pub spray_accuracy: f64,
    pub t_opening_aggression_success_rate: f64,
    pub t_opening_duel_success_percentage: f64,
    pub traded_deaths_success_percentage: f64,
    pub trade_kill_opportunities_per_round: f64,
    pub trade_kills_success_percentage: f64,
    pub utility_on_death_avg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentMatch {
    pub id: String,
    pub finished_at: DateTime<Utc>,
    pub data_source: String,
    pub outcome: String,
    pub rank: u32,
    #[serde(default)]
    pub rank_type: Option<u32>,
    pub map_name: String,
    pub leetify_rating: f64,
    #[serde(deserialize_with = "deserialize_score")]
    pub score: [u32; 2],
    pub preaim: f64,
    pub reaction_time_ms: u32,
    pub accuracy_enemy_spotted: f64,
    pub accuracy_head: f64,
    pub spray_accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentTeammate {
    pub steam64_id: String,
    pub recent_matches_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformBanInfo {
    pub platform: String,
    pub platform_nickname: String,
    pub banned_since: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchDetailsResponse {
    pub id: String,
    pub finished_at: DateTime<Utc>,
    pub data_source: String,
    pub data_source_match_id: String,
    pub map_name: String,
    pub has_banned_player: bool,
    #[serde(deserialize_with = "deserialize_team_scores")]
    pub team_scores: [TeamScore; 2],
    pub stats: Vec<PlayerStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamScore {
    pub team_number: u32,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub steam64_id: String,
    pub name: String,
    pub mvps: u32,
    pub preaim: f64,
    pub reaction_time: f64,
    pub accuracy: f64,
    pub accuracy_enemy_spotted: f64,
    pub accuracy_head: f64,
    pub shots_fired_enemy_spotted: u32,
    pub shots_fired: u32,
    pub shots_hit_enemy_spotted: u32,
    pub shots_hit_friend: u32,
    pub shots_hit_friend_head: u32,
    pub shots_hit_foe: u32,
    pub shots_hit_foe_head: u32,
    pub utility_on_death_avg: f64,
    pub he_foes_damage_avg: f64,
    pub he_friends_damage_avg: f64,
    pub he_thrown: u32,
    pub molotov_thrown: u32,
    pub smoke_thrown: u32,
    pub counter_strafing_shots_all: u32,
    pub counter_strafing_shots_bad: u32,
    pub counter_strafing_shots_good: u32,
    pub counter_strafing_shots_good_ratio: f64,
    pub flashbang_hit_foe: u32,
    pub flashbang_leading_to_kill: u32,
    pub flashbang_hit_foe_avg_duration: f64,
    pub flashbang_hit_friend: u32,
    pub flashbang_thrown: u32,
    pub flash_assist: u32,
    pub score: u32,
    pub initial_team_number: u32,
    pub spray_accuracy: f64,
    pub total_kills: u32,
    pub total_deaths: u32,
    pub kd_ratio: f64,
    pub rounds_survived: u32,
    pub rounds_survived_percentage: f64,
    pub dpr: f64,
    pub total_assists: u32,
    pub total_damage: u32,
    #[serde(default)]
    pub leetify_rating: Option<f64>,
    #[serde(default)]
    pub ct_leetify_rating: Option<f64>,
    #[serde(default)]
    pub t_leetify_rating: Option<f64>,
    pub multi1k: u32,
    pub multi2k: u32,
    pub multi3k: u32,
    pub multi4k: u32,
    pub multi5k: u32,
    pub rounds_count: u32,
    pub rounds_won: u32,
    pub rounds_lost: u32,
    pub total_hs_kills: u32,
    pub trade_kill_opportunities: u32,
    pub trade_kill_attempts: u32,
    pub trade_kills_succeed: u32,
    pub trade_kill_attempts_percentage: f64,
    pub trade_kills_success_percentage: f64,
    pub trade_kill_opportunities_per_round: f64,
    pub traded_death_opportunities: u32,
    pub traded_death_attempts: u32,
    pub traded_deaths_succeed: u32,
    pub traded_death_attempts_percentage: f64,
    pub traded_deaths_success_percentage: f64,
    pub traded_deaths_opportunities_per_round: f64,
}

fn deserialize_score<'de, D>(deserializer: D) -> Result<[u32; 2], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec: Vec<u32> = Vec::deserialize(deserializer)?;
    if vec.len() == 2 {
        Ok([vec[0], vec[1]])
    } else {
        Err(serde::de::Error::invalid_length(
            vec.len(),
            &"exactly 2 elements",
        ))
    }
}

fn deserialize_team_scores<'de, D>(deserializer: D) -> Result<[TeamScore; 2], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec: Vec<TeamScore> = Vec::deserialize(deserializer)?;
    if vec.len() == 2 {
        // Convert Vec to array by taking ownership
        let mut iter = vec.into_iter();
        Ok([
            iter.next()
                .ok_or_else(|| serde::de::Error::custom("Missing first element"))?,
            iter.next()
                .ok_or_else(|| serde::de::Error::custom("Missing second element"))?,
        ])
    } else {
        Err(serde::de::Error::invalid_length(
            vec.len(),
            &"exactly 2 elements",
        ))
    }
}
