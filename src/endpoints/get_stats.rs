//! Request and response module for fetching stats for a domain.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     get_stats::{GetStatsParam, GetStatsParamList},
//! # };
//! # let client = Client::new("", "");
//! let request = GetStatsParamList::default()
//!     .add(GetStatsParam::Resolution("month"));
//!
//! let stats = client.get_stats(request).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-stats.html)

use crate::{Param, ParamError, ParamList};

//- Request

/// A parameter for fetching stats for a domain.
#[derive(Debug)]
pub enum GetStatsParam<'a> {
    /// Period of time with resoluton encoded. If provided, overwrites the
    /// start date.
    Duration(&'a str),
    /// The ending date. Should be in RFC 2822 or unix epoch format.
    /// Default: current time.
    End(&'a str),
    /// The type of the event.
    Event(&'a str),
    /// Can be either hour, day or month. Default: day.
    Resolution(&'a str),
    /// The starting time. Should be in RFC 2822 or unix epoch format.
    /// Default: 7 days from the current time.
    Start(&'a str),
}

impl<'a> Param for GetStatsParam<'a> {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::Duration(v) => ("duration".to_string(), v.to_string()),
            Self::End(v) => ("end".to_string(), v.to_string()),
            Self::Event(v) => ("event".to_string(), v.to_string()),
            Self::Resolution(v) => ("resolution".to_string(), v.to_string()),
            Self::Start(v) => ("start".to_string(), v.to_string()),
        })
    }
}

/// List of parameters for fetching stats for a domain.
#[derive(Debug)]
pub struct GetStatsParamList<'a> {
    pub values: Vec<GetStatsParam<'a>>,
}

impl<'a> Default for GetStatsParamList<'a> {
    fn default() -> Self {
        Self {
            values: vec![
                GetStatsParam::Event("accepted"),
                GetStatsParam::Event("delivered"),
                GetStatsParam::Event("failed"),
            ],
        }
    }
}

impl<'a> ParamList for GetStatsParamList<'a> {
    type ParamType = GetStatsParam<'a>;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by get stats endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct GetStatsResponse {
    end: String,
    resolution: String,
    start: String,
    stats: Vec<StatItem>,
}

/// A single item found in [`GetStatsResponse`](struct.GetStatsResponse.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct StatItem {
    time: String,
    accepted: StatAccepted,
    delivered: StatDelivered,
    failed: StatFailed,
}

/// A single accepted item found in [`StatItem`](struct.StatItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct StatAccepted {
    outgoing: i64,
    incoming: i64,
    total: i64,
}

/// A single delivered item found in [`StatItem`](struct.StatItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct StatDelivered {
    smtp: i64,
    http: i64,
    total: i64,
}

/// A single failed item found in [`StatItem`](struct.StatItem.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct StatFailed {
    permanent: StatFailedPermanent,
    temporary: StatFailedTemporary,
}

/// A single failed permanent item found in [`StatFailed`](struct.StatFailed.html).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StatFailedPermanent {
    bounce: i64,
    delayed_bounce: i64,
    suppress_bounce: i64,
    suppress_unsubscribe: i64,
    suppress_complaint: i64,
    total: i64,
}

/// A single failed temporary item found in [`StatFailed`](struct.StatFailed.html).
#[derive(Debug, Deserialize, Serialize)]
pub struct StatFailedTemporary {
    espblock: i64,
}
