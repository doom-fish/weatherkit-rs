//! WeatherKit error types.

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::private::parse_json_from_static;

/// Names the bridge error domain used by the WeatherKit wrapper.
pub const WEATHERKIT_BRIDGE_ERROR_DOMAIN: &str = "WeatherKitBridge";

/// Represents an error returned by WeatherKit or the bridge layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatherKitError {
    /// Matches the WeatherKit domain value.
    pub domain: String,
    /// Matches the WeatherKit code value.
    pub code: i64,
    /// Matches the WeatherKit message value.
    pub message: String,
}

/// Represents a typed WeatherKit error case.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeatherError {
    /// Matches the WeatherKit `PermissionDenied` case.
    PermissionDenied,
    /// Stores an unrecognized WeatherKit case name.
    Unknown(String),
}

/// Represents the WeatherKit `WeatherErrorDescriptor` payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherErrorDescriptor {
    /// Matches the WeatherKit raw value value.
    pub raw_value: String,
    /// Matches the WeatherKit error description value.
    pub error_description: Option<String>,
    /// Matches the WeatherKit failure reason value.
    pub failure_reason: Option<String>,
    /// Matches the WeatherKit help anchor value.
    pub help_anchor: Option<String>,
    /// Matches the WeatherKit recovery suggestion value.
    pub recovery_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorPayload {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

impl WeatherError {
    /// Returns the WeatherKit raw value for this case.
    pub fn raw_value(&self) -> &str {
        match self {
            Self::PermissionDenied => "permissionDenied",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Returns the WeatherKit descriptor catalog for this enum.
    pub fn descriptors() -> Result<Vec<WeatherErrorDescriptor>, WeatherKitError> {
        parse_json_from_static(
            ffi::error::wk_weather_error_copy_descriptors_json,
            "weather error descriptors",
        )
    }
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

    /// Returns whether this WeatherKit error looks like an entitlement or authorization issue.
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
