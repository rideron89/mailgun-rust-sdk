//! Request and response module for fetching unsubscribes for a domain.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     get_unsubscribes::{GetUnsubscribesParam, GetUnsubscribesParamList},
//! # };
//! # let client = Client::new("", "");
//! let request = GetUnsubscribesParamList::default()
//!     .add(GetUnsubscribesParam::Limit(1));
//!
//! let unsubscribes = client.get_unsubscribes(request).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#unsubscribes)

use crate::{Paging, Param, ParamError, ParamList};

//- Request

/// A parameter for fetching unsubscribes for a domain.
#[derive(Debug)]
pub enum GetUnsubscribesParam {
    /// Maximum number of records to return (default: 100, max: 10000).
    Limit(usize),
}

impl Param for GetUnsubscribesParam {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::Limit(v) => ("limit".to_string(), v.to_string()),
        })
    }
}

/// List of parameters for fetching unsubscribes for a domain.
#[derive(Debug)]
pub struct GetUnsubscribesParamList {
    pub values: Vec<GetUnsubscribesParam>,
}

impl Default for GetUnsubscribesParamList {
    fn default() -> Self {
        Self {
            values: vec![],
        }
    }
}

impl ParamList for GetUnsubscribesParamList {
    type ParamType = GetUnsubscribesParam;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by get unsubscribes endpoint.
#[derive(Debug, Deserialize)]
pub struct GetUnsubscribesResponse {
    pub items: Vec<UnsubscribeItem>,
    pub paging: Paging,
}

/// A single item found in [`GetUnsubscribesResponse`](struct.GetUnsubscribesResponse.html).
#[derive(Debug, Deserialize)]
pub struct UnsubscribeItem {
    pub address: String,
    pub tag: String,
    pub created_at: String,
}
