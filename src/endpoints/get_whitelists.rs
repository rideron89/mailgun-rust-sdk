//! Request and response module for fetching whitelist records for a domain.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     get_whitelists::{GetWhitelistsParam, GetWhitelistsParamList},
//! # };
//! # let client = Client::new("", "");
//! let request = GetWhitelistsParamList::default()
//!     .add(GetWhitelistsParam::Limit(1));
//!
//! let whitelists = client.get_whitelists(request).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#view-all-whitelist-records)

use crate::{Paging, Param, ParamError, ParamList};

//- Request

/// A parameter for fetching whitelist records for a domain.
#[derive(Debug)]
pub enum GetWhitelistsParam {
    /// Maximum number of records to return (default: 100, max: 10000).
    Limit(usize),
}

impl Param for GetWhitelistsParam {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::Limit(v) => ("limit".to_string(), v.to_string()),
        })
    }
}

/// List of parameters for fetching whitelist records for a domain.
#[derive(Debug)]
pub struct GetWhitelistsParamList {
    pub values: Vec<GetWhitelistsParam>,
}

impl Default for GetWhitelistsParamList {
    fn default() -> Self {
        Self {
            values: vec![],
        }
    }
}

impl ParamList for GetWhitelistsParamList {
    type ParamType = GetWhitelistsParam;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by get whitelist records endpoint.
#[derive(Debug, Deserialize)]
pub struct GetWhitelistsResponse {
    pub items: Vec<WhitelistItem>,
    pub paging: Paging,
}

/// A single item found in [`GetWhitelistsResponse`](struct.GetWhitelistsResponse.html).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhitelistItem {
    pub value: String,
    pub reason: String,
    pub r#type: String,
    pub created_at: String,
}
