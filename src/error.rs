use core::fmt;

use serde::{Deserialize, Serialize};

pub const WEATHERKIT_BRIDGE_ERROR_DOMAIN: &str = "WeatherKitBridge";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatherKitError {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorPayload {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

impl WeatherKitError {
    pub(crate) fn from_payload(payload: ErrorPayload) -> Self {
        Self {
            domain: payload.domain,
            code: payload.code,
            message: payload.message,
        }
    }

    pub(crate) fn bridge(code: i64, message: impl Into<String>) -> Self {
        Self {
            domain: WEATHERKIT_BRIDGE_ERROR_DOMAIN.into(),
            code,
            message: message.into(),
        }
    }

    pub fn is_entitlement_issue(&self) -> bool {
        let domain = self.domain.to_ascii_lowercase();
        let message = self.message.to_ascii_lowercase();
        message.contains("entitlement")
            || message.contains("permission")
            || message.contains("bundle")
            || message.contains("not authorized")
            || message.contains("denied")
            || domain.contains("jwtauthenticator")
            || domain.contains("weatherdaemon")
            || domain.contains("auth")
    }
}

impl fmt::Display for WeatherKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) [{}]", self.message, self.code, self.domain)
    }
}

impl std::error::Error for WeatherKitError {}
