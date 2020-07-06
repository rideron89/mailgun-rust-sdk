//! MailGun API client written in Rust.
//!
//! This crate helps facilitate interacting with the MailGun API. You will need
//! to supply both an *API Key* and *Domain*.
//!
//! [API Reference](https://documentation.mailgun.com/en/latest/api_reference.html)
//!
//! ### Send a Message
//!
//! ```no_run
//! use mailgun_sdk::{
//!     Client,
//!     ParamList,
//!     send_message::{SendMessageParam, SendMessageParamList},
//! };
//! let client = Client::new("ApiKey", "Domain");
//!
//! let params = SendMessageParamList::default()
//!     .add(SendMessageParam::To("to@test.com"))
//!     .add(SendMessageParam::From("from@your-domain.com"))
//!     .add(SendMessageParam::Subject("Test Message"))
//!     .add(SendMessageParam::Html(r#"<html>
//!         <body>
//!             <h1>Test Message</h1>
//!         </body>
//!     </html>"#));
//!
//! if let Err(error) = client.send_message(params) {
//!     eprintln!("Error: {:?}", error);
//! }
//! ```
//!
//! This crate does not enforce rules on sending messages. However, you should
//! almost always set the following when sending a message:
//!
//! - **Subject**
//! - **To**
//! - **From**
//! - **Html** _and/or_ **Text**
//!
//! **Caution:** Not all send message request parameters have been tested. If
//! you notice any that do not work, please feel free to create a ticket, or
//! create a pull a request.
//!
//! ### Pagination
//!
//! For API calls that return a list of results, MailGun returns a `paging`
//! structure. The paging fields are all URLs. Instead of having to parse these,
//! you may use the `call` method to fetch these pages.
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     get_bounces::GetBouncesParamList,
//! # };
//! # let client = Client::new("ApiKey", "Domain");
//! let mut response = client.get_bounces(GetBouncesParamList::default()).unwrap();
//! let mut bounces = response.items;
//!
//! if bounces.len() > 0 {
//!     loop {
//!         response = client.call(&response.paging.next).unwrap();
//!
//!         if response.items.len() == 0 {
//!             break;
//!         } else {
//!             bounces.append(&mut response.items);
//!         }
//!     }
//! }
//! ```
//!
//! ### Further Examples
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     get_bounces::GetBouncesParamList,
//! #     get_events::GetEventsParamList,
//! #     get_stats::GetStatsParamList,
//! # };
//! let client = Client::new("ApiKey", "Domain");
//!
//! // Get all events.
//! let events = client.get_events(GetEventsParamList::default()).unwrap();
//!
//! // Get all bounces.
//! let bounces = client.get_bounces(GetBouncesParamList::default()).unwrap();
//!
//! // Get account stats.
//! let stats = client.get_stats(GetStatsParamList::default()).unwrap();
//! ```

#[macro_use] extern crate serde;

mod client;
pub use client::{Client, ClientError};

mod endpoints;
pub use endpoints::*;

mod param;
pub use param::*;

/// Base URL for the MailGun API.
pub const MAILGUN_API_BASE: &'static str = "https://api.mailgun.net/v3";

#[cfg(test)]
pub mod test_util {
    #[derive(Debug)]
    pub struct Config {
        pub mailgun_api_key: String,
        pub mailgun_domain: String,
    }

    /// Load config settings from a local `.test.env` file.
    pub fn load_config() -> Config {
        dotenv::from_path("./.test.env").ok();

        Config {
            mailgun_api_key: dotenv::var("MAILGUN_API_KEY").unwrap(),
            mailgun_domain: dotenv::var("MAILGUN_DOMAIN").unwrap(),
        }
    }
}
