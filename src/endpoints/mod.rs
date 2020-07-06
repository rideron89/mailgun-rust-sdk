//! API endpoints and their request/response structures.

pub mod get_bounces;
pub mod get_complaints;
pub mod get_events;
pub mod get_stats;
pub mod get_unsubscribes;
pub mod get_whitelists;
pub mod send_message;

/// Structure of pagination data returned by some API endpoints.
#[derive(Debug, Deserialize)]
pub struct Paging {
    pub first: Option<String>,
    pub next: String,
    pub previous: String,
    pub last: Option<String>,
}
