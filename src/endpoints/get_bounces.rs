//! Request and response module for fetching bounces for a domain.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     get_bounces::{GetBouncesParam, GetBouncesParamList},
//! # };
//! # let client = Client::new("", "");
//! let request = GetBouncesParamList::default()
//!     .add(GetBouncesParam::Limit(1));
//!
//! let bounces = client.get_bounces(request).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#bounces)

use crate::{Paging, Param, ParamError, ParamList};

//- Request

/// A parameter for fetching bounces for a domain.
#[derive(Debug)]
pub enum GetBouncesParam {
    /// Maximum number of records to return (default: 100, max: 10000).
    Limit(usize),
}

impl Param for GetBouncesParam {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::Limit(v) => ("limit".to_string(), v.to_string()),
        })
    }
}

/// List of parameters for fetching bounces for a domain.
#[derive(Debug)]
pub struct GetBouncesParamList {
    pub values: Vec<GetBouncesParam>,
}

impl Default for GetBouncesParamList {
    fn default() -> Self {
        Self {
            values: vec![],
        }
    }
}

impl ParamList for GetBouncesParamList {
    type ParamType = GetBouncesParam;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by get bounces endpoint.
#[derive(Debug, Deserialize)]
pub struct GetBouncesResponse {
    pub items: Vec<BounceItem>,
    pub paging: Paging,
}

/// A single item found in [`GetBouncesResponse`](struct.GetBouncesResponse.html).
#[derive(Debug, Deserialize)]
pub struct BounceItem {
    pub address: String,
    pub code: String,
    pub error: String,
    pub created_at: String,
}
