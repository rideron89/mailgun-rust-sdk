//! Request and response module for fetching complaints for a domain.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     get_complaints::{GetComplaintsParam, GetComplaintsParamList},
//! # };
//! # let client = Client::new("", "");;
//! let request = GetComplaintsParamList::default()
//!     .add(GetComplaintsParam::Limit(1));
//!
//! let complaints = client.get_complaints(request).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#view-all-complaints)

use crate::{Paging, Param, ParamError, ParamList};

//- Request

/// A parameter for fetching complaints for a domain.
#[derive(Debug)]
pub enum GetComplaintsParam {
    /// Maximum number of records to return (default: 100, max: 10000).
    Limit(usize),
}

impl Param for GetComplaintsParam {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::Limit(v) => ("limit".to_string(), v.to_string()),
        })
    }
}

/// List of parameters for fetching complaints for a domain.
#[derive(Debug)]
pub struct GetComplaintsParamList {
    pub values: Vec<GetComplaintsParam>,
}

impl Default for GetComplaintsParamList {
    fn default() -> Self {
        Self {
            values: vec![],
        }
    }
}

impl ParamList for GetComplaintsParamList {
    type ParamType = GetComplaintsParam;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by get complaints endpoint.
#[derive(Debug, Deserialize)]
pub struct GetComplaintsResponse {
    pub items: Vec<ComplaintItem>,
    pub paging: Paging,
}

/// A single item found in [`GetComplaintsResponse`](struct.GetComplaintsResponse.html).
#[derive(Debug, Deserialize)]
pub struct ComplaintItem {
    pub address: String,
    pub tag: String,
    pub created_at: String,
}
