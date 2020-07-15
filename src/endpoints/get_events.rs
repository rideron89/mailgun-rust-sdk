//! Request and response module for fetching events for a domain.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     get_events::{GetEventsParam, GetEventsParamList},
//! # };
//! # let client = Client::new("", "");
//! let request = GetEventsParamList::default()
//!     .add(GetEventsParam::Limit(1));
//!
//! let events = client.get_events(request).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-events.html#events)

use crate::{Paging, Param, ParamError, ParamList};

//- Request

/// A parameter for fetching events for a domain.
#[derive(Debug)]
pub enum GetEventsParam<'a> {
    /// Pretty print the result JSON.
    Pretty(bool),
    /// The beginning of the search time range.
    Begin(&'a str),
    /// The end of the search time range.
    End(&'a str),
    /// Defines the direction of the search time range and must be provided if
    /// the range end time is not specified.
    Ascending(bool),
    /// Number of entries to return (300 max).
    Limit(usize),
    /// An event type.
    Event(&'a str),
    /// The email address of a mailing list the message was originally sent to.
    List(&'a str),
    /// A name of an attached file.
    Attachment(&'a str),
    /// An email address mentioned in the from MIME header.
    From(&'a str),
    /// A Mailgun message id returned by the messages API.
    MessageId(&'a str),
    /// A subject line.
    Subject(&'a str),
    /// An email address mentioned in the to MIME header.
    To(&'a str),
    /// Message size.
    Size(&'a str),
    /// An email address of a particular recipient. While messages are
    /// addressable to one or more recipients, each event (with one exception)
    /// tracks one recipient.
    Recipient(&'a str),
    /// Specific to stored events, this field tracks all of the potential
    /// message recipients.
    Recipients(&'a str),
    /// User defined tags.
    Tags(&'a str),
    /// Temporary or Permanent. Used to filter events based on severity, if
    /// exists (Currently failed events only).
    Severity(&'a str),
}

impl<'a> Param for GetEventsParam<'a> {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::Pretty(v) => ("pretty".to_string(), v.to_string()),
            Self::Begin(v) => ("begin".to_string(), v.to_string()),
            Self::End(v) => ("end".to_string(), v.to_string()),
            Self::Ascending(v) => ("ascending".to_string(), if *v { "yes".to_string() } else { "no".to_string() }),
            Self::Limit(v) => ("limit".to_string(), v.to_string()),
            Self::Event(v) => ("event".to_string(), v.to_string()),
            Self::List(v) => ("list".to_string(), v.to_string()),
            Self::Attachment(v) => ("attachment".to_string(), v.to_string()),
            Self::From(v) => ("from".to_string(), v.to_string()),
            Self::MessageId(v) => ("message_id".to_string(), v.to_string()),
            Self::Subject(v) => ("subject".to_string(), v.to_string()),
            Self::To(v) => ("to".to_string(), v.to_string()),
            Self::Size(v) => ("size".to_string(), v.to_string()),
            Self::Recipient(v) => ("recipient".to_string(), v.to_string()),
            Self::Recipients(v) => ("recipients".to_string(), v.to_string()),
            Self::Tags(v) => ("tags".to_string(), v.to_string()),
            Self::Severity(v) => ("severity".to_string(), v.to_string()),
        })
    }
}

/// List of parameters for fetching events for a domain.
#[derive(Debug)]
pub struct GetEventsParamList<'a> {
    pub values: Vec<GetEventsParam<'a>>,
}

impl<'a> Default for GetEventsParamList<'a> {
    fn default() -> Self {
        Self {
            values: vec![
                GetEventsParam::Pretty(false),
            ],
        }
    }
}

impl<'a> ParamList for GetEventsParamList<'a> {
    type ParamType = GetEventsParam<'a>;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by get events endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetEventsResponse {
    pub items: Vec<EventItem>,
    pub paging: Paging,
}

/// A single item found in [`GetEventsResponse`](struct.GetEventsResponse.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventItem {
    pub event: String,
    pub id: String,
    pub timestamp: f64,
    pub log_level: Option<String>,
    pub method: Option<String>,
    pub severity: Option<String>,
    pub envelope: Option<EventEnvelope>,
    pub flags: Option<EventFlags>,
    pub reject: Option<EventReject>,
    pub delivery_status: Option<EventDeliveryStatus>,
    pub message: EventMessage,
    pub storage: Option<EventStorage>,
    pub recipient: String,
    pub recipient_domain: Option<String>,
    pub geolocation: Option<EventGeolocation>,
    // TODO: Figure out what the `campaigns` field looks like.
    pub tags: Option<Vec<String>>,
    pub ip: Option<String>,
    pub client_info: Option<EventClientInfo>,
    // TODO: Figure out what the `user-variables` field looks like.
}

/// A single envelope item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct EventEnvelope {
    pub targets: String,
    pub transport: String,
    pub sender: String,
}

/// A single event flag item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventFlags {
    pub is_authenticated: Option<bool>,
    pub is_delayed_bounce: Option<bool>,
    pub is_routed: Option<bool>,
    pub is_system_test: Option<bool>,
    pub is_test_mode: Option<bool>,
}

/// A single event reject item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct EventReject {
    pub reason: Option<String>,
    pub description: Option<String>,
}

/// A single event delivery status item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventDeliveryStatus {
    pub tls: Option<bool>,
    pub mx_host: Option<String>,
    pub code: Option<i64>,
    pub description: Option<String>,
    pub session_seconds: Option<f64>,
    pub utf8: Option<bool>,
    pub attempt_no: Option<usize>,
    pub message: Option<String>,
    pub certificate_verified: Option<bool>,
}

/// A single event message item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct EventMessage {
    pub headers: Option<EventMessageHeader>,
    pub attachments: Option<Vec<EventMessageAttachment>>,
    pub recipients: Option<Vec<String>>,
    pub size: Option<i64>,
}

/// A single event message header item found in [`EventMessage`](struct.EventMessage.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventMessageHeader {
    pub to: Option<String>,
    pub message_id: Option<String>,
    pub from: Option<String>,
    pub subject: Option<String>,
}

/// A single event message attachment item found in [`EventMessage`](struct.EventMessage.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventMessageAttachment {
    pub size: Option<i64>,
    pub content_type: Option<String>,
    pub filename: Option<String>,
}

/// A single event storage item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct EventStorage {
    pub url: String,
    pub key: String,
}

/// A single event geolocation item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct EventGeolocation {
    pub country: String,
    pub region: String,
    pub city: String,
}

/// A single event client info item found in [`EventItem`](struct.EventItem.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventClientInfo {
    pub client_type: Option<String>,
    pub client_os: Option<String>,
    pub device_type: Option<String>,
    pub client_name: Option<String>,
    pub user_agent: Option<String>,
}
