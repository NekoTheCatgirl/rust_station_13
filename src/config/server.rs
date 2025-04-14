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

