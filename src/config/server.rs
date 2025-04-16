use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    main_config: MainConfig,
    
    forum_url: String,
    wiki_url: String,
    rules_url: String,
    github_url: String,


    exp_config: ExpConfig,

    picture_logging_camera: bool,

    inactivity_period: u32,
    afk_period: u32,

    kick_inactive: bool,

    allow_admin_ooccolor: bool,
    allow_admin_asaycolor: bool,

    id_console_jobslot_delay: u32,

    allow_vote_restart: bool,
    allow_vote_map: bool,
    allow_rock_the_vote: bool,

    max_rocking_votes: u32,

    vote_delay: u32,
    vote_period: u32,

    no_dead_vote: bool,
    default_no_vote: bool,

    allow_respawn: RespawnPermissions,
    
    respawn_delay: u32,

    ip_intel: IpIntelConfig,

    discord_bot_command_prefix: char,

    round_stats_url: String,
    game_log_url: String,
    ban_appeal: String,

    forbit_singulo_posession: bool,
    
    popup_admin_pm: bool,
    
    allow_holidays: bool,

    ticklag: f64,

    see_own_notes: bool,

    note_fresh_days: f64,
    note_stale_days: f64,

    auto_lag_switch_pop: u32,

    soft_popcap: u32,
    soft_popcap_message: String,

    hard_popcap: u32,
    hard_popcap_message: String,

    extreme_popcap: u32,
    extreme_popcap_message: String,

    // Used only for active donators
    donator_bypass_popcap: bool,

    notify_new_player_age: f64,

    notify_new_player_account_age: f64,

    panic_bunker: bool,
    panic_bunker_interview: bool,
    panic_bunker_living: u32,
    panic_bunker_message: String,
    panic_server_address: String,
    panic_server_name: String,

    announce_admin_logout: bool,
    announce_admin_logout_message: String,
    announce_admin_login: bool,
    announce_admin_login_message: String,

    map_rotation: bool,

    preferece_map_voting: u32,

    // Dangerous, giving all players admin could be problematic.
    auto_admin: bool,
    // Auto deadmin configuration (if admin is joining a round, remove their admin power for the round)
    auto_deadmin_always: bool,
    auto_deadmin_on_ready_or_latejoin: bool,
    auto_deadmin_antag: bool,
    auto_deadmin_heads: bool,
    auto_deadmin_sec: bool,
    auto_deadmin_silicon: bool,

    client_min_version: Version,

    second_topic_limit: u32,
    minute_topic_limit: u32,

    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    fix: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MainConfig {
    server_name: String,
    server_sql_name: String,
    hosted_by: String,
    public_address: String,
    server: String,
    station_name: String,

    lobby_countdown: u32,
    round_end_countdown: u32,
    enable_localhost_rank: bool,
    use_age_restriction_for_jobs: bool,
    use_account_age_for_jobs: bool,
    use_whitelist: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpConfig { 
    use_exp_tracking: bool,
    use_exp_restrictions_heads: bool,
    use_exp_restrictions_heads_hours: u32,
    use_exp_restrictions_heads_department: bool,
    use_exp_restrictions_other: bool,
    use_exp_restrictions_admin_bypass: bool,

    use_low_living_hour_intern: bool,
    use_low_living_hour_intern_hours: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpIntelConfig {
    ip_intel_rating_bad: f64,
    ip_intel_email: String,
    ip_intel_base: String,
    ip_intel_max_query_minute: u32,
    ip_intel_max_query_day: u32,
    ip_intel_reject_rate_limited: bool,
    ip_intel_reject_bad: bool,
    ip_intel_reject_unknown: bool,
    ip_intel_cache_length: u32,
    ip_intel_exempt_playtime_living: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RespawnPermissions {
    CannotRespawn,
    CanRespawn,
    RequireDifferentCharacter,
}

