pub mod google;
pub mod microsoft;

use async_trait::async_trait;
use chrono::prelude::*;

use secret_structs::secret::*;
use secret_structs::ternary_lattice as sec_lat;
use secret_structs::integrity_lattice as int_lat;

pub struct Calendar {
    pub account_id: u32,
    pub id: String,
    pub name: String,
    pub selected: bool,
}

impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Event {
    pub id: String,
    pub name: Option<String>,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}
unsafe impl InvisibleSideEffectFree for Event {}

#[async_trait]
pub trait GetResources {
    async fn get_calendars(token: &str) -> anyhow::Result<Vec<Calendar>>;
    async fn get_calendar_events(
        total_duration: &mut std::time::Duration,
        token: &str,
        calendar_id: &str,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
    ) -> anyhow::Result<SecureValue<Vec<Event>, sec_lat::Label_Empty, int_lat::Label_All, DynLabel<Sec>, ()>>;
    async fn create_event(
        token: &str,
        calendar_id: &str,
        title: &str,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
    ) -> anyhow::Result<()>;
}
