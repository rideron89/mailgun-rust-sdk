//! Request and response module for sending messages from a domain.
//!
//! **Caution:** Not all send message request parameters have been tested. If
//! you notice any that do not work, please feel free to create a ticket, or
//! create a pull a request.
//!
//! ### Example
//!
//! ```no_run
//! # use mailgun_sdk::{
//! #     Client,
//! #     ParamList,
//! #     send_message::{SendMessageParam, SendMessageParamList},
//! # };
//! # let client = Client::new("", "");
//! let params = SendMessageParamList::default()
//!     .add(SendMessageParam::Html("<html><body><h1>Your Message</h1></body></html>"))
//!     .add(SendMessageParam::To("you@domain.com"))
//!     .add(SendMessageParam::From("Test <test@domain.com>"));
//!
//! client.send_message(params).unwrap();
//! ```
//!
//! [API Documentation](https://documentation.mailgun.com/en/latest/api-sending.html#sending)

use crate::{Param, ParamError, ParamList};

//- Request

/// A parameter for sending message from a domain.
#[derive(Debug)]
pub enum SendMessageParam<'a, T: ?Sized> where T: serde::Serialize {
    /// Email address for From header.
    From(&'a str),
    /// Email address of the recipient(s). Example: "Bob <bob@host.com>". You can use commas to separate multiple recipients.
    To(&'a str),
    /// Same as To but for Cc.
    Cc(&'a str),
    /// Same as To but for Bcc.
    Bcc(&'a str),
    /// Message subject.
    Subject(&'a str),
    /// Body of the message (text version).
    Text(&'a str),
    /// Body of the message (HTML version).
    Html(&'a str),
    /// AMP part of the message. Please follow google guidelines to compose and send AMP emails.
    AmpHtml(&'a str),
    /// File attachment. You can post multiple attachment values.
    Attachment(&'a str),
    /// Attachment with inline disposition. Can be used to send inline images (see example). You can post multiple inline values.
    Inline(&'a str),
    /// Name of a template stored via template API.
    Template(&'a str),
    /// Use this parameter to send a message to specific version of a template.
    TVersion(&'a str),
    /// Pass `yes` if you want to have rendered template in the text part of the message in case of template sending.
    TText(&'a str),
    /// Tag string.
    OTag(&'a str),
    /// Enables/disables DKIM signatures on per-message basis.
    ODkim(bool),
    /// Desired time of delivery. Note: Messages can be scheduled for a maximum of 3 days in the future.
    ODeliveryTime(&'a str),
    /// Toggles Send Time Optimization (STO) on a per-message basis. String should be set to the number of hours in `[0-9]+h` format, with the minimum being `24h` and the maximum being `72h`. This value defines the time window in which Mailgun will run the optimization algorithm based on prior engagement data of a given recipient. _Please note that STO is only available on certain plans._
    ODeliveryTimeOptimizePeriod(&'a str),
    /// Toggles Timezone Optimization (TZO) on a per message basis. String should be set to preferred delivery time in `HH:mm` or `hh:mmaa` format, where `HH:mm` is used for 24 hour format without AM/PM and `hh:mmaa` is used for 12 hour format with AM/PM. See Sending a message with TZO for details. _Please note that TZO is only available on certain plans._
    OTimeZoneLocalize(&'a str),
    /// Enables sending in test mode.
    OTestMode(bool),
    /// Toggles tracking on a per-message basis.
    OTracking(bool),
    /// Toggles clicks tracking on a per-message basis. Has higher priority than domain-level setting. Pass `yes`, `no`, `true`, `false` or `htmlonly`.
    OTrackingClicks(&'a str),
    /// Toggles opens tracking on a per-message basis. Has higher priority than domain-level setting.
    OTrackingOpens(bool),
    /// If `true`, the message will only be sent over a TLS connection. The message will fail if a TLS connection cannot be established. If `false`, MailGun will try to upgrade the connection, but still send over a plaintext STMP connection on failure.
    ORequireTls(bool),
    /// If `true`, the certificate and hostname will not be verified when trying to establish a TLS connection.
    OSkipVerification(bool),
    /// Add a custom header to the request.
    CustomHeader { key: &'a str, value: &'a str },
    /// Add custom JSON data to the message.
    CustomVariable { key: &'a str, value: &'a str },
    /// Add JSON data that can be referenced in the message body. Each key should be a plain recipient address and each value should be a dictionary.
    RecipientVariables(&'a T),
}

impl<'a, T: ?Sized> Param for SendMessageParam<'a, T> where T: serde::Serialize {
    fn try_as_tuple(&self) -> Result<(String, String), ParamError> {
        Ok(match self {
            Self::From(v) => ("from".to_string(), v.to_string()),
            Self::To(v) => ("to".to_string(), v.to_string()),
            Self::Cc(v) => ("cc".to_string(), v.to_string()),
            Self::Bcc(v) => ("bcc".to_string(), v.to_string()),
            Self::Subject(v) => ("subject".to_string(), v.to_string()),
            Self::Text(v) => ("text".to_string(), v.to_string()),
            Self::Html(v) => ("html".to_string(), v.to_string()),
            Self::AmpHtml(v) => ("amp-html".to_string(), v.to_string()),
            Self::Attachment(v) => ("attachment".to_string(), v.to_string()),
            Self::Inline(v) => ("inline".to_string(), v.to_string()),
            Self::Template(v) => ("template".to_string(), v.to_string()),
            Self::TVersion(v) => ("t:version".to_string(), v.to_string()),
            Self::TText(v) => ("t:text".to_string(), v.to_string()),
            Self::OTag(v) => ("o:tag".to_string(), v.to_string()),
            Self::ODkim(v) => ("o:dkim".to_string(), if *v { "yes".to_string() } else { "no".to_string() }),
            Self::ODeliveryTime(v) => ("o:delivery-time".to_string(), v.to_string()),
            Self::ODeliveryTimeOptimizePeriod(v) => ("o:delivery-time-optimize-period".to_string(), v.to_string()),
            Self::OTimeZoneLocalize(v) => ("o:time-zone-localize".to_string(), v.to_string()),
            Self::OTestMode(v) => ("o:testmode".to_string(), if *v { "yes".to_string() } else { "no".to_string() }),
            Self::OTracking(v) => ("o:tracking".to_string(), v.to_string()),
            Self::OTrackingClicks(v) => ("o:tracking-clicks".to_string(), v.to_string()),
            Self::OTrackingOpens(v) => ("o:tracking-open".to_string(), if *v { "yes".to_string() } else { "no".to_string() }),
            Self::ORequireTls(v) => ("o:require-tls".to_string(), if *v { "yes".to_string() } else { "no".to_string() }),
            Self::OSkipVerification(v) => ("o:skip-verification".to_string(), if *v { "yes".to_string() } else { "no".to_string() }),
            Self::CustomHeader { key, value } => (format!("h:{}", key), value.to_string()),
            Self::CustomVariable { key, value } => (format!("v:{}", key), value.to_string()),
            Self::RecipientVariables(v) => {
                let v = serde_json::to_string(v)
                    .map_err(|error| ParamError::InvalidJson("recipient-variables".to_string(), error))?;

                ("recipient-variables".to_string(), v)
            },
        })
    }
}

/// List of parameters for sending message from a domain.
#[derive(Debug)]
pub struct SendMessageParamList<'a, T: ?Sized> where T: serde::Serialize {
    pub values: Vec<SendMessageParam<'a, T>>,
}

impl<'a, T: ?Sized> Default for SendMessageParamList<'a, T> where T: serde::Serialize {
    fn default() -> Self {
        Self {
            values: vec![],
        }
    }
}

impl<'a, T: ?Sized> ParamList for SendMessageParamList<'a, T> where T: serde::Serialize {
    type ParamType = SendMessageParam<'a, T>;

    fn add(mut self, param: Self::ParamType) -> Self {
        self.values.push(param);

        self
    }
}

//- Response

/// Response returned by send message endpoint.
#[derive(Debug, Deserialize, Serialize)]
pub struct SendMessageResponse {
    pub id: String,
    pub message: String,
}
