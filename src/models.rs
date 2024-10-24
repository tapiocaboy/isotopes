use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(FromRow)]
pub struct AiModule {
    pub ai_id: u32,
    pub business_id: String,
    pub module_id: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub active: bool,
}

#[derive(FromRow)]
pub struct RawModule {
    pub raw_module_id: u32,
    pub module_name: String,
    pub module_code: String,
    pub hosted_link: String,
}

#[derive(FromRow)]
pub struct Machine {
    pub machine_id: u32,
    pub machine_name: String,
    pub owner: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub active: bool,
    pub raw_module_id: u32,
}
