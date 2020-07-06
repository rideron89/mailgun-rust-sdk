use crate::endpoints::{
    get_bounces::{GetBouncesParamList, GetBouncesResponse},
    get_complaints::{GetComplaintsParamList, GetComplaintsResponse},
    get_events::{GetEventsParamList, GetEventsResponse},
    get_stats::{GetStatsParamList, GetStatsResponse},
    get_unsubscribes::{GetUnsubscribesParamList, GetUnsubscribesResponse},
    get_whitelists::{GetWhitelistsParamList, GetWhitelistsResponse},
    send_message::{SendMessageParamList, SendMessageResponse},
};
use crate::param::{Param, ParamError};
use crate::MAILGUN_API_BASE;
use thiserror::Error;

use std::io;

#[derive(Debug)]
pub struct Client {
    api_key: String,
    domain: String,
}

impl Client {
    /// Create a new client.
    pub fn new(api_key: &str, domain: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            domain: domain.to_string(),
        }
    }

    /// Make an API call from a URL.
    ///
    /// This will primarily be used with pagination URLs.
    pub fn call<T>(&self, url: &str) -> Result<T, ClientError>
    where T: serde::de::DeserializeOwned {
        let response = ureq::get(url)
            .auth("api", &self.api_key)
            .call();

        let status = response.status();
        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    /// View all bounces.
    ///
    /// [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#bounces)
    pub fn get_bounces(&self, params: GetBouncesParamList) -> Result<GetBouncesResponse, ClientError> {
        let url = format!("{}/{}/bounces", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::get(&url);
        request.auth("api", &self.api_key);

        for (key, value) in params.values.iter().map(|param| param.as_tuple()) {
            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    /// View all complaints.
    ///
    /// [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#view-all-complaints)
    pub fn get_complaints(&self, params: GetComplaintsParamList) -> Result<GetComplaintsResponse, ClientError> {
        let url = format!("{}/{}/complaints", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::get(&url);
        request.auth("api", &self.api_key);

        for (key, value) in params.values.iter().map(|param| param.as_tuple()) {
            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    /// View all events.
    ///
    /// [API Documentation](https://documentation.mailgun.com/en/latest/api-events.html)
    pub fn get_events(&self, params: GetEventsParamList) -> Result<GetEventsResponse, ClientError> {
        let url = format!("{}/{}/events", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::get(&url);
        request.auth("api", &self.api_key);

        for (key, value) in params.values.iter().map(|param| param.as_tuple()) {
            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    /// View all stats.
    ///
    /// [API Documentation](https://documentation.mailgun.com/en/latest/api-stats.html)
    pub fn get_stats(&self, params: GetStatsParamList) -> Result<GetStatsResponse, ClientError> {
        let url = format!("{}/{}/stats/total", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::get(&url);
        request.auth("api", &self.api_key);

        for (key, value) in params.values.iter().map(|param| param.as_tuple()) {
            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    /// View all unsubscribes.
    ///
    /// [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#unsubscribes)
    pub fn get_unsubscribes(&self, params: GetUnsubscribesParamList) -> Result<GetUnsubscribesResponse, ClientError> {
        let url = format!("{}/{}/unsubscribes", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::get(&url);
        request.auth("api", &self.api_key);

        for (key, value) in params.values.iter().map(|param| param.as_tuple()) {
            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    /// View all whitelist records.
    ///
    /// [API Documentation](https://documentation.mailgun.com/en/latest/api-suppressions.html#view-all-whitelist-records)
    pub fn get_whitelists(&self, params: GetWhitelistsParamList) -> Result<GetWhitelistsResponse, ClientError> {
        let url = format!("{}/{}/whitelists", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::get(&url);
        request.auth("api", &self.api_key);

        for (key, value) in params.values.iter().map(|param| param.as_tuple()) {
            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }

    pub fn send_message(&self, params: SendMessageParamList<String>) -> Result<SendMessageResponse, ClientError> {
        let url = format!("{}/{}/messages", MAILGUN_API_BASE, self.domain);

        let mut request = ureq::post(&url);
        request.auth("api", &self.api_key);

        for param in params.values {
            let (key, value) = param.try_as_tuple()?;

            // TODO: If key == "attachment", set content-type to "multipart/form-data".

            request.query(&key, &value);
        }

        let response = request.call();
        let status = response.status();

        let raw = response
            .into_string()
            .map_err(|error| ClientError::ReadResponse(error))?;

        if status != 200 {
            if let Ok(error) = serde_json::from_str::<ErrorResponse>(&raw) {
                return Err(ClientError::ApiError(error));
            }

            return Err(ClientError::HttpError(status, raw));
        }

        serde_json::from_str(&raw).map_err(|error| ClientError::ParseResponse(error))
    }
}

#[derive(Debug, Deserialize, Error)]
#[serde(untagged)]
pub enum ErrorResponse {
    #[error("With error: {error}")]
    WithError {
        #[serde(alias = "Error" )]
        error: String
    },
    #[error("With message: {message}")]
    WithMessage {
        message: String,
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Received an error message from the server: {0}")]
    ApiError(#[from] ErrorResponse),

    #[error("Received a {0} HTTP status code: {1}")]
    HttpError(u16, String),

    #[error("A request parameter is invalid: {0}")]
    ParamError(#[from] ParamError),

    #[error("Failed to parse response string: {0}")]
    ParseResponse(serde_json::error::Error),

    #[error("Failed to read response string: {0}")]
    ReadResponse(io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::{
        get_bounces::{GetBouncesParam, GetBouncesParamList},
        get_complaints::{GetComplaintsParam, GetComplaintsParamList},
        get_events::{GetEventsParam, GetEventsParamList, GetEventsResponse},
        get_stats::GetStatsParamList,
        get_unsubscribes::{GetUnsubscribesParam, GetUnsubscribesParamList},
        get_whitelists::{GetWhitelistsParam, GetWhitelistsParamList},
        send_message::{SendMessageParam, SendMessageParamList},
    };
    use crate::param::ParamList;
    use crate::test_util::load_config;

    #[test]
    fn call() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let all = client.get_events(GetEventsParamList::default()).unwrap();
        let _: GetEventsResponse = client.call(&all.paging.next).unwrap();
    }

    #[test]
    fn get_bounces() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let _all = client.get_bounces(GetBouncesParamList::default()).unwrap();

        let params = GetBouncesParamList::default()
            .add(GetBouncesParam::Limit(1));
        let _single = client.get_bounces(params).unwrap();

        // TODO: Test the response.
    }

    #[test]
    fn get_complains() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let _all = client.get_complaints(GetComplaintsParamList::default()).unwrap();

        let params = GetComplaintsParamList::default()
            .add(GetComplaintsParam::Limit(1));
        let _single = client.get_complaints(params).unwrap();

        // TODO: Test the response.
    }

    #[test]
    fn get_events() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let _all = client.get_events(GetEventsParamList::default()).unwrap();

        let params = GetEventsParamList::default()
            .add(GetEventsParam::Limit(1));
        let _single = client.get_events(params).unwrap();
    }

    #[test]
    fn get_stats() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let _response = client.get_stats(GetStatsParamList::default()).unwrap();

        // TODO: Test the response.
    }

    #[test]
    fn get_unsubscribes() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let _all = client.get_unsubscribes(GetUnsubscribesParamList::default()).unwrap();

        let params = GetUnsubscribesParamList::default()
            .add(GetUnsubscribesParam::Limit(1));
        let _single = client.get_unsubscribes(params).unwrap();

        // TODO: Test the response.
    }

    #[test]
    fn get_whitelists() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let _all = client.get_whitelists(GetWhitelistsParamList::default()).unwrap();

        let params = GetWhitelistsParamList::default()
            .add(GetWhitelistsParam::Limit(1));
        let _single = client.get_whitelists(params).unwrap();

        // TODO: Test the response.
    }

    #[test]
    fn send_message() {
        let config = load_config();
        let client = Client::new(&config.mailgun_api_key, &config.mailgun_domain);

        let from = format!("Test <test@{}>", &config.mailgun_domain);
        let params = SendMessageParamList::default()
            .add(SendMessageParam::Text("test message"))
            .add(SendMessageParam::To("rrider@pfgcapital.com"))
            .add(SendMessageParam::From(&from))
            .add(SendMessageParam::OTestMode(true));

        client.send_message(params).unwrap();
    }
}
