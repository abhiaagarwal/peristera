mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
use reqwest::header::AUTHORIZATION;
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///App
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "organization": {
    ///      "$ref": "#/components/schemas/Organization"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct App {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub organization: Option<Organization>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
    }

    impl From<&App> for App {
        fn from(value: &App) -> Self {
            value.clone()
        }
    }

    impl App {
        pub fn builder() -> builder::App {
            Default::default()
        }
    }

    ///CheckStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "output": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CheckStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub output: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
    }

    impl From<&CheckStatus> for CheckStatus {
        fn from(value: &CheckStatus) -> Self {
            value.clone()
        }
    }

    impl CheckStatus {
        pub fn builder() -> builder::CheckStatus {
            Default::default()
        }
    }

    ///CreateAppRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "app_name": {
    ///      "type": "string"
    ///    },
    ///    "enable_subdomains": {
    ///      "type": "boolean"
    ///    },
    ///    "network": {
    ///      "type": "string"
    ///    },
    ///    "org_slug": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateAppRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub app_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enable_subdomains: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub org_slug: Option<String>,
    }

    impl From<&CreateAppRequest> for CreateAppRequest {
        fn from(value: &CreateAppRequest) -> Self {
            value.clone()
        }
    }

    impl CreateAppRequest {
        pub fn builder() -> builder::CreateAppRequest {
            Default::default()
        }
    }

    ///CreateLeaseRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "ttl": {
    ///      "description": "seconds lease will be valid",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateLeaseRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///seconds lease will be valid
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ttl: Option<i64>,
    }

    impl From<&CreateLeaseRequest> for CreateLeaseRequest {
        fn from(value: &CreateLeaseRequest) -> Self {
            value.clone()
        }
    }

    impl CreateLeaseRequest {
        pub fn builder() -> builder::CreateLeaseRequest {
            Default::default()
        }
    }

    ///CreateMachineRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "description": "An object defining the Machine configuration",
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/fly.MachineConfig"
    ///        }
    ///      ]
    ///    },
    ///    "lease_ttl": {
    ///      "type": "integer"
    ///    },
    ///    "lsvd": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "description": "Unique name for this Machine. If omitted, one is
    /// generated for you",
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "description": "The target region. Omitting this param launches in
    /// the same region as your WireGuard peer connection (somewhere near
    /// you).",
    ///      "type": "string"
    ///    },
    ///    "skip_launch": {
    ///      "type": "boolean"
    ///    },
    ///    "skip_service_registration": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateMachineRequest {
        ///An object defining the Machine configuration
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<FlyMachineConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_ttl: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lsvd: Option<bool>,
        ///Unique name for this Machine. If omitted, one is generated for you
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///The target region. Omitting this param launches in the same region
        /// as your WireGuard peer connection (somewhere near you).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_launch: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_service_registration: Option<bool>,
    }

    impl From<&CreateMachineRequest> for CreateMachineRequest {
        fn from(value: &CreateMachineRequest) -> Self {
            value.clone()
        }
    }

    impl CreateMachineRequest {
        pub fn builder() -> builder::CreateMachineRequest {
            Default::default()
        }
    }

    ///CreateOidcTokenRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "aud": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateOidcTokenRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aud: Option<String>,
    }

    impl From<&CreateOidcTokenRequest> for CreateOidcTokenRequest {
        fn from(value: &CreateOidcTokenRequest) -> Self {
            value.clone()
        }
    }

    impl CreateOidcTokenRequest {
        pub fn builder() -> builder::CreateOidcTokenRequest {
            Default::default()
        }
    }

    ///CreateVolumeRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "compute": {
    ///      "$ref": "#/components/schemas/fly.MachineGuest"
    ///    },
    ///    "compute_image": {
    ///      "type": "string"
    ///    },
    ///    "encrypted": {
    ///      "type": "boolean"
    ///    },
    ///    "fstype": {
    ///      "type": "string"
    ///    },
    ///    "machines_only": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "require_unique_zone": {
    ///      "type": "boolean"
    ///    },
    ///    "size_gb": {
    ///      "type": "integer"
    ///    },
    ///    "snapshot_id": {
    ///      "description": "restore from snapshot",
    ///      "type": "string"
    ///    },
    ///    "snapshot_retention": {
    ///      "type": "integer"
    ///    },
    ///    "source_volume_id": {
    ///      "description": "fork from remote volume",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateVolumeRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compute: Option<FlyMachineGuest>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compute_image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fstype: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub machines_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_unique_zone: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size_gb: Option<i64>,
        ///restore from snapshot
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_retention: Option<i64>,
        ///fork from remote volume
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_volume_id: Option<String>,
    }

    impl From<&CreateVolumeRequest> for CreateVolumeRequest {
        fn from(value: &CreateVolumeRequest) -> Self {
            value.clone()
        }
    }

    impl CreateVolumeRequest {
        pub fn builder() -> builder::CreateVolumeRequest {
            Default::default()
        }
    }

    ///ErrorResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "details": {
    ///      "description": "Deprecated",
    ///      "type": "object"
    ///    },
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/main.statusCode"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ErrorResponse {
        ///Deprecated
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub details: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<MainStatusCode>,
    }

    impl From<&ErrorResponse> for ErrorResponse {
        fn from(value: &ErrorResponse) -> Self {
            value.clone()
        }
    }

    impl ErrorResponse {
        pub fn builder() -> builder::ErrorResponse {
            Default::default()
        }
    }

    ///ExtendVolumeRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "size_gb": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExtendVolumeRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size_gb: Option<i64>,
    }

    impl From<&ExtendVolumeRequest> for ExtendVolumeRequest {
        fn from(value: &ExtendVolumeRequest) -> Self {
            value.clone()
        }
    }

    impl ExtendVolumeRequest {
        pub fn builder() -> builder::ExtendVolumeRequest {
            Default::default()
        }
    }

    ///ExtendVolumeResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "needs_restart": {
    ///      "type": "boolean"
    ///    },
    ///    "volume": {
    ///      "$ref": "#/components/schemas/Volume"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExtendVolumeResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub needs_restart: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume: Option<Volume>,
    }

    impl From<&ExtendVolumeResponse> for ExtendVolumeResponse {
        fn from(value: &ExtendVolumeResponse) -> Self {
            value.clone()
        }
    }

    impl ExtendVolumeResponse {
        pub fn builder() -> builder::ExtendVolumeResponse {
            Default::default()
        }
    }

    ///FlyDnsConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "dns_forward_rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.dnsForwardRule"
    ///      }
    ///    },
    ///    "nameservers": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "options": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.dnsOption"
    ///      }
    ///    },
    ///    "searches": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "skip_registration": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyDnsConfig {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub dns_forward_rules: Vec<FlyDnsForwardRule>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub nameservers: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub options: Vec<FlyDnsOption>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub searches: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_registration: Option<bool>,
    }

    impl From<&FlyDnsConfig> for FlyDnsConfig {
        fn from(value: &FlyDnsConfig) -> Self {
            value.clone()
        }
    }

    impl FlyDnsConfig {
        pub fn builder() -> builder::FlyDnsConfig {
            Default::default()
        }
    }

    ///FlyDnsForwardRule
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "addr": {
    ///      "type": "string"
    ///    },
    ///    "basename": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyDnsForwardRule {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub addr: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub basename: Option<String>,
    }

    impl From<&FlyDnsForwardRule> for FlyDnsForwardRule {
        fn from(value: &FlyDnsForwardRule) -> Self {
            value.clone()
        }
    }

    impl FlyDnsForwardRule {
        pub fn builder() -> builder::FlyDnsForwardRule {
            Default::default()
        }
    }

    ///FlyDnsOption
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyDnsOption {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&FlyDnsOption> for FlyDnsOption {
        fn from(value: &FlyDnsOption) -> Self {
            value.clone()
        }
    }

    impl FlyDnsOption {
        pub fn builder() -> builder::FlyDnsOption {
            Default::default()
        }
    }

    ///FlyDuration
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "time.Duration": {
    ///      "type": "integer",
    ///      "x-enum-varnames": [
    ///        "minDuration",
    ///        "maxDuration",
    ///        "Nanosecond",
    ///        "Microsecond",
    ///        "Millisecond",
    ///        "Second",
    ///        "Minute",
    ///        "Hour",
    ///        "minDuration",
    ///        "maxDuration",
    ///        "Nanosecond",
    ///        "Microsecond",
    ///        "Millisecond",
    ///        "Second",
    ///        "Minute",
    ///        "Hour"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyDuration {
        #[serde(
            rename = "time.Duration",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_duration: Option<i64>,
    }

    impl From<&FlyDuration> for FlyDuration {
        fn from(value: &FlyDuration) -> Self {
            value.clone()
        }
    }

    impl FlyDuration {
        pub fn builder() -> builder::FlyDuration {
            Default::default()
        }
    }

    ///EnvVar defines an environment variable to be populated from a machine
    /// field, env_var
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "EnvVar defines an environment variable to be populated
    /// from a machine field, env_var",
    ///  "type": "object",
    ///  "properties": {
    ///    "env_var": {
    ///      "description": "EnvVar is required and is the name of the
    /// environment variable that will be set from the\nsecret. It must be a
    /// valid environment variable name.",
    ///      "type": "string"
    ///    },
    ///    "field_ref": {
    ///      "description": "FieldRef selects a field of the Machine: supports
    /// id, version, app_name, private_ip, region, image.",
    ///      "type": "string",
    ///      "enum": [
    ///        "id",
    ///        "version",
    ///        "app_name",
    ///        "private_ip",
    ///        "region",
    ///        "image"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyEnvFrom {
        ///EnvVar is required and is the name of the environment variable that
        /// will be set from the secret. It must be a valid environment
        /// variable name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub env_var: Option<String>,
        ///FieldRef selects a field of the Machine: supports id, version,
        /// app_name, private_ip, region, image.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub field_ref: Option<FlyEnvFromFieldRef>,
    }

    impl From<&FlyEnvFrom> for FlyEnvFrom {
        fn from(value: &FlyEnvFrom) -> Self {
            value.clone()
        }
    }

    impl FlyEnvFrom {
        pub fn builder() -> builder::FlyEnvFrom {
            Default::default()
        }
    }

    ///FieldRef selects a field of the Machine: supports id, version, app_name,
    /// private_ip, region, image.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "FieldRef selects a field of the Machine: supports id,
    /// version, app_name, private_ip, region, image.",
    ///  "type": "string",
    ///  "enum": [
    ///    "id",
    ///    "version",
    ///    "app_name",
    ///    "private_ip",
    ///    "region",
    ///    "image"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum FlyEnvFromFieldRef {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "app_name")]
        AppName,
        #[serde(rename = "private_ip")]
        PrivateIp,
        #[serde(rename = "region")]
        Region,
        #[serde(rename = "image")]
        Image,
    }

    impl From<&FlyEnvFromFieldRef> for FlyEnvFromFieldRef {
        fn from(value: &FlyEnvFromFieldRef) -> Self {
            value.clone()
        }
    }

    impl ToString for FlyEnvFromFieldRef {
        fn to_string(&self) -> String {
            match *self {
                Self::Id => "id".to_string(),
                Self::Version => "version".to_string(),
                Self::AppName => "app_name".to_string(),
                Self::PrivateIp => "private_ip".to_string(),
                Self::Region => "region".to_string(),
                Self::Image => "image".to_string(),
            }
        }
    }

    impl std::str::FromStr for FlyEnvFromFieldRef {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "id" => Ok(Self::Id),
                "version" => Ok(Self::Version),
                "app_name" => Ok(Self::AppName),
                "private_ip" => Ok(Self::PrivateIp),
                "region" => Ok(Self::Region),
                "image" => Ok(Self::Image),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FlyEnvFromFieldRef {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FlyEnvFromFieldRef {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FlyEnvFromFieldRef {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A file that will be written to the Machine. One of RawValue or
    /// SecretName must be set.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A file that will be written to the Machine. One of
    /// RawValue or SecretName must be set.",
    ///  "type": "object",
    ///  "properties": {
    ///    "guest_path": {
    ///      "description": "GuestPath is the path on the machine where the file
    /// will be written and must be an absolute path.\nFor example:
    /// /full/path/to/file.json",
    ///      "type": "string"
    ///    },
    ///    "raw_value": {
    ///      "description": "The base64 encoded string of the file contents.",
    ///      "type": "string"
    ///    },
    ///    "secret_name": {
    ///      "description": "The name of the secret that contains the base64
    /// encoded file contents.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyFile {
        ///GuestPath is the path on the machine where the file will be written
        /// and must be an absolute path. For example:
        /// /full/path/to/file.json
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub guest_path: Option<String>,
        ///The base64 encoded string of the file contents.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub raw_value: Option<String>,
        ///The name of the secret that contains the base64 encoded file
        /// contents.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret_name: Option<String>,
    }

    impl From<&FlyFile> for FlyFile {
        fn from(value: &FlyFile) -> Self {
            value.clone()
        }
    }

    impl FlyFile {
        pub fn builder() -> builder::FlyFile {
            Default::default()
        }
    }

    ///FlyHttpOptions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "compress": {
    ///      "type": "boolean"
    ///    },
    ///    "h2_backend": {
    ///      "type": "boolean"
    ///    },
    ///    "response": {
    ///      "$ref": "#/components/schemas/fly.HTTPResponseOptions"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyHttpOptions {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compress: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub h2_backend: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response: Option<FlyHttpResponseOptions>,
    }

    impl From<&FlyHttpOptions> for FlyHttpOptions {
        fn from(value: &FlyHttpOptions) -> Self {
            value.clone()
        }
    }

    impl FlyHttpOptions {
        pub fn builder() -> builder::FlyHttpOptions {
            Default::default()
        }
    }

    ///FlyHttpResponseOptions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "headers": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object"
    ///      }
    ///    },
    ///    "pristine": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyHttpResponseOptions {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub headers: std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pristine: Option<bool>,
    }

    impl From<&FlyHttpResponseOptions> for FlyHttpResponseOptions {
        fn from(value: &FlyHttpResponseOptions) -> Self {
            value.clone()
        }
    }

    impl FlyHttpResponseOptions {
        pub fn builder() -> builder::FlyHttpResponseOptions {
            Default::default()
        }
    }

    ///An optional object that defines one or more named checks. The key for
    /// each check is the check name.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "An optional object that defines one or more named
    /// checks. The key for each check is the check name.",
    ///  "type": "object",
    ///  "properties": {
    ///    "grace_period": {
    ///      "description": "The time to wait after a VM starts before checking
    /// its health",
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/fly.Duration"
    ///        }
    ///      ]
    ///    },
    ///    "headers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachineHTTPHeader"
    ///      }
    ///    },
    ///    "interval": {
    ///      "description": "The time between connectivity checks",
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/fly.Duration"
    ///        }
    ///      ]
    ///    },
    ///    "method": {
    ///      "description": "For http checks, the HTTP method to use to when
    /// making the request",
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "description": "For http checks, the path to send the request to",
    ///      "type": "string"
    ///    },
    ///    "port": {
    ///      "description": "The port to connect to, often the same as
    /// internal_port",
    ///      "type": "integer"
    ///    },
    ///    "protocol": {
    ///      "description": "For http checks, whether to use http or https",
    ///      "type": "string"
    ///    },
    ///    "timeout": {
    ///      "description": "The maximum time a connection can take before being
    /// reported as failing its health check",
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/fly.Duration"
    ///        }
    ///      ]
    ///    },
    ///    "tls_server_name": {
    ///      "description": "If the protocol is https, the hostname to use for
    /// TLS certificate validation",
    ///      "type": "string"
    ///    },
    ///    "tls_skip_verify": {
    ///      "description": "For http checks with https protocol, whether or not
    /// to verify the TLS certificate",
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "description": "tcp or http",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineCheck {
        ///The time to wait after a VM starts before checking its health
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub grace_period: Option<FlyDuration>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub headers: Vec<FlyMachineHttpHeader>,
        ///The time between connectivity checks
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<FlyDuration>,
        ///For http checks, the HTTP method to use to when making the request
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,
        ///For http checks, the path to send the request to
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        ///The port to connect to, often the same as internal_port
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
        ///For http checks, whether to use http or https
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        ///The maximum time a connection can take before being reported as
        /// failing its health check
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<FlyDuration>,
        ///If the protocol is https, the hostname to use for TLS certificate
        /// validation
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tls_server_name: Option<String>,
        ///For http checks with https protocol, whether or not to verify the
        /// TLS certificate
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tls_skip_verify: Option<bool>,
        ///tcp or http
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&FlyMachineCheck> for FlyMachineCheck {
        fn from(value: &FlyMachineCheck) -> Self {
            value.clone()
        }
    }

    impl FlyMachineCheck {
        pub fn builder() -> builder::FlyMachineCheck {
            Default::default()
        }
    }

    ///FlyMachineConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "auto_destroy": {
    ///      "description": "Optional boolean telling the Machine to destroy
    /// itself once it’s complete (default false)",
    ///      "type": "boolean"
    ///    },
    ///    "checks": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/fly.MachineCheck"
    ///      }
    ///    },
    ///    "disable_machine_autostart": {
    ///      "description": "Deprecated: use Service.Autostart instead",
    ///      "type": "boolean"
    ///    },
    ///    "dns": {
    ///      "$ref": "#/components/schemas/fly.DNSConfig"
    ///    },
    ///    "env": {
    ///      "description": "An object filled with key/value pairs to be set as
    /// environment variables",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "files": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.File"
    ///      }
    ///    },
    ///    "guest": {
    ///      "$ref": "#/components/schemas/fly.MachineGuest"
    ///    },
    ///    "image": {
    ///      "description": "The docker image to run",
    ///      "type": "string"
    ///    },
    ///    "init": {
    ///      "$ref": "#/components/schemas/fly.MachineInit"
    ///    },
    ///    "metadata": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "metrics": {
    ///      "$ref": "#/components/schemas/fly.MachineMetrics"
    ///    },
    ///    "mounts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachineMount"
    ///      }
    ///    },
    ///    "processes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachineProcess"
    ///      }
    ///    },
    ///    "restart": {
    ///      "$ref": "#/components/schemas/fly.MachineRestart"
    ///    },
    ///    "schedule": {
    ///      "type": "string"
    ///    },
    ///    "services": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachineService"
    ///      }
    ///    },
    ///    "size": {
    ///      "description": "Deprecated: use Guest instead",
    ///      "type": "string"
    ///    },
    ///    "standbys": {
    ///      "description": "Standbys enable a machine to be a standby for
    /// another. In the event of a hardware failure,\nthe standby machine will
    /// be started.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "statics": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.Static"
    ///      }
    ///    },
    ///    "stop_config": {
    ///      "$ref": "#/components/schemas/fly.StopConfig"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineConfig {
        ///Optional boolean telling the Machine to destroy itself once it’s
        /// complete (default false)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auto_destroy: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub checks: std::collections::HashMap<String, FlyMachineCheck>,
        ///Deprecated: use Service.Autostart instead
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disable_machine_autostart: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dns: Option<FlyDnsConfig>,
        ///An object filled with key/value pairs to be set as environment
        /// variables
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub env: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub files: Vec<FlyFile>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub guest: Option<FlyMachineGuest>,
        ///The docker image to run
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub init: Option<FlyMachineInit>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub metadata: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metrics: Option<FlyMachineMetrics>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub mounts: Vec<FlyMachineMount>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub processes: Vec<FlyMachineProcess>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub restart: Option<FlyMachineRestart>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub schedule: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub services: Vec<FlyMachineService>,
        ///Deprecated: use Guest instead
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
        ///Standbys enable a machine to be a standby for another. In the event
        /// of a hardware failure, the standby machine will be started.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub standbys: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub statics: Vec<FlyStatic>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stop_config: Option<FlyStopConfig>,
    }

    impl From<&FlyMachineConfig> for FlyMachineConfig {
        fn from(value: &FlyMachineConfig) -> Self {
            value.clone()
        }
    }

    impl FlyMachineConfig {
        pub fn builder() -> builder::FlyMachineConfig {
            Default::default()
        }
    }

    ///FlyMachineGuest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cpu_kind": {
    ///      "type": "string"
    ///    },
    ///    "cpus": {
    ///      "type": "integer"
    ///    },
    ///    "gpu_kind": {
    ///      "type": "string"
    ///    },
    ///    "gpus": {
    ///      "type": "integer"
    ///    },
    ///    "host_dedication_id": {
    ///      "type": "string"
    ///    },
    ///    "kernel_args": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "memory_mb": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineGuest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpu_kind: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpus: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gpu_kind: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gpus: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host_dedication_id: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub kernel_args: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub memory_mb: Option<i64>,
    }

    impl From<&FlyMachineGuest> for FlyMachineGuest {
        fn from(value: &FlyMachineGuest) -> Self {
            value.clone()
        }
    }

    impl FlyMachineGuest {
        pub fn builder() -> builder::FlyMachineGuest {
            Default::default()
        }
    }

    ///For http checks, an array of objects with string field Name and array of
    /// strings field Values. The key/value pairs specify header and header
    /// values that will get passed with the check call.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "For http checks, an array of objects with string field
    /// Name and array of strings field Values. The key/value pairs specify
    /// header and header values that will get passed with the check call.",
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "description": "The header name",
    ///      "type": "string"
    ///    },
    ///    "values": {
    ///      "description": "The header value",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineHttpHeader {
        ///The header name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///The header value
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub values: Vec<String>,
    }

    impl From<&FlyMachineHttpHeader> for FlyMachineHttpHeader {
        fn from(value: &FlyMachineHttpHeader) -> Self {
            value.clone()
        }
    }

    impl FlyMachineHttpHeader {
        pub fn builder() -> builder::FlyMachineHttpHeader {
            Default::default()
        }
    }

    ///FlyMachineInit
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cmd": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "entrypoint": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "exec": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "kernel_args": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "swap_size_mb": {
    ///      "type": "integer"
    ///    },
    ///    "tty": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineInit {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub cmd: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub entrypoint: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub exec: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub kernel_args: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub swap_size_mb: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tty: Option<bool>,
    }

    impl From<&FlyMachineInit> for FlyMachineInit {
        fn from(value: &FlyMachineInit) -> Self {
            value.clone()
        }
    }

    impl FlyMachineInit {
        pub fn builder() -> builder::FlyMachineInit {
            Default::default()
        }
    }

    ///FlyMachineMetrics
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "port": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineMetrics {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
    }

    impl From<&FlyMachineMetrics> for FlyMachineMetrics {
        fn from(value: &FlyMachineMetrics) -> Self {
            value.clone()
        }
    }

    impl FlyMachineMetrics {
        pub fn builder() -> builder::FlyMachineMetrics {
            Default::default()
        }
    }

    ///FlyMachineMount
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "add_size_gb": {
    ///      "type": "integer"
    ///    },
    ///    "encrypted": {
    ///      "type": "boolean"
    ///    },
    ///    "extend_threshold_percent": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "size_gb": {
    ///      "type": "integer"
    ///    },
    ///    "size_gb_limit": {
    ///      "type": "integer"
    ///    },
    ///    "volume": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineMount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub add_size_gb: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extend_threshold_percent: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size_gb: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size_gb_limit: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume: Option<String>,
    }

    impl From<&FlyMachineMount> for FlyMachineMount {
        fn from(value: &FlyMachineMount) -> Self {
            value.clone()
        }
    }

    impl FlyMachineMount {
        pub fn builder() -> builder::FlyMachineMount {
            Default::default()
        }
    }

    ///FlyMachinePort
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "end_port": {
    ///      "type": "integer"
    ///    },
    ///    "force_https": {
    ///      "type": "boolean"
    ///    },
    ///    "handlers": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "http_options": {
    ///      "$ref": "#/components/schemas/fly.HTTPOptions"
    ///    },
    ///    "port": {
    ///      "type": "integer"
    ///    },
    ///    "proxy_proto_options": {
    ///      "$ref": "#/components/schemas/fly.ProxyProtoOptions"
    ///    },
    ///    "start_port": {
    ///      "type": "integer"
    ///    },
    ///    "tls_options": {
    ///      "$ref": "#/components/schemas/fly.TLSOptions"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachinePort {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub end_port: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub force_https: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub handlers: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_options: Option<FlyHttpOptions>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub proxy_proto_options: Option<FlyProxyProtoOptions>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start_port: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tls_options: Option<FlyTlsOptions>,
    }

    impl From<&FlyMachinePort> for FlyMachinePort {
        fn from(value: &FlyMachinePort) -> Self {
            value.clone()
        }
    }

    impl FlyMachinePort {
        pub fn builder() -> builder::FlyMachinePort {
            Default::default()
        }
    }

    ///FlyMachineProcess
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cmd": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "entrypoint": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "env": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "env_from": {
    ///      "description": "EnvFrom can be provided to set environment
    /// variables from machine fields.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.EnvFrom"
    ///      }
    ///    },
    ///    "exec": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ignore_app_secrets": {
    ///      "description": "IgnoreAppSecrets can be set to true to ignore the
    /// secrets for the App the Machine belongs to\nand only use the secrets
    /// provided at the process level. The default/legacy behavior is to
    /// use\nthe secrets provided at the App level.",
    ///      "type": "boolean"
    ///    },
    ///    "secrets": {
    ///      "description": "Secrets can be provided at the process level to explicitly indicate which secrets should be\nused for the process. If not provided, the secrets provided at the machine level will be used.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachineSecret"
    ///      }
    ///    },
    ///    "user": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineProcess {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub cmd: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub entrypoint: Vec<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub env: std::collections::HashMap<String, String>,
        ///EnvFrom can be provided to set environment variables from machine
        /// fields.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub env_from: Vec<FlyEnvFrom>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub exec: Vec<String>,
        ///IgnoreAppSecrets can be set to true to ignore the secrets for the
        /// App the Machine belongs to and only use the secrets provided
        /// at the process level. The default/legacy behavior is to use
        /// the secrets provided at the App level.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ignore_app_secrets: Option<bool>,
        ///Secrets can be provided at the process level to explicitly indicate
        /// which secrets should be used for the process. If not
        /// provided, the secrets provided at the machine level will be used.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub secrets: Vec<FlyMachineSecret>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&FlyMachineProcess> for FlyMachineProcess {
        fn from(value: &FlyMachineProcess) -> Self {
            value.clone()
        }
    }

    impl FlyMachineProcess {
        pub fn builder() -> builder::FlyMachineProcess {
            Default::default()
        }
    }

    ///The Machine restart policy defines whether and how flyd restarts a Machine after its main process exits. See https://fly.io/docs/machines/guides-examples/machine-restart-policy/.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The Machine restart policy defines whether and how flyd restarts a Machine after its main process exits. See https://fly.io/docs/machines/guides-examples/machine-restart-policy/.",
    ///  "type": "object",
    ///  "properties": {
    ///    "max_retries": {
    ///      "description": "When policy is on-failure, the maximum number of
    /// times to attempt to restart the Machine before letting it stop.",
    ///      "type": "integer"
    ///    },
    ///    "policy": {
    ///      "description": "* no - Never try to restart a Machine automatically
    /// when its main process exits, whether that’s on purpose or on a crash.\n*
    /// always - Always restart a Machine automatically and never let it enter a
    /// stopped state, even when the main process exits cleanly.\n* on-failure -
    /// Try up to MaxRetries times to automatically restart the Machine if it
    /// exits with a non-zero exit code. Default when no explicit policy is set,
    /// and for Machines with schedules.",
    ///      "type": "string",
    ///      "enum": [
    ///        "no",
    ///        "always",
    ///        "on-failure"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineRestart {
        ///When policy is on-failure, the maximum number of times to attempt to
        /// restart the Machine before letting it stop.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_retries: Option<i64>,
        /// * no - Never try to restart a Machine automatically when its main
        ///   process exits, whether that’s on purpose or on a crash.
        /// * always - Always restart a Machine automatically and never let it
        ///   enter a stopped state, even when the main process exits cleanly.
        /// * on-failure - Try up to MaxRetries times to automatically restart
        ///   the Machine if it exits with a non-zero exit code. Default when no
        ///   explicit policy is set, and for Machines with schedules.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy: Option<FlyMachineRestartPolicy>,
    }

    impl From<&FlyMachineRestart> for FlyMachineRestart {
        fn from(value: &FlyMachineRestart) -> Self {
            value.clone()
        }
    }

    impl FlyMachineRestart {
        pub fn builder() -> builder::FlyMachineRestart {
            Default::default()
        }
    }

    /// * no - Never try to restart a Machine automatically when its main
    ///   process exits, whether that’s on purpose or on a crash.
    /// * always - Always restart a Machine automatically and never let it enter
    ///   a stopped state, even when the main process exits cleanly.
    /// * on-failure - Try up to MaxRetries times to automatically restart the
    ///   Machine if it exits with a non-zero exit code. Default when no
    ///   explicit policy is set, and for Machines with schedules.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "* no - Never try to restart a Machine automatically
    /// when its main process exits, whether that’s on purpose or on a crash.\n*
    /// always - Always restart a Machine automatically and never let it enter a
    /// stopped state, even when the main process exits cleanly.\n* on-failure -
    /// Try up to MaxRetries times to automatically restart the Machine if it
    /// exits with a non-zero exit code. Default when no explicit policy is set,
    /// and for Machines with schedules.",
    ///  "type": "string",
    ///  "enum": [
    ///    "no",
    ///    "always",
    ///    "on-failure"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum FlyMachineRestartPolicy {
        #[serde(rename = "no")]
        No,
        #[serde(rename = "always")]
        Always,
        #[serde(rename = "on-failure")]
        OnFailure,
    }

    impl From<&FlyMachineRestartPolicy> for FlyMachineRestartPolicy {
        fn from(value: &FlyMachineRestartPolicy) -> Self {
            value.clone()
        }
    }

    impl ToString for FlyMachineRestartPolicy {
        fn to_string(&self) -> String {
            match *self {
                Self::No => "no".to_string(),
                Self::Always => "always".to_string(),
                Self::OnFailure => "on-failure".to_string(),
            }
        }
    }

    impl std::str::FromStr for FlyMachineRestartPolicy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "no" => Ok(Self::No),
                "always" => Ok(Self::Always),
                "on-failure" => Ok(Self::OnFailure),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FlyMachineRestartPolicy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FlyMachineRestartPolicy {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FlyMachineRestartPolicy {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A Secret needing to be set in the environment of the Machine. env_var is
    /// required
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Secret needing to be set in the environment of the
    /// Machine. env_var is required",
    ///  "type": "object",
    ///  "properties": {
    ///    "env_var": {
    ///      "description": "EnvVar is required and is the name of the
    /// environment variable that will be set from the\nsecret. It must be a
    /// valid environment variable name.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Name is optional and when provided is used to
    /// reference a secret name where the EnvVar is\ndifferent from what was set
    /// as the secret name.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineSecret {
        ///EnvVar is required and is the name of the environment variable that
        /// will be set from the secret. It must be a valid environment
        /// variable name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub env_var: Option<String>,
        ///Name is optional and when provided is used to reference a secret
        /// name where the EnvVar is different from what was set as the
        /// secret name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&FlyMachineSecret> for FlyMachineSecret {
        fn from(value: &FlyMachineSecret) -> Self {
            value.clone()
        }
    }

    impl FlyMachineSecret {
        pub fn builder() -> builder::FlyMachineSecret {
            Default::default()
        }
    }

    ///FlyMachineService
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "autostart": {
    ///      "type": "boolean"
    ///    },
    ///    "autostop": {
    ///      "type": "boolean"
    ///    },
    ///    "checks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachineCheck"
    ///      }
    ///    },
    ///    "concurrency": {
    ///      "$ref": "#/components/schemas/fly.MachineServiceConcurrency"
    ///    },
    ///    "force_instance_description": {
    ///      "type": "string"
    ///    },
    ///    "force_instance_key": {
    ///      "type": "string"
    ///    },
    ///    "internal_port": {
    ///      "type": "integer"
    ///    },
    ///    "min_machines_running": {
    ///      "type": "integer"
    ///    },
    ///    "ports": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/fly.MachinePort"
    ///      }
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineService {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub autostart: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub autostop: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub checks: Vec<FlyMachineCheck>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub concurrency: Option<FlyMachineServiceConcurrency>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub force_instance_description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub force_instance_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub internal_port: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_machines_running: Option<i64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ports: Vec<FlyMachinePort>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
    }

    impl From<&FlyMachineService> for FlyMachineService {
        fn from(value: &FlyMachineService) -> Self {
            value.clone()
        }
    }

    impl FlyMachineService {
        pub fn builder() -> builder::FlyMachineService {
            Default::default()
        }
    }

    ///FlyMachineServiceConcurrency
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "hard_limit": {
    ///      "type": "integer"
    ///    },
    ///    "soft_limit": {
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyMachineServiceConcurrency {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hard_limit: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub soft_limit: Option<i64>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&FlyMachineServiceConcurrency> for FlyMachineServiceConcurrency {
        fn from(value: &FlyMachineServiceConcurrency) -> Self {
            value.clone()
        }
    }

    impl FlyMachineServiceConcurrency {
        pub fn builder() -> builder::FlyMachineServiceConcurrency {
            Default::default()
        }
    }

    ///FlyProxyProtoOptions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyProxyProtoOptions {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&FlyProxyProtoOptions> for FlyProxyProtoOptions {
        fn from(value: &FlyProxyProtoOptions) -> Self {
            value.clone()
        }
    }

    impl FlyProxyProtoOptions {
        pub fn builder() -> builder::FlyProxyProtoOptions {
            Default::default()
        }
    }

    ///FlyStatic
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "guest_path",
    ///    "url_prefix"
    ///  ],
    ///  "properties": {
    ///    "guest_path": {
    ///      "type": "string"
    ///    },
    ///    "tigris_bucket": {
    ///      "type": "string"
    ///    },
    ///    "url_prefix": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyStatic {
        pub guest_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tigris_bucket: Option<String>,
        pub url_prefix: String,
    }

    impl From<&FlyStatic> for FlyStatic {
        fn from(value: &FlyStatic) -> Self {
            value.clone()
        }
    }

    impl FlyStatic {
        pub fn builder() -> builder::FlyStatic {
            Default::default()
        }
    }

    ///FlyStopConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "signal": {
    ///      "type": "string"
    ///    },
    ///    "timeout": {
    ///      "$ref": "#/components/schemas/fly.Duration"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyStopConfig {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signal: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<FlyDuration>,
    }

    impl From<&FlyStopConfig> for FlyStopConfig {
        fn from(value: &FlyStopConfig) -> Self {
            value.clone()
        }
    }

    impl FlyStopConfig {
        pub fn builder() -> builder::FlyStopConfig {
            Default::default()
        }
    }

    ///FlyTlsOptions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "alpn": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "default_self_signed": {
    ///      "type": "boolean"
    ///    },
    ///    "versions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlyTlsOptions {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub alpn: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_self_signed: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub versions: Vec<String>,
    }

    impl From<&FlyTlsOptions> for FlyTlsOptions {
        fn from(value: &FlyTlsOptions) -> Self {
            value.clone()
        }
    }

    impl FlyTlsOptions {
        pub fn builder() -> builder::FlyTlsOptions {
            Default::default()
        }
    }

    ///ImageRef
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "digest": {
    ///      "type": "string"
    ///    },
    ///    "labels": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "registry": {
    ///      "type": "string"
    ///    },
    ///    "repository": {
    ///      "type": "string"
    ///    },
    ///    "tag": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ImageRef {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub digest: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub labels: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub registry: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub repository: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    impl From<&ImageRef> for ImageRef {
        fn from(value: &ImageRef) -> Self {
            value.clone()
        }
    }

    impl ImageRef {
        pub fn builder() -> builder::ImageRef {
            Default::default()
        }
    }

    ///Lease
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "description": "Description or reason for the Lease.",
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "description": "ExpiresAt is the unix timestamp in UTC to denote
    /// when the Lease will no longer be valid.",
    ///      "type": "integer"
    ///    },
    ///    "nonce": {
    ///      "description": "Nonce is the unique ID autogenerated and associated
    /// with the Lease.",
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "description": "Owner is the user identifier which acquired the
    /// Lease.",
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "description": "Machine version",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Lease {
        ///Description or reason for the Lease.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///ExpiresAt is the unix timestamp in UTC to denote when the Lease will
        /// no longer be valid.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expires_at: Option<i64>,
        ///Nonce is the unique ID autogenerated and associated with the Lease.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        ///Owner is the user identifier which acquired the Lease.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<String>,
        ///Machine version
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&Lease> for Lease {
        fn from(value: &Lease) -> Self {
            value.clone()
        }
    }

    impl Lease {
        pub fn builder() -> builder::Lease {
            Default::default()
        }
    }

    ///ListApp
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "machine_count": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "network": {
    ///      "type": "object"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListApp {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub machine_count: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub network: serde_json::Map<String, serde_json::Value>,
    }

    impl From<&ListApp> for ListApp {
        fn from(value: &ListApp) -> Self {
            value.clone()
        }
    }

    impl ListApp {
        pub fn builder() -> builder::ListApp {
            Default::default()
        }
    }

    ///ListAppsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "apps": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ListApp"
    ///      }
    ///    },
    ///    "total_apps": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListAppsResponse {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub apps: Vec<ListApp>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub total_apps: Option<i64>,
    }

    impl From<&ListAppsResponse> for ListAppsResponse {
        fn from(value: &ListAppsResponse) -> Self {
            value.clone()
        }
    }

    impl ListAppsResponse {
        pub fn builder() -> builder::ListAppsResponse {
            Default::default()
        }
    }

    ///ListenSocket
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "address": {
    ///      "type": "string"
    ///    },
    ///    "proto": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListenSocket {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub proto: Option<String>,
    }

    impl From<&ListenSocket> for ListenSocket {
        fn from(value: &ListenSocket) -> Self {
            value.clone()
        }
    }

    impl ListenSocket {
        pub fn builder() -> builder::ListenSocket {
            Default::default()
        }
    }

    ///Machine
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "checks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CheckStatus"
    ///      }
    ///    },
    ///    "config": {
    ///      "$ref": "#/components/schemas/fly.MachineConfig"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "events": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MachineEvent"
    ///      }
    ///    },
    ///    "host_status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ok",
    ///        "unknown",
    ///        "unreachable"
    ///      ]
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "image_ref": {
    ///      "$ref": "#/components/schemas/ImageRef"
    ///    },
    ///    "instance_id": {
    ///      "description": "InstanceID is unique for each version of the
    /// machine",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nonce": {
    ///      "description": "Nonce is only every returned on machine creation if
    /// a lease_duration was provided.",
    ///      "type": "string"
    ///    },
    ///    "private_ip": {
    ///      "description": "PrivateIP is the internal 6PN address of the
    /// machine.",
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Machine {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub checks: Vec<CheckStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<FlyMachineConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<MachineEvent>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host_status: Option<MachineHostStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image_ref: Option<ImageRef>,
        ///InstanceID is unique for each version of the machine
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Nonce is only every returned on machine creation if a lease_duration
        /// was provided.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        ///PrivateIP is the internal 6PN address of the machine.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private_ip: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
    }

    impl From<&Machine> for Machine {
        fn from(value: &Machine) -> Self {
            value.clone()
        }
    }

    impl Machine {
        pub fn builder() -> builder::Machine {
            Default::default()
        }
    }

    ///MachineEvent
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "request": {
    ///      "type": "object"
    ///    },
    ///    "source": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "timestamp": {
    ///      "type": "integer"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MachineEvent {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub request: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&MachineEvent> for MachineEvent {
        fn from(value: &MachineEvent) -> Self {
            value.clone()
        }
    }

    impl MachineEvent {
        pub fn builder() -> builder::MachineEvent {
            Default::default()
        }
    }

    ///MachineExecRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cmd": {
    ///      "description": "Deprecated: use Command instead",
    ///      "type": "string"
    ///    },
    ///    "command": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "timeout": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MachineExecRequest {
        ///Deprecated: use Command instead
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cmd: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub command: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<i64>,
    }

    impl From<&MachineExecRequest> for MachineExecRequest {
        fn from(value: &MachineExecRequest) -> Self {
            value.clone()
        }
    }

    impl MachineExecRequest {
        pub fn builder() -> builder::MachineExecRequest {
            Default::default()
        }
    }

    ///MachineHostStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ok",
    ///    "unknown",
    ///    "unreachable"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum MachineHostStatus {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "unreachable")]
        Unreachable,
    }

    impl From<&MachineHostStatus> for MachineHostStatus {
        fn from(value: &MachineHostStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for MachineHostStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Ok => "ok".to_string(),
                Self::Unknown => "unknown".to_string(),
                Self::Unreachable => "unreachable".to_string(),
            }
        }
    }

    impl std::str::FromStr for MachineHostStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                "unknown" => Ok(Self::Unknown),
                "unreachable" => Ok(Self::Unreachable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for MachineHostStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for MachineHostStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for MachineHostStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MachineVersion
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "user_config": {
    ///      "$ref": "#/components/schemas/fly.MachineConfig"
    ///    },
    ///    "version": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MachineVersion {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user_config: Option<FlyMachineConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&MachineVersion> for MachineVersion {
        fn from(value: &MachineVersion) -> Self {
            value.clone()
        }
    }

    impl MachineVersion {
        pub fn builder() -> builder::MachineVersion {
            Default::default()
        }
    }

    ///MachinesWaitState
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "started",
    ///    "stopped",
    ///    "destroyed"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum MachinesWaitState {
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "stopped")]
        Stopped,
        #[serde(rename = "destroyed")]
        Destroyed,
    }

    impl From<&MachinesWaitState> for MachinesWaitState {
        fn from(value: &MachinesWaitState) -> Self {
            value.clone()
        }
    }

    impl ToString for MachinesWaitState {
        fn to_string(&self) -> String {
            match *self {
                Self::Started => "started".to_string(),
                Self::Stopped => "stopped".to_string(),
                Self::Destroyed => "destroyed".to_string(),
            }
        }
    }

    impl std::str::FromStr for MachinesWaitState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "started" => Ok(Self::Started),
                "stopped" => Ok(Self::Stopped),
                "destroyed" => Ok(Self::Destroyed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for MachinesWaitState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for MachinesWaitState {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for MachinesWaitState {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MainStatusCode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "unknown",
    ///    "insufficient_capacity"
    ///  ],
    ///  "x-enum-varnames": [
    ///    "unknown",
    ///    "capacityErr"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum MainStatusCode {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "insufficient_capacity")]
        InsufficientCapacity,
    }

    impl From<&MainStatusCode> for MainStatusCode {
        fn from(value: &MainStatusCode) -> Self {
            value.clone()
        }
    }

    impl ToString for MainStatusCode {
        fn to_string(&self) -> String {
            match *self {
                Self::Unknown => "unknown".to_string(),
                Self::InsufficientCapacity => "insufficient_capacity".to_string(),
            }
        }
    }

    impl std::str::FromStr for MainStatusCode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "unknown" => Ok(Self::Unknown),
                "insufficient_capacity" => Ok(Self::InsufficientCapacity),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for MainStatusCode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for MainStatusCode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for MainStatusCode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Organization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "slug": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Organization {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slug: Option<String>,
    }

    impl From<&Organization> for Organization {
        fn from(value: &Organization) -> Self {
            value.clone()
        }
    }

    impl Organization {
        pub fn builder() -> builder::Organization {
            Default::default()
        }
    }

    ///ProcessStat
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "command": {
    ///      "type": "string"
    ///    },
    ///    "cpu": {
    ///      "type": "integer"
    ///    },
    ///    "directory": {
    ///      "type": "string"
    ///    },
    ///    "listen_sockets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ListenSocket"
    ///      }
    ///    },
    ///    "pid": {
    ///      "type": "integer"
    ///    },
    ///    "rss": {
    ///      "type": "integer"
    ///    },
    ///    "rtime": {
    ///      "type": "integer"
    ///    },
    ///    "stime": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProcessStat {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub command: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub directory: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub listen_sockets: Vec<ListenSocket>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pid: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rss: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rtime: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stime: Option<i64>,
    }

    impl From<&ProcessStat> for ProcessStat {
        fn from(value: &ProcessStat) -> Self {
            value.clone()
        }
    }

    impl ProcessStat {
        pub fn builder() -> builder::ProcessStat {
            Default::default()
        }
    }

    ///SignalRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "signal": {
    ///      "type": "string",
    ///      "enum": [
    ///        "SIGABRT",
    ///        "SIGALRM",
    ///        "SIGFPE",
    ///        "SIGHUP",
    ///        "SIGILL",
    ///        "SIGINT",
    ///        "SIGKILL",
    ///        "SIGPIPE",
    ///        "SIGQUIT",
    ///        "SIGSEGV",
    ///        "SIGTERM",
    ///        "SIGTRAP",
    ///        "SIGUSR1"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SignalRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signal: Option<SignalRequestSignal>,
    }

    impl From<&SignalRequest> for SignalRequest {
        fn from(value: &SignalRequest) -> Self {
            value.clone()
        }
    }

    impl SignalRequest {
        pub fn builder() -> builder::SignalRequest {
            Default::default()
        }
    }

    ///SignalRequestSignal
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SIGABRT",
    ///    "SIGALRM",
    ///    "SIGFPE",
    ///    "SIGHUP",
    ///    "SIGILL",
    ///    "SIGINT",
    ///    "SIGKILL",
    ///    "SIGPIPE",
    ///    "SIGQUIT",
    ///    "SIGSEGV",
    ///    "SIGTERM",
    ///    "SIGTRAP",
    ///    "SIGUSR1"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum SignalRequestSignal {
        #[serde(rename = "SIGABRT")]
        Sigabrt,
        #[serde(rename = "SIGALRM")]
        Sigalrm,
        #[serde(rename = "SIGFPE")]
        Sigfpe,
        #[serde(rename = "SIGHUP")]
        Sighup,
        #[serde(rename = "SIGILL")]
        Sigill,
        #[serde(rename = "SIGINT")]
        Sigint,
        #[serde(rename = "SIGKILL")]
        Sigkill,
        #[serde(rename = "SIGPIPE")]
        Sigpipe,
        #[serde(rename = "SIGQUIT")]
        Sigquit,
        #[serde(rename = "SIGSEGV")]
        Sigsegv,
        #[serde(rename = "SIGTERM")]
        Sigterm,
        #[serde(rename = "SIGTRAP")]
        Sigtrap,
        #[serde(rename = "SIGUSR1")]
        Sigusr1,
    }

    impl From<&SignalRequestSignal> for SignalRequestSignal {
        fn from(value: &SignalRequestSignal) -> Self {
            value.clone()
        }
    }

    impl ToString for SignalRequestSignal {
        fn to_string(&self) -> String {
            match *self {
                Self::Sigabrt => "SIGABRT".to_string(),
                Self::Sigalrm => "SIGALRM".to_string(),
                Self::Sigfpe => "SIGFPE".to_string(),
                Self::Sighup => "SIGHUP".to_string(),
                Self::Sigill => "SIGILL".to_string(),
                Self::Sigint => "SIGINT".to_string(),
                Self::Sigkill => "SIGKILL".to_string(),
                Self::Sigpipe => "SIGPIPE".to_string(),
                Self::Sigquit => "SIGQUIT".to_string(),
                Self::Sigsegv => "SIGSEGV".to_string(),
                Self::Sigterm => "SIGTERM".to_string(),
                Self::Sigtrap => "SIGTRAP".to_string(),
                Self::Sigusr1 => "SIGUSR1".to_string(),
            }
        }
    }

    impl std::str::FromStr for SignalRequestSignal {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "SIGABRT" => Ok(Self::Sigabrt),
                "SIGALRM" => Ok(Self::Sigalrm),
                "SIGFPE" => Ok(Self::Sigfpe),
                "SIGHUP" => Ok(Self::Sighup),
                "SIGILL" => Ok(Self::Sigill),
                "SIGINT" => Ok(Self::Sigint),
                "SIGKILL" => Ok(Self::Sigkill),
                "SIGPIPE" => Ok(Self::Sigpipe),
                "SIGQUIT" => Ok(Self::Sigquit),
                "SIGSEGV" => Ok(Self::Sigsegv),
                "SIGTERM" => Ok(Self::Sigterm),
                "SIGTRAP" => Ok(Self::Sigtrap),
                "SIGUSR1" => Ok(Self::Sigusr1),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for SignalRequestSignal {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SignalRequestSignal {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SignalRequestSignal {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///StopRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "signal": {
    ///      "type": "string"
    ///    },
    ///    "timeout": {
    ///      "$ref": "#/components/schemas/fly.Duration"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StopRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signal: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<FlyDuration>,
    }

    impl From<&StopRequest> for StopRequest {
        fn from(value: &StopRequest) -> Self {
            value.clone()
        }
    }

    impl StopRequest {
        pub fn builder() -> builder::StopRequest {
            Default::default()
        }
    }

    ///UpdateMachineRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "description": "An object defining the Machine configuration",
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/fly.MachineConfig"
    ///        }
    ///      ]
    ///    },
    ///    "current_version": {
    ///      "type": "string"
    ///    },
    ///    "lease_ttl": {
    ///      "type": "integer"
    ///    },
    ///    "lsvd": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "description": "Unique name for this Machine. If omitted, one is
    /// generated for you",
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "description": "The target region. Omitting this param launches in
    /// the same region as your WireGuard peer connection (somewhere near
    /// you).",
    ///      "type": "string"
    ///    },
    ///    "skip_launch": {
    ///      "type": "boolean"
    ///    },
    ///    "skip_service_registration": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateMachineRequest {
        ///An object defining the Machine configuration
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<FlyMachineConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub current_version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lease_ttl: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lsvd: Option<bool>,
        ///Unique name for this Machine. If omitted, one is generated for you
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///The target region. Omitting this param launches in the same region
        /// as your WireGuard peer connection (somewhere near you).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_launch: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_service_registration: Option<bool>,
    }

    impl From<&UpdateMachineRequest> for UpdateMachineRequest {
        fn from(value: &UpdateMachineRequest) -> Self {
            value.clone()
        }
    }

    impl UpdateMachineRequest {
        pub fn builder() -> builder::UpdateMachineRequest {
            Default::default()
        }
    }

    ///UpdateVolumeRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "auto_backup_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "snapshot_retention": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateVolumeRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auto_backup_enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_retention: Option<i64>,
    }

    impl From<&UpdateVolumeRequest> for UpdateVolumeRequest {
        fn from(value: &UpdateVolumeRequest) -> Self {
            value.clone()
        }
    }

    impl UpdateVolumeRequest {
        pub fn builder() -> builder::UpdateVolumeRequest {
            Default::default()
        }
    }

    ///Volume
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attached_alloc_id": {
    ///      "type": "string"
    ///    },
    ///    "attached_machine_id": {
    ///      "type": "string"
    ///    },
    ///    "auto_backup_enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "block_size": {
    ///      "type": "integer"
    ///    },
    ///    "blocks": {
    ///      "type": "integer"
    ///    },
    ///    "blocks_avail": {
    ///      "type": "integer"
    ///    },
    ///    "blocks_free": {
    ///      "type": "integer"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "encrypted": {
    ///      "type": "boolean"
    ///    },
    ///    "fstype": {
    ///      "type": "string"
    ///    },
    ///    "host_status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ok",
    ///        "unknown",
    ///        "unreachable"
    ///      ]
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "size_gb": {
    ///      "type": "integer"
    ///    },
    ///    "snapshot_retention": {
    ///      "type": "integer"
    ///    },
    ///    "state": {
    ///      "type": "string"
    ///    },
    ///    "zone": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Volume {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attached_alloc_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attached_machine_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auto_backup_enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block_size: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub blocks: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub blocks_avail: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub blocks_free: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encrypted: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fstype: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host_status: Option<VolumeHostStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size_gb: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_retention: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zone: Option<String>,
    }

    impl From<&Volume> for Volume {
        fn from(value: &Volume) -> Self {
            value.clone()
        }
    }

    impl Volume {
        pub fn builder() -> builder::Volume {
            Default::default()
        }
    }

    ///VolumeHostStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ok",
    ///    "unknown",
    ///    "unreachable"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum VolumeHostStatus {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "unreachable")]
        Unreachable,
    }

    impl From<&VolumeHostStatus> for VolumeHostStatus {
        fn from(value: &VolumeHostStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for VolumeHostStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Ok => "ok".to_string(),
                Self::Unknown => "unknown".to_string(),
                Self::Unreachable => "unreachable".to_string(),
            }
        }
    }

    impl std::str::FromStr for VolumeHostStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "ok" => Ok(Self::Ok),
                "unknown" => Ok(Self::Unknown),
                "unreachable" => Ok(Self::Unreachable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VolumeHostStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VolumeHostStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VolumeHostStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///VolumeSnapshot
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "digest": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "type": "integer"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VolumeSnapshot {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub digest: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
    }

    impl From<&VolumeSnapshot> for VolumeSnapshot {
        fn from(value: &VolumeSnapshot) -> Self {
            value.clone()
        }
    }

    impl VolumeSnapshot {
        pub fn builder() -> builder::VolumeSnapshot {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct App {
            id: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            organization: Result<Option<super::Organization>, String>,
            status: Result<Option<String>, String>,
        }

        impl Default for App {
            fn default() -> Self {
                Self {
                    id: Ok(Default::default()),
                    name: Ok(Default::default()),
                    organization: Ok(Default::default()),
                    status: Ok(Default::default()),
                }
            }
        }

        impl App {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn organization<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Organization>>,
                T::Error: std::fmt::Display,
            {
                self.organization = value.try_into().map_err(|e| {
                    format!("error converting supplied value for organization: {}", e)
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<App> for super::App {
            type Error = super::error::ConversionError;
            fn try_from(value: App) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    name: value.name?,
                    organization: value.organization?,
                    status: value.status?,
                })
            }
        }

        impl From<super::App> for App {
            fn from(value: super::App) -> Self {
                Self {
                    id: Ok(value.id),
                    name: Ok(value.name),
                    organization: Ok(value.organization),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CheckStatus {
            name: Result<Option<String>, String>,
            output: Result<Option<String>, String>,
            status: Result<Option<String>, String>,
            updated_at: Result<Option<String>, String>,
        }

        impl Default for CheckStatus {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                    output: Ok(Default::default()),
                    status: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                }
            }
        }

        impl CheckStatus {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn output<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.output = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for output: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CheckStatus> for super::CheckStatus {
            type Error = super::error::ConversionError;
            fn try_from(value: CheckStatus) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    output: value.output?,
                    status: value.status?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl From<super::CheckStatus> for CheckStatus {
            fn from(value: super::CheckStatus) -> Self {
                Self {
                    name: Ok(value.name),
                    output: Ok(value.output),
                    status: Ok(value.status),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateAppRequest {
            app_name: Result<Option<String>, String>,
            enable_subdomains: Result<Option<bool>, String>,
            network: Result<Option<String>, String>,
            org_slug: Result<Option<String>, String>,
        }

        impl Default for CreateAppRequest {
            fn default() -> Self {
                Self {
                    app_name: Ok(Default::default()),
                    enable_subdomains: Ok(Default::default()),
                    network: Ok(Default::default()),
                    org_slug: Ok(Default::default()),
                }
            }
        }

        impl CreateAppRequest {
            pub fn app_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.app_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for app_name: {}", e));
                self
            }
            pub fn enable_subdomains<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.enable_subdomains = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for enable_subdomains: {}",
                        e
                    )
                });
                self
            }
            pub fn network<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.network = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for network: {}", e));
                self
            }
            pub fn org_slug<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.org_slug = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for org_slug: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateAppRequest> for super::CreateAppRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: CreateAppRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    app_name: value.app_name?,
                    enable_subdomains: value.enable_subdomains?,
                    network: value.network?,
                    org_slug: value.org_slug?,
                })
            }
        }

        impl From<super::CreateAppRequest> for CreateAppRequest {
            fn from(value: super::CreateAppRequest) -> Self {
                Self {
                    app_name: Ok(value.app_name),
                    enable_subdomains: Ok(value.enable_subdomains),
                    network: Ok(value.network),
                    org_slug: Ok(value.org_slug),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateLeaseRequest {
            description: Result<Option<String>, String>,
            ttl: Result<Option<i64>, String>,
        }

        impl Default for CreateLeaseRequest {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    ttl: Ok(Default::default()),
                }
            }
        }

        impl CreateLeaseRequest {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn ttl<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.ttl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ttl: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateLeaseRequest> for super::CreateLeaseRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: CreateLeaseRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    description: value.description?,
                    ttl: value.ttl?,
                })
            }
        }

        impl From<super::CreateLeaseRequest> for CreateLeaseRequest {
            fn from(value: super::CreateLeaseRequest) -> Self {
                Self {
                    description: Ok(value.description),
                    ttl: Ok(value.ttl),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateMachineRequest {
            config: Result<Option<super::FlyMachineConfig>, String>,
            lease_ttl: Result<Option<i64>, String>,
            lsvd: Result<Option<bool>, String>,
            name: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            skip_launch: Result<Option<bool>, String>,
            skip_service_registration: Result<Option<bool>, String>,
        }

        impl Default for CreateMachineRequest {
            fn default() -> Self {
                Self {
                    config: Ok(Default::default()),
                    lease_ttl: Ok(Default::default()),
                    lsvd: Ok(Default::default()),
                    name: Ok(Default::default()),
                    region: Ok(Default::default()),
                    skip_launch: Ok(Default::default()),
                    skip_service_registration: Ok(Default::default()),
                }
            }
        }

        impl CreateMachineRequest {
            pub fn config<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineConfig>>,
                T::Error: std::fmt::Display,
            {
                self.config = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for config: {}", e));
                self
            }
            pub fn lease_ttl<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.lease_ttl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lease_ttl: {}", e));
                self
            }
            pub fn lsvd<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.lsvd = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lsvd: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn skip_launch<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_launch = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for skip_launch: {}", e));
                self
            }
            pub fn skip_service_registration<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_service_registration = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_service_registration: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<CreateMachineRequest> for super::CreateMachineRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreateMachineRequest,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    config: value.config?,
                    lease_ttl: value.lease_ttl?,
                    lsvd: value.lsvd?,
                    name: value.name?,
                    region: value.region?,
                    skip_launch: value.skip_launch?,
                    skip_service_registration: value.skip_service_registration?,
                })
            }
        }

        impl From<super::CreateMachineRequest> for CreateMachineRequest {
            fn from(value: super::CreateMachineRequest) -> Self {
                Self {
                    config: Ok(value.config),
                    lease_ttl: Ok(value.lease_ttl),
                    lsvd: Ok(value.lsvd),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    skip_launch: Ok(value.skip_launch),
                    skip_service_registration: Ok(value.skip_service_registration),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateOidcTokenRequest {
            aud: Result<Option<String>, String>,
        }

        impl Default for CreateOidcTokenRequest {
            fn default() -> Self {
                Self {
                    aud: Ok(Default::default()),
                }
            }
        }

        impl CreateOidcTokenRequest {
            pub fn aud<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.aud = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for aud: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateOidcTokenRequest> for super::CreateOidcTokenRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreateOidcTokenRequest,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self { aud: value.aud? })
            }
        }

        impl From<super::CreateOidcTokenRequest> for CreateOidcTokenRequest {
            fn from(value: super::CreateOidcTokenRequest) -> Self {
                Self { aud: Ok(value.aud) }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateVolumeRequest {
            compute: Result<Option<super::FlyMachineGuest>, String>,
            compute_image: Result<Option<String>, String>,
            encrypted: Result<Option<bool>, String>,
            fstype: Result<Option<String>, String>,
            machines_only: Result<Option<bool>, String>,
            name: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            require_unique_zone: Result<Option<bool>, String>,
            size_gb: Result<Option<i64>, String>,
            snapshot_id: Result<Option<String>, String>,
            snapshot_retention: Result<Option<i64>, String>,
            source_volume_id: Result<Option<String>, String>,
        }

        impl Default for CreateVolumeRequest {
            fn default() -> Self {
                Self {
                    compute: Ok(Default::default()),
                    compute_image: Ok(Default::default()),
                    encrypted: Ok(Default::default()),
                    fstype: Ok(Default::default()),
                    machines_only: Ok(Default::default()),
                    name: Ok(Default::default()),
                    region: Ok(Default::default()),
                    require_unique_zone: Ok(Default::default()),
                    size_gb: Ok(Default::default()),
                    snapshot_id: Ok(Default::default()),
                    snapshot_retention: Ok(Default::default()),
                    source_volume_id: Ok(Default::default()),
                }
            }
        }

        impl CreateVolumeRequest {
            pub fn compute<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineGuest>>,
                T::Error: std::fmt::Display,
            {
                self.compute = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for compute: {}", e));
                self
            }
            pub fn compute_image<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.compute_image = value.try_into().map_err(|e| {
                    format!("error converting supplied value for compute_image: {}", e)
                });
                self
            }
            pub fn encrypted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.encrypted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for encrypted: {}", e));
                self
            }
            pub fn fstype<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fstype = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fstype: {}", e));
                self
            }
            pub fn machines_only<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.machines_only = value.try_into().map_err(|e| {
                    format!("error converting supplied value for machines_only: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn require_unique_zone<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.require_unique_zone = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for require_unique_zone: {}",
                        e
                    )
                });
                self
            }
            pub fn size_gb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size_gb = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_gb: {}", e));
                self
            }
            pub fn snapshot_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.snapshot_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for snapshot_id: {}", e));
                self
            }
            pub fn snapshot_retention<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.snapshot_retention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snapshot_retention: {}",
                        e
                    )
                });
                self
            }
            pub fn source_volume_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.source_volume_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for source_volume_id: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<CreateVolumeRequest> for super::CreateVolumeRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: CreateVolumeRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    compute: value.compute?,
                    compute_image: value.compute_image?,
                    encrypted: value.encrypted?,
                    fstype: value.fstype?,
                    machines_only: value.machines_only?,
                    name: value.name?,
                    region: value.region?,
                    require_unique_zone: value.require_unique_zone?,
                    size_gb: value.size_gb?,
                    snapshot_id: value.snapshot_id?,
                    snapshot_retention: value.snapshot_retention?,
                    source_volume_id: value.source_volume_id?,
                })
            }
        }

        impl From<super::CreateVolumeRequest> for CreateVolumeRequest {
            fn from(value: super::CreateVolumeRequest) -> Self {
                Self {
                    compute: Ok(value.compute),
                    compute_image: Ok(value.compute_image),
                    encrypted: Ok(value.encrypted),
                    fstype: Ok(value.fstype),
                    machines_only: Ok(value.machines_only),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    require_unique_zone: Ok(value.require_unique_zone),
                    size_gb: Ok(value.size_gb),
                    snapshot_id: Ok(value.snapshot_id),
                    snapshot_retention: Ok(value.snapshot_retention),
                    source_volume_id: Ok(value.source_volume_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ErrorResponse {
            details: Result<serde_json::Map<String, serde_json::Value>, String>,
            error: Result<Option<String>, String>,
            status: Result<Option<super::MainStatusCode>, String>,
        }

        impl Default for ErrorResponse {
            fn default() -> Self {
                Self {
                    details: Ok(Default::default()),
                    error: Ok(Default::default()),
                    status: Ok(Default::default()),
                }
            }
        }

        impl ErrorResponse {
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {}", e));
                self
            }
            pub fn error<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::MainStatusCode>>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ErrorResponse> for super::ErrorResponse {
            type Error = super::error::ConversionError;
            fn try_from(value: ErrorResponse) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    details: value.details?,
                    error: value.error?,
                    status: value.status?,
                })
            }
        }

        impl From<super::ErrorResponse> for ErrorResponse {
            fn from(value: super::ErrorResponse) -> Self {
                Self {
                    details: Ok(value.details),
                    error: Ok(value.error),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ExtendVolumeRequest {
            size_gb: Result<Option<i64>, String>,
        }

        impl Default for ExtendVolumeRequest {
            fn default() -> Self {
                Self {
                    size_gb: Ok(Default::default()),
                }
            }
        }

        impl ExtendVolumeRequest {
            pub fn size_gb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size_gb = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_gb: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ExtendVolumeRequest> for super::ExtendVolumeRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: ExtendVolumeRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    size_gb: value.size_gb?,
                })
            }
        }

        impl From<super::ExtendVolumeRequest> for ExtendVolumeRequest {
            fn from(value: super::ExtendVolumeRequest) -> Self {
                Self {
                    size_gb: Ok(value.size_gb),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ExtendVolumeResponse {
            needs_restart: Result<Option<bool>, String>,
            volume: Result<Option<super::Volume>, String>,
        }

        impl Default for ExtendVolumeResponse {
            fn default() -> Self {
                Self {
                    needs_restart: Ok(Default::default()),
                    volume: Ok(Default::default()),
                }
            }
        }

        impl ExtendVolumeResponse {
            pub fn needs_restart<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.needs_restart = value.try_into().map_err(|e| {
                    format!("error converting supplied value for needs_restart: {}", e)
                });
                self
            }
            pub fn volume<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Volume>>,
                T::Error: std::fmt::Display,
            {
                self.volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volume: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ExtendVolumeResponse> for super::ExtendVolumeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ExtendVolumeResponse,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    needs_restart: value.needs_restart?,
                    volume: value.volume?,
                })
            }
        }

        impl From<super::ExtendVolumeResponse> for ExtendVolumeResponse {
            fn from(value: super::ExtendVolumeResponse) -> Self {
                Self {
                    needs_restart: Ok(value.needs_restart),
                    volume: Ok(value.volume),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyDnsConfig {
            dns_forward_rules: Result<Vec<super::FlyDnsForwardRule>, String>,
            nameservers: Result<Vec<String>, String>,
            options: Result<Vec<super::FlyDnsOption>, String>,
            searches: Result<Vec<String>, String>,
            skip_registration: Result<Option<bool>, String>,
        }

        impl Default for FlyDnsConfig {
            fn default() -> Self {
                Self {
                    dns_forward_rules: Ok(Default::default()),
                    nameservers: Ok(Default::default()),
                    options: Ok(Default::default()),
                    searches: Ok(Default::default()),
                    skip_registration: Ok(Default::default()),
                }
            }
        }

        impl FlyDnsConfig {
            pub fn dns_forward_rules<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyDnsForwardRule>>,
                T::Error: std::fmt::Display,
            {
                self.dns_forward_rules = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for dns_forward_rules: {}",
                        e
                    )
                });
                self
            }
            pub fn nameservers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.nameservers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nameservers: {}", e));
                self
            }
            pub fn options<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyDnsOption>>,
                T::Error: std::fmt::Display,
            {
                self.options = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for options: {}", e));
                self
            }
            pub fn searches<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.searches = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for searches: {}", e));
                self
            }
            pub fn skip_registration<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_registration = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_registration: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<FlyDnsConfig> for super::FlyDnsConfig {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyDnsConfig) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    dns_forward_rules: value.dns_forward_rules?,
                    nameservers: value.nameservers?,
                    options: value.options?,
                    searches: value.searches?,
                    skip_registration: value.skip_registration?,
                })
            }
        }

        impl From<super::FlyDnsConfig> for FlyDnsConfig {
            fn from(value: super::FlyDnsConfig) -> Self {
                Self {
                    dns_forward_rules: Ok(value.dns_forward_rules),
                    nameservers: Ok(value.nameservers),
                    options: Ok(value.options),
                    searches: Ok(value.searches),
                    skip_registration: Ok(value.skip_registration),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyDnsForwardRule {
            addr: Result<Option<String>, String>,
            basename: Result<Option<String>, String>,
        }

        impl Default for FlyDnsForwardRule {
            fn default() -> Self {
                Self {
                    addr: Ok(Default::default()),
                    basename: Ok(Default::default()),
                }
            }
        }

        impl FlyDnsForwardRule {
            pub fn addr<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.addr = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for addr: {}", e));
                self
            }
            pub fn basename<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.basename = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for basename: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyDnsForwardRule> for super::FlyDnsForwardRule {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyDnsForwardRule) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    addr: value.addr?,
                    basename: value.basename?,
                })
            }
        }

        impl From<super::FlyDnsForwardRule> for FlyDnsForwardRule {
            fn from(value: super::FlyDnsForwardRule) -> Self {
                Self {
                    addr: Ok(value.addr),
                    basename: Ok(value.basename),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyDnsOption {
            name: Result<Option<String>, String>,
            value: Result<Option<String>, String>,
        }

        impl Default for FlyDnsOption {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }

        impl FlyDnsOption {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyDnsOption> for super::FlyDnsOption {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyDnsOption) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    value: value.value?,
                })
            }
        }

        impl From<super::FlyDnsOption> for FlyDnsOption {
            fn from(value: super::FlyDnsOption) -> Self {
                Self {
                    name: Ok(value.name),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyDuration {
            time_duration: Result<Option<i64>, String>,
        }

        impl Default for FlyDuration {
            fn default() -> Self {
                Self {
                    time_duration: Ok(Default::default()),
                }
            }
        }

        impl FlyDuration {
            pub fn time_duration<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.time_duration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for time_duration: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<FlyDuration> for super::FlyDuration {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyDuration) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    time_duration: value.time_duration?,
                })
            }
        }

        impl From<super::FlyDuration> for FlyDuration {
            fn from(value: super::FlyDuration) -> Self {
                Self {
                    time_duration: Ok(value.time_duration),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyEnvFrom {
            env_var: Result<Option<String>, String>,
            field_ref: Result<Option<super::FlyEnvFromFieldRef>, String>,
        }

        impl Default for FlyEnvFrom {
            fn default() -> Self {
                Self {
                    env_var: Ok(Default::default()),
                    field_ref: Ok(Default::default()),
                }
            }
        }

        impl FlyEnvFrom {
            pub fn env_var<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.env_var = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for env_var: {}", e));
                self
            }
            pub fn field_ref<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyEnvFromFieldRef>>,
                T::Error: std::fmt::Display,
            {
                self.field_ref = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for field_ref: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyEnvFrom> for super::FlyEnvFrom {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyEnvFrom) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    env_var: value.env_var?,
                    field_ref: value.field_ref?,
                })
            }
        }

        impl From<super::FlyEnvFrom> for FlyEnvFrom {
            fn from(value: super::FlyEnvFrom) -> Self {
                Self {
                    env_var: Ok(value.env_var),
                    field_ref: Ok(value.field_ref),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyFile {
            guest_path: Result<Option<String>, String>,
            raw_value: Result<Option<String>, String>,
            secret_name: Result<Option<String>, String>,
        }

        impl Default for FlyFile {
            fn default() -> Self {
                Self {
                    guest_path: Ok(Default::default()),
                    raw_value: Ok(Default::default()),
                    secret_name: Ok(Default::default()),
                }
            }
        }

        impl FlyFile {
            pub fn guest_path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.guest_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for guest_path: {}", e));
                self
            }
            pub fn raw_value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.raw_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for raw_value: {}", e));
                self
            }
            pub fn secret_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.secret_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret_name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyFile> for super::FlyFile {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyFile) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    guest_path: value.guest_path?,
                    raw_value: value.raw_value?,
                    secret_name: value.secret_name?,
                })
            }
        }

        impl From<super::FlyFile> for FlyFile {
            fn from(value: super::FlyFile) -> Self {
                Self {
                    guest_path: Ok(value.guest_path),
                    raw_value: Ok(value.raw_value),
                    secret_name: Ok(value.secret_name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyHttpOptions {
            compress: Result<Option<bool>, String>,
            h2_backend: Result<Option<bool>, String>,
            response: Result<Option<super::FlyHttpResponseOptions>, String>,
        }

        impl Default for FlyHttpOptions {
            fn default() -> Self {
                Self {
                    compress: Ok(Default::default()),
                    h2_backend: Ok(Default::default()),
                    response: Ok(Default::default()),
                }
            }
        }

        impl FlyHttpOptions {
            pub fn compress<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.compress = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for compress: {}", e));
                self
            }
            pub fn h2_backend<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.h2_backend = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for h2_backend: {}", e));
                self
            }
            pub fn response<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyHttpResponseOptions>>,
                T::Error: std::fmt::Display,
            {
                self.response = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for response: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyHttpOptions> for super::FlyHttpOptions {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyHttpOptions) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    compress: value.compress?,
                    h2_backend: value.h2_backend?,
                    response: value.response?,
                })
            }
        }

        impl From<super::FlyHttpOptions> for FlyHttpOptions {
            fn from(value: super::FlyHttpOptions) -> Self {
                Self {
                    compress: Ok(value.compress),
                    h2_backend: Ok(value.h2_backend),
                    response: Ok(value.response),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyHttpResponseOptions {
            headers: Result<
                std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
                String,
            >,
            pristine: Result<Option<bool>, String>,
        }

        impl Default for FlyHttpResponseOptions {
            fn default() -> Self {
                Self {
                    headers: Ok(Default::default()),
                    pristine: Ok(Default::default()),
                }
            }
        }

        impl FlyHttpResponseOptions {
            pub fn headers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
                >,
                T::Error: std::fmt::Display,
            {
                self.headers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for headers: {}", e));
                self
            }
            pub fn pristine<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.pristine = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pristine: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyHttpResponseOptions> for super::FlyHttpResponseOptions {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FlyHttpResponseOptions,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    headers: value.headers?,
                    pristine: value.pristine?,
                })
            }
        }

        impl From<super::FlyHttpResponseOptions> for FlyHttpResponseOptions {
            fn from(value: super::FlyHttpResponseOptions) -> Self {
                Self {
                    headers: Ok(value.headers),
                    pristine: Ok(value.pristine),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineCheck {
            grace_period: Result<Option<super::FlyDuration>, String>,
            headers: Result<Vec<super::FlyMachineHttpHeader>, String>,
            interval: Result<Option<super::FlyDuration>, String>,
            method: Result<Option<String>, String>,
            path: Result<Option<String>, String>,
            port: Result<Option<i64>, String>,
            protocol: Result<Option<String>, String>,
            timeout: Result<Option<super::FlyDuration>, String>,
            tls_server_name: Result<Option<String>, String>,
            tls_skip_verify: Result<Option<bool>, String>,
            type_: Result<Option<String>, String>,
        }

        impl Default for FlyMachineCheck {
            fn default() -> Self {
                Self {
                    grace_period: Ok(Default::default()),
                    headers: Ok(Default::default()),
                    interval: Ok(Default::default()),
                    method: Ok(Default::default()),
                    path: Ok(Default::default()),
                    port: Ok(Default::default()),
                    protocol: Ok(Default::default()),
                    timeout: Ok(Default::default()),
                    tls_server_name: Ok(Default::default()),
                    tls_skip_verify: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineCheck {
            pub fn grace_period<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyDuration>>,
                T::Error: std::fmt::Display,
            {
                self.grace_period = value.try_into().map_err(|e| {
                    format!("error converting supplied value for grace_period: {}", e)
                });
                self
            }
            pub fn headers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachineHttpHeader>>,
                T::Error: std::fmt::Display,
            {
                self.headers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for headers: {}", e));
                self
            }
            pub fn interval<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyDuration>>,
                T::Error: std::fmt::Display,
            {
                self.interval = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for interval: {}", e));
                self
            }
            pub fn method<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.method = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for method: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn port<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.port = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for port: {}", e));
                self
            }
            pub fn protocol<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.protocol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for protocol: {}", e));
                self
            }
            pub fn timeout<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyDuration>>,
                T::Error: std::fmt::Display,
            {
                self.timeout = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timeout: {}", e));
                self
            }
            pub fn tls_server_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tls_server_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tls_server_name: {}", e)
                });
                self
            }
            pub fn tls_skip_verify<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.tls_skip_verify = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tls_skip_verify: {}", e)
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineCheck> for super::FlyMachineCheck {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineCheck) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    grace_period: value.grace_period?,
                    headers: value.headers?,
                    interval: value.interval?,
                    method: value.method?,
                    path: value.path?,
                    port: value.port?,
                    protocol: value.protocol?,
                    timeout: value.timeout?,
                    tls_server_name: value.tls_server_name?,
                    tls_skip_verify: value.tls_skip_verify?,
                    type_: value.type_?,
                })
            }
        }

        impl From<super::FlyMachineCheck> for FlyMachineCheck {
            fn from(value: super::FlyMachineCheck) -> Self {
                Self {
                    grace_period: Ok(value.grace_period),
                    headers: Ok(value.headers),
                    interval: Ok(value.interval),
                    method: Ok(value.method),
                    path: Ok(value.path),
                    port: Ok(value.port),
                    protocol: Ok(value.protocol),
                    timeout: Ok(value.timeout),
                    tls_server_name: Ok(value.tls_server_name),
                    tls_skip_verify: Ok(value.tls_skip_verify),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineConfig {
            auto_destroy: Result<Option<bool>, String>,
            checks: Result<std::collections::HashMap<String, super::FlyMachineCheck>, String>,
            disable_machine_autostart: Result<Option<bool>, String>,
            dns: Result<Option<super::FlyDnsConfig>, String>,
            env: Result<std::collections::HashMap<String, String>, String>,
            files: Result<Vec<super::FlyFile>, String>,
            guest: Result<Option<super::FlyMachineGuest>, String>,
            image: Result<Option<String>, String>,
            init: Result<Option<super::FlyMachineInit>, String>,
            metadata: Result<std::collections::HashMap<String, String>, String>,
            metrics: Result<Option<super::FlyMachineMetrics>, String>,
            mounts: Result<Vec<super::FlyMachineMount>, String>,
            processes: Result<Vec<super::FlyMachineProcess>, String>,
            restart: Result<Option<super::FlyMachineRestart>, String>,
            schedule: Result<Option<String>, String>,
            services: Result<Vec<super::FlyMachineService>, String>,
            size: Result<Option<String>, String>,
            standbys: Result<Vec<String>, String>,
            statics: Result<Vec<super::FlyStatic>, String>,
            stop_config: Result<Option<super::FlyStopConfig>, String>,
        }

        impl Default for FlyMachineConfig {
            fn default() -> Self {
                Self {
                    auto_destroy: Ok(Default::default()),
                    checks: Ok(Default::default()),
                    disable_machine_autostart: Ok(Default::default()),
                    dns: Ok(Default::default()),
                    env: Ok(Default::default()),
                    files: Ok(Default::default()),
                    guest: Ok(Default::default()),
                    image: Ok(Default::default()),
                    init: Ok(Default::default()),
                    metadata: Ok(Default::default()),
                    metrics: Ok(Default::default()),
                    mounts: Ok(Default::default()),
                    processes: Ok(Default::default()),
                    restart: Ok(Default::default()),
                    schedule: Ok(Default::default()),
                    services: Ok(Default::default()),
                    size: Ok(Default::default()),
                    standbys: Ok(Default::default()),
                    statics: Ok(Default::default()),
                    stop_config: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineConfig {
            pub fn auto_destroy<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.auto_destroy = value.try_into().map_err(|e| {
                    format!("error converting supplied value for auto_destroy: {}", e)
                });
                self
            }
            pub fn checks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, super::FlyMachineCheck>>,
                T::Error: std::fmt::Display,
            {
                self.checks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for checks: {}", e));
                self
            }
            pub fn disable_machine_autostart<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.disable_machine_autostart = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for disable_machine_autostart: {}",
                        e
                    )
                });
                self
            }
            pub fn dns<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyDnsConfig>>,
                T::Error: std::fmt::Display,
            {
                self.dns = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dns: {}", e));
                self
            }
            pub fn env<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, String>>,
                T::Error: std::fmt::Display,
            {
                self.env = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for env: {}", e));
                self
            }
            pub fn files<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyFile>>,
                T::Error: std::fmt::Display,
            {
                self.files = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for files: {}", e));
                self
            }
            pub fn guest<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineGuest>>,
                T::Error: std::fmt::Display,
            {
                self.guest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for guest: {}", e));
                self
            }
            pub fn image<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.image = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for image: {}", e));
                self
            }
            pub fn init<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineInit>>,
                T::Error: std::fmt::Display,
            {
                self.init = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for init: {}", e));
                self
            }
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, String>>,
                T::Error: std::fmt::Display,
            {
                self.metadata = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for metadata: {}", e));
                self
            }
            pub fn metrics<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineMetrics>>,
                T::Error: std::fmt::Display,
            {
                self.metrics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for metrics: {}", e));
                self
            }
            pub fn mounts<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachineMount>>,
                T::Error: std::fmt::Display,
            {
                self.mounts = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mounts: {}", e));
                self
            }
            pub fn processes<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachineProcess>>,
                T::Error: std::fmt::Display,
            {
                self.processes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for processes: {}", e));
                self
            }
            pub fn restart<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineRestart>>,
                T::Error: std::fmt::Display,
            {
                self.restart = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for restart: {}", e));
                self
            }
            pub fn schedule<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.schedule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for schedule: {}", e));
                self
            }
            pub fn services<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachineService>>,
                T::Error: std::fmt::Display,
            {
                self.services = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for services: {}", e));
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
            pub fn standbys<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.standbys = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for standbys: {}", e));
                self
            }
            pub fn statics<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyStatic>>,
                T::Error: std::fmt::Display,
            {
                self.statics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for statics: {}", e));
                self
            }
            pub fn stop_config<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyStopConfig>>,
                T::Error: std::fmt::Display,
            {
                self.stop_config = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stop_config: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineConfig> for super::FlyMachineConfig {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineConfig) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    auto_destroy: value.auto_destroy?,
                    checks: value.checks?,
                    disable_machine_autostart: value.disable_machine_autostart?,
                    dns: value.dns?,
                    env: value.env?,
                    files: value.files?,
                    guest: value.guest?,
                    image: value.image?,
                    init: value.init?,
                    metadata: value.metadata?,
                    metrics: value.metrics?,
                    mounts: value.mounts?,
                    processes: value.processes?,
                    restart: value.restart?,
                    schedule: value.schedule?,
                    services: value.services?,
                    size: value.size?,
                    standbys: value.standbys?,
                    statics: value.statics?,
                    stop_config: value.stop_config?,
                })
            }
        }

        impl From<super::FlyMachineConfig> for FlyMachineConfig {
            fn from(value: super::FlyMachineConfig) -> Self {
                Self {
                    auto_destroy: Ok(value.auto_destroy),
                    checks: Ok(value.checks),
                    disable_machine_autostart: Ok(value.disable_machine_autostart),
                    dns: Ok(value.dns),
                    env: Ok(value.env),
                    files: Ok(value.files),
                    guest: Ok(value.guest),
                    image: Ok(value.image),
                    init: Ok(value.init),
                    metadata: Ok(value.metadata),
                    metrics: Ok(value.metrics),
                    mounts: Ok(value.mounts),
                    processes: Ok(value.processes),
                    restart: Ok(value.restart),
                    schedule: Ok(value.schedule),
                    services: Ok(value.services),
                    size: Ok(value.size),
                    standbys: Ok(value.standbys),
                    statics: Ok(value.statics),
                    stop_config: Ok(value.stop_config),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineGuest {
            cpu_kind: Result<Option<String>, String>,
            cpus: Result<Option<i64>, String>,
            gpu_kind: Result<Option<String>, String>,
            gpus: Result<Option<i64>, String>,
            host_dedication_id: Result<Option<String>, String>,
            kernel_args: Result<Vec<String>, String>,
            memory_mb: Result<Option<i64>, String>,
        }

        impl Default for FlyMachineGuest {
            fn default() -> Self {
                Self {
                    cpu_kind: Ok(Default::default()),
                    cpus: Ok(Default::default()),
                    gpu_kind: Ok(Default::default()),
                    gpus: Ok(Default::default()),
                    host_dedication_id: Ok(Default::default()),
                    kernel_args: Ok(Default::default()),
                    memory_mb: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineGuest {
            pub fn cpu_kind<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.cpu_kind = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cpu_kind: {}", e));
                self
            }
            pub fn cpus<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.cpus = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cpus: {}", e));
                self
            }
            pub fn gpu_kind<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.gpu_kind = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gpu_kind: {}", e));
                self
            }
            pub fn gpus<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.gpus = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gpus: {}", e));
                self
            }
            pub fn host_dedication_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.host_dedication_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for host_dedication_id: {}",
                        e
                    )
                });
                self
            }
            pub fn kernel_args<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.kernel_args = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kernel_args: {}", e));
                self
            }
            pub fn memory_mb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.memory_mb = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for memory_mb: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineGuest> for super::FlyMachineGuest {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineGuest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cpu_kind: value.cpu_kind?,
                    cpus: value.cpus?,
                    gpu_kind: value.gpu_kind?,
                    gpus: value.gpus?,
                    host_dedication_id: value.host_dedication_id?,
                    kernel_args: value.kernel_args?,
                    memory_mb: value.memory_mb?,
                })
            }
        }

        impl From<super::FlyMachineGuest> for FlyMachineGuest {
            fn from(value: super::FlyMachineGuest) -> Self {
                Self {
                    cpu_kind: Ok(value.cpu_kind),
                    cpus: Ok(value.cpus),
                    gpu_kind: Ok(value.gpu_kind),
                    gpus: Ok(value.gpus),
                    host_dedication_id: Ok(value.host_dedication_id),
                    kernel_args: Ok(value.kernel_args),
                    memory_mb: Ok(value.memory_mb),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineHttpHeader {
            name: Result<Option<String>, String>,
            values: Result<Vec<String>, String>,
        }

        impl Default for FlyMachineHttpHeader {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                    values: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineHttpHeader {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn values<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.values = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for values: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineHttpHeader> for super::FlyMachineHttpHeader {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FlyMachineHttpHeader,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    values: value.values?,
                })
            }
        }

        impl From<super::FlyMachineHttpHeader> for FlyMachineHttpHeader {
            fn from(value: super::FlyMachineHttpHeader) -> Self {
                Self {
                    name: Ok(value.name),
                    values: Ok(value.values),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineInit {
            cmd: Result<Vec<String>, String>,
            entrypoint: Result<Vec<String>, String>,
            exec: Result<Vec<String>, String>,
            kernel_args: Result<Vec<String>, String>,
            swap_size_mb: Result<Option<i64>, String>,
            tty: Result<Option<bool>, String>,
        }

        impl Default for FlyMachineInit {
            fn default() -> Self {
                Self {
                    cmd: Ok(Default::default()),
                    entrypoint: Ok(Default::default()),
                    exec: Ok(Default::default()),
                    kernel_args: Ok(Default::default()),
                    swap_size_mb: Ok(Default::default()),
                    tty: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineInit {
            pub fn cmd<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.cmd = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cmd: {}", e));
                self
            }
            pub fn entrypoint<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.entrypoint = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entrypoint: {}", e));
                self
            }
            pub fn exec<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.exec = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exec: {}", e));
                self
            }
            pub fn kernel_args<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.kernel_args = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kernel_args: {}", e));
                self
            }
            pub fn swap_size_mb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.swap_size_mb = value.try_into().map_err(|e| {
                    format!("error converting supplied value for swap_size_mb: {}", e)
                });
                self
            }
            pub fn tty<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.tty = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tty: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineInit> for super::FlyMachineInit {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineInit) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cmd: value.cmd?,
                    entrypoint: value.entrypoint?,
                    exec: value.exec?,
                    kernel_args: value.kernel_args?,
                    swap_size_mb: value.swap_size_mb?,
                    tty: value.tty?,
                })
            }
        }

        impl From<super::FlyMachineInit> for FlyMachineInit {
            fn from(value: super::FlyMachineInit) -> Self {
                Self {
                    cmd: Ok(value.cmd),
                    entrypoint: Ok(value.entrypoint),
                    exec: Ok(value.exec),
                    kernel_args: Ok(value.kernel_args),
                    swap_size_mb: Ok(value.swap_size_mb),
                    tty: Ok(value.tty),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineMetrics {
            path: Result<Option<String>, String>,
            port: Result<Option<i64>, String>,
        }

        impl Default for FlyMachineMetrics {
            fn default() -> Self {
                Self {
                    path: Ok(Default::default()),
                    port: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineMetrics {
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn port<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.port = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for port: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineMetrics> for super::FlyMachineMetrics {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineMetrics) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    path: value.path?,
                    port: value.port?,
                })
            }
        }

        impl From<super::FlyMachineMetrics> for FlyMachineMetrics {
            fn from(value: super::FlyMachineMetrics) -> Self {
                Self {
                    path: Ok(value.path),
                    port: Ok(value.port),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineMount {
            add_size_gb: Result<Option<i64>, String>,
            encrypted: Result<Option<bool>, String>,
            extend_threshold_percent: Result<Option<i64>, String>,
            name: Result<Option<String>, String>,
            path: Result<Option<String>, String>,
            size_gb: Result<Option<i64>, String>,
            size_gb_limit: Result<Option<i64>, String>,
            volume: Result<Option<String>, String>,
        }

        impl Default for FlyMachineMount {
            fn default() -> Self {
                Self {
                    add_size_gb: Ok(Default::default()),
                    encrypted: Ok(Default::default()),
                    extend_threshold_percent: Ok(Default::default()),
                    name: Ok(Default::default()),
                    path: Ok(Default::default()),
                    size_gb: Ok(Default::default()),
                    size_gb_limit: Ok(Default::default()),
                    volume: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineMount {
            pub fn add_size_gb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.add_size_gb = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for add_size_gb: {}", e));
                self
            }
            pub fn encrypted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.encrypted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for encrypted: {}", e));
                self
            }
            pub fn extend_threshold_percent<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.extend_threshold_percent = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for extend_threshold_percent: {}",
                        e
                    )
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn size_gb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size_gb = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_gb: {}", e));
                self
            }
            pub fn size_gb_limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size_gb_limit = value.try_into().map_err(|e| {
                    format!("error converting supplied value for size_gb_limit: {}", e)
                });
                self
            }
            pub fn volume<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volume: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineMount> for super::FlyMachineMount {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineMount) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    add_size_gb: value.add_size_gb?,
                    encrypted: value.encrypted?,
                    extend_threshold_percent: value.extend_threshold_percent?,
                    name: value.name?,
                    path: value.path?,
                    size_gb: value.size_gb?,
                    size_gb_limit: value.size_gb_limit?,
                    volume: value.volume?,
                })
            }
        }

        impl From<super::FlyMachineMount> for FlyMachineMount {
            fn from(value: super::FlyMachineMount) -> Self {
                Self {
                    add_size_gb: Ok(value.add_size_gb),
                    encrypted: Ok(value.encrypted),
                    extend_threshold_percent: Ok(value.extend_threshold_percent),
                    name: Ok(value.name),
                    path: Ok(value.path),
                    size_gb: Ok(value.size_gb),
                    size_gb_limit: Ok(value.size_gb_limit),
                    volume: Ok(value.volume),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachinePort {
            end_port: Result<Option<i64>, String>,
            force_https: Result<Option<bool>, String>,
            handlers: Result<Vec<String>, String>,
            http_options: Result<Option<super::FlyHttpOptions>, String>,
            port: Result<Option<i64>, String>,
            proxy_proto_options: Result<Option<super::FlyProxyProtoOptions>, String>,
            start_port: Result<Option<i64>, String>,
            tls_options: Result<Option<super::FlyTlsOptions>, String>,
        }

        impl Default for FlyMachinePort {
            fn default() -> Self {
                Self {
                    end_port: Ok(Default::default()),
                    force_https: Ok(Default::default()),
                    handlers: Ok(Default::default()),
                    http_options: Ok(Default::default()),
                    port: Ok(Default::default()),
                    proxy_proto_options: Ok(Default::default()),
                    start_port: Ok(Default::default()),
                    tls_options: Ok(Default::default()),
                }
            }
        }

        impl FlyMachinePort {
            pub fn end_port<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.end_port = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_port: {}", e));
                self
            }
            pub fn force_https<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.force_https = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for force_https: {}", e));
                self
            }
            pub fn handlers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.handlers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for handlers: {}", e));
                self
            }
            pub fn http_options<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyHttpOptions>>,
                T::Error: std::fmt::Display,
            {
                self.http_options = value.try_into().map_err(|e| {
                    format!("error converting supplied value for http_options: {}", e)
                });
                self
            }
            pub fn port<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.port = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for port: {}", e));
                self
            }
            pub fn proxy_proto_options<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyProxyProtoOptions>>,
                T::Error: std::fmt::Display,
            {
                self.proxy_proto_options = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for proxy_proto_options: {}",
                        e
                    )
                });
                self
            }
            pub fn start_port<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.start_port = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_port: {}", e));
                self
            }
            pub fn tls_options<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyTlsOptions>>,
                T::Error: std::fmt::Display,
            {
                self.tls_options = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tls_options: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachinePort> for super::FlyMachinePort {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachinePort) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    end_port: value.end_port?,
                    force_https: value.force_https?,
                    handlers: value.handlers?,
                    http_options: value.http_options?,
                    port: value.port?,
                    proxy_proto_options: value.proxy_proto_options?,
                    start_port: value.start_port?,
                    tls_options: value.tls_options?,
                })
            }
        }

        impl From<super::FlyMachinePort> for FlyMachinePort {
            fn from(value: super::FlyMachinePort) -> Self {
                Self {
                    end_port: Ok(value.end_port),
                    force_https: Ok(value.force_https),
                    handlers: Ok(value.handlers),
                    http_options: Ok(value.http_options),
                    port: Ok(value.port),
                    proxy_proto_options: Ok(value.proxy_proto_options),
                    start_port: Ok(value.start_port),
                    tls_options: Ok(value.tls_options),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineProcess {
            cmd: Result<Vec<String>, String>,
            entrypoint: Result<Vec<String>, String>,
            env: Result<std::collections::HashMap<String, String>, String>,
            env_from: Result<Vec<super::FlyEnvFrom>, String>,
            exec: Result<Vec<String>, String>,
            ignore_app_secrets: Result<Option<bool>, String>,
            secrets: Result<Vec<super::FlyMachineSecret>, String>,
            user: Result<Option<String>, String>,
        }

        impl Default for FlyMachineProcess {
            fn default() -> Self {
                Self {
                    cmd: Ok(Default::default()),
                    entrypoint: Ok(Default::default()),
                    env: Ok(Default::default()),
                    env_from: Ok(Default::default()),
                    exec: Ok(Default::default()),
                    ignore_app_secrets: Ok(Default::default()),
                    secrets: Ok(Default::default()),
                    user: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineProcess {
            pub fn cmd<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.cmd = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cmd: {}", e));
                self
            }
            pub fn entrypoint<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.entrypoint = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entrypoint: {}", e));
                self
            }
            pub fn env<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, String>>,
                T::Error: std::fmt::Display,
            {
                self.env = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for env: {}", e));
                self
            }
            pub fn env_from<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyEnvFrom>>,
                T::Error: std::fmt::Display,
            {
                self.env_from = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for env_from: {}", e));
                self
            }
            pub fn exec<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.exec = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exec: {}", e));
                self
            }
            pub fn ignore_app_secrets<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.ignore_app_secrets = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ignore_app_secrets: {}",
                        e
                    )
                });
                self
            }
            pub fn secrets<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachineSecret>>,
                T::Error: std::fmt::Display,
            {
                self.secrets = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secrets: {}", e));
                self
            }
            pub fn user<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.user = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineProcess> for super::FlyMachineProcess {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineProcess) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cmd: value.cmd?,
                    entrypoint: value.entrypoint?,
                    env: value.env?,
                    env_from: value.env_from?,
                    exec: value.exec?,
                    ignore_app_secrets: value.ignore_app_secrets?,
                    secrets: value.secrets?,
                    user: value.user?,
                })
            }
        }

        impl From<super::FlyMachineProcess> for FlyMachineProcess {
            fn from(value: super::FlyMachineProcess) -> Self {
                Self {
                    cmd: Ok(value.cmd),
                    entrypoint: Ok(value.entrypoint),
                    env: Ok(value.env),
                    env_from: Ok(value.env_from),
                    exec: Ok(value.exec),
                    ignore_app_secrets: Ok(value.ignore_app_secrets),
                    secrets: Ok(value.secrets),
                    user: Ok(value.user),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineRestart {
            max_retries: Result<Option<i64>, String>,
            policy: Result<Option<super::FlyMachineRestartPolicy>, String>,
        }

        impl Default for FlyMachineRestart {
            fn default() -> Self {
                Self {
                    max_retries: Ok(Default::default()),
                    policy: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineRestart {
            pub fn max_retries<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.max_retries = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for max_retries: {}", e));
                self
            }
            pub fn policy<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineRestartPolicy>>,
                T::Error: std::fmt::Display,
            {
                self.policy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for policy: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineRestart> for super::FlyMachineRestart {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineRestart) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    max_retries: value.max_retries?,
                    policy: value.policy?,
                })
            }
        }

        impl From<super::FlyMachineRestart> for FlyMachineRestart {
            fn from(value: super::FlyMachineRestart) -> Self {
                Self {
                    max_retries: Ok(value.max_retries),
                    policy: Ok(value.policy),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineSecret {
            env_var: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
        }

        impl Default for FlyMachineSecret {
            fn default() -> Self {
                Self {
                    env_var: Ok(Default::default()),
                    name: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineSecret {
            pub fn env_var<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.env_var = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for env_var: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineSecret> for super::FlyMachineSecret {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineSecret) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    env_var: value.env_var?,
                    name: value.name?,
                })
            }
        }

        impl From<super::FlyMachineSecret> for FlyMachineSecret {
            fn from(value: super::FlyMachineSecret) -> Self {
                Self {
                    env_var: Ok(value.env_var),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineService {
            autostart: Result<Option<bool>, String>,
            autostop: Result<Option<bool>, String>,
            checks: Result<Vec<super::FlyMachineCheck>, String>,
            concurrency: Result<Option<super::FlyMachineServiceConcurrency>, String>,
            force_instance_description: Result<Option<String>, String>,
            force_instance_key: Result<Option<String>, String>,
            internal_port: Result<Option<i64>, String>,
            min_machines_running: Result<Option<i64>, String>,
            ports: Result<Vec<super::FlyMachinePort>, String>,
            protocol: Result<Option<String>, String>,
        }

        impl Default for FlyMachineService {
            fn default() -> Self {
                Self {
                    autostart: Ok(Default::default()),
                    autostop: Ok(Default::default()),
                    checks: Ok(Default::default()),
                    concurrency: Ok(Default::default()),
                    force_instance_description: Ok(Default::default()),
                    force_instance_key: Ok(Default::default()),
                    internal_port: Ok(Default::default()),
                    min_machines_running: Ok(Default::default()),
                    ports: Ok(Default::default()),
                    protocol: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineService {
            pub fn autostart<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.autostart = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autostart: {}", e));
                self
            }
            pub fn autostop<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.autostop = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for autostop: {}", e));
                self
            }
            pub fn checks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachineCheck>>,
                T::Error: std::fmt::Display,
            {
                self.checks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for checks: {}", e));
                self
            }
            pub fn concurrency<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineServiceConcurrency>>,
                T::Error: std::fmt::Display,
            {
                self.concurrency = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for concurrency: {}", e));
                self
            }
            pub fn force_instance_description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.force_instance_description = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for force_instance_description: {}",
                        e
                    )
                });
                self
            }
            pub fn force_instance_key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.force_instance_key = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for force_instance_key: {}",
                        e
                    )
                });
                self
            }
            pub fn internal_port<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.internal_port = value.try_into().map_err(|e| {
                    format!("error converting supplied value for internal_port: {}", e)
                });
                self
            }
            pub fn min_machines_running<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.min_machines_running = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for min_machines_running: {}",
                        e
                    )
                });
                self
            }
            pub fn ports<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FlyMachinePort>>,
                T::Error: std::fmt::Display,
            {
                self.ports = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ports: {}", e));
                self
            }
            pub fn protocol<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.protocol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for protocol: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineService> for super::FlyMachineService {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyMachineService) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    autostart: value.autostart?,
                    autostop: value.autostop?,
                    checks: value.checks?,
                    concurrency: value.concurrency?,
                    force_instance_description: value.force_instance_description?,
                    force_instance_key: value.force_instance_key?,
                    internal_port: value.internal_port?,
                    min_machines_running: value.min_machines_running?,
                    ports: value.ports?,
                    protocol: value.protocol?,
                })
            }
        }

        impl From<super::FlyMachineService> for FlyMachineService {
            fn from(value: super::FlyMachineService) -> Self {
                Self {
                    autostart: Ok(value.autostart),
                    autostop: Ok(value.autostop),
                    checks: Ok(value.checks),
                    concurrency: Ok(value.concurrency),
                    force_instance_description: Ok(value.force_instance_description),
                    force_instance_key: Ok(value.force_instance_key),
                    internal_port: Ok(value.internal_port),
                    min_machines_running: Ok(value.min_machines_running),
                    ports: Ok(value.ports),
                    protocol: Ok(value.protocol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyMachineServiceConcurrency {
            hard_limit: Result<Option<i64>, String>,
            soft_limit: Result<Option<i64>, String>,
            type_: Result<Option<String>, String>,
        }

        impl Default for FlyMachineServiceConcurrency {
            fn default() -> Self {
                Self {
                    hard_limit: Ok(Default::default()),
                    soft_limit: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl FlyMachineServiceConcurrency {
            pub fn hard_limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.hard_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hard_limit: {}", e));
                self
            }
            pub fn soft_limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.soft_limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for soft_limit: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyMachineServiceConcurrency> for super::FlyMachineServiceConcurrency {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FlyMachineServiceConcurrency,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    hard_limit: value.hard_limit?,
                    soft_limit: value.soft_limit?,
                    type_: value.type_?,
                })
            }
        }

        impl From<super::FlyMachineServiceConcurrency> for FlyMachineServiceConcurrency {
            fn from(value: super::FlyMachineServiceConcurrency) -> Self {
                Self {
                    hard_limit: Ok(value.hard_limit),
                    soft_limit: Ok(value.soft_limit),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyProxyProtoOptions {
            version: Result<Option<String>, String>,
        }

        impl Default for FlyProxyProtoOptions {
            fn default() -> Self {
                Self {
                    version: Ok(Default::default()),
                }
            }
        }

        impl FlyProxyProtoOptions {
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyProxyProtoOptions> for super::FlyProxyProtoOptions {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FlyProxyProtoOptions,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    version: value.version?,
                })
            }
        }

        impl From<super::FlyProxyProtoOptions> for FlyProxyProtoOptions {
            fn from(value: super::FlyProxyProtoOptions) -> Self {
                Self {
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyStatic {
            guest_path: Result<String, String>,
            tigris_bucket: Result<Option<String>, String>,
            url_prefix: Result<String, String>,
        }

        impl Default for FlyStatic {
            fn default() -> Self {
                Self {
                    guest_path: Err("no value supplied for guest_path".to_string()),
                    tigris_bucket: Ok(Default::default()),
                    url_prefix: Err("no value supplied for url_prefix".to_string()),
                }
            }
        }

        impl FlyStatic {
            pub fn guest_path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.guest_path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for guest_path: {}", e));
                self
            }
            pub fn tigris_bucket<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tigris_bucket = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tigris_bucket: {}", e)
                });
                self
            }
            pub fn url_prefix<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.url_prefix = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url_prefix: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyStatic> for super::FlyStatic {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyStatic) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    guest_path: value.guest_path?,
                    tigris_bucket: value.tigris_bucket?,
                    url_prefix: value.url_prefix?,
                })
            }
        }

        impl From<super::FlyStatic> for FlyStatic {
            fn from(value: super::FlyStatic) -> Self {
                Self {
                    guest_path: Ok(value.guest_path),
                    tigris_bucket: Ok(value.tigris_bucket),
                    url_prefix: Ok(value.url_prefix),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyStopConfig {
            signal: Result<Option<String>, String>,
            timeout: Result<Option<super::FlyDuration>, String>,
        }

        impl Default for FlyStopConfig {
            fn default() -> Self {
                Self {
                    signal: Ok(Default::default()),
                    timeout: Ok(Default::default()),
                }
            }
        }

        impl FlyStopConfig {
            pub fn signal<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.signal = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for signal: {}", e));
                self
            }
            pub fn timeout<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyDuration>>,
                T::Error: std::fmt::Display,
            {
                self.timeout = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timeout: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyStopConfig> for super::FlyStopConfig {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyStopConfig) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    signal: value.signal?,
                    timeout: value.timeout?,
                })
            }
        }

        impl From<super::FlyStopConfig> for FlyStopConfig {
            fn from(value: super::FlyStopConfig) -> Self {
                Self {
                    signal: Ok(value.signal),
                    timeout: Ok(value.timeout),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FlyTlsOptions {
            alpn: Result<Vec<String>, String>,
            default_self_signed: Result<Option<bool>, String>,
            versions: Result<Vec<String>, String>,
        }

        impl Default for FlyTlsOptions {
            fn default() -> Self {
                Self {
                    alpn: Ok(Default::default()),
                    default_self_signed: Ok(Default::default()),
                    versions: Ok(Default::default()),
                }
            }
        }

        impl FlyTlsOptions {
            pub fn alpn<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.alpn = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alpn: {}", e));
                self
            }
            pub fn default_self_signed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.default_self_signed = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for default_self_signed: {}",
                        e
                    )
                });
                self
            }
            pub fn versions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.versions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for versions: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FlyTlsOptions> for super::FlyTlsOptions {
            type Error = super::error::ConversionError;
            fn try_from(value: FlyTlsOptions) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    alpn: value.alpn?,
                    default_self_signed: value.default_self_signed?,
                    versions: value.versions?,
                })
            }
        }

        impl From<super::FlyTlsOptions> for FlyTlsOptions {
            fn from(value: super::FlyTlsOptions) -> Self {
                Self {
                    alpn: Ok(value.alpn),
                    default_self_signed: Ok(value.default_self_signed),
                    versions: Ok(value.versions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ImageRef {
            digest: Result<Option<String>, String>,
            labels: Result<std::collections::HashMap<String, String>, String>,
            registry: Result<Option<String>, String>,
            repository: Result<Option<String>, String>,
            tag: Result<Option<String>, String>,
        }

        impl Default for ImageRef {
            fn default() -> Self {
                Self {
                    digest: Ok(Default::default()),
                    labels: Ok(Default::default()),
                    registry: Ok(Default::default()),
                    repository: Ok(Default::default()),
                    tag: Ok(Default::default()),
                }
            }
        }

        impl ImageRef {
            pub fn digest<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.digest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for digest: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, String>>,
                T::Error: std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn registry<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.registry = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for registry: {}", e));
                self
            }
            pub fn repository<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.repository = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for repository: {}", e));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ImageRef> for super::ImageRef {
            type Error = super::error::ConversionError;
            fn try_from(value: ImageRef) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    digest: value.digest?,
                    labels: value.labels?,
                    registry: value.registry?,
                    repository: value.repository?,
                    tag: value.tag?,
                })
            }
        }

        impl From<super::ImageRef> for ImageRef {
            fn from(value: super::ImageRef) -> Self {
                Self {
                    digest: Ok(value.digest),
                    labels: Ok(value.labels),
                    registry: Ok(value.registry),
                    repository: Ok(value.repository),
                    tag: Ok(value.tag),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Lease {
            description: Result<Option<String>, String>,
            expires_at: Result<Option<i64>, String>,
            nonce: Result<Option<String>, String>,
            owner: Result<Option<String>, String>,
            version: Result<Option<String>, String>,
        }

        impl Default for Lease {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    expires_at: Ok(Default::default()),
                    nonce: Ok(Default::default()),
                    owner: Ok(Default::default()),
                    version: Ok(Default::default()),
                }
            }
        }

        impl Lease {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {}", e));
                self
            }
            pub fn nonce<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.nonce = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nonce: {}", e));
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Lease> for super::Lease {
            type Error = super::error::ConversionError;
            fn try_from(value: Lease) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    description: value.description?,
                    expires_at: value.expires_at?,
                    nonce: value.nonce?,
                    owner: value.owner?,
                    version: value.version?,
                })
            }
        }

        impl From<super::Lease> for Lease {
            fn from(value: super::Lease) -> Self {
                Self {
                    description: Ok(value.description),
                    expires_at: Ok(value.expires_at),
                    nonce: Ok(value.nonce),
                    owner: Ok(value.owner),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListApp {
            id: Result<Option<String>, String>,
            machine_count: Result<Option<i64>, String>,
            name: Result<Option<String>, String>,
            network: Result<serde_json::Map<String, serde_json::Value>, String>,
        }

        impl Default for ListApp {
            fn default() -> Self {
                Self {
                    id: Ok(Default::default()),
                    machine_count: Ok(Default::default()),
                    name: Ok(Default::default()),
                    network: Ok(Default::default()),
                }
            }
        }

        impl ListApp {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn machine_count<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.machine_count = value.try_into().map_err(|e| {
                    format!("error converting supplied value for machine_count: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn network<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self.network = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for network: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListApp> for super::ListApp {
            type Error = super::error::ConversionError;
            fn try_from(value: ListApp) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    machine_count: value.machine_count?,
                    name: value.name?,
                    network: value.network?,
                })
            }
        }

        impl From<super::ListApp> for ListApp {
            fn from(value: super::ListApp) -> Self {
                Self {
                    id: Ok(value.id),
                    machine_count: Ok(value.machine_count),
                    name: Ok(value.name),
                    network: Ok(value.network),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListAppsResponse {
            apps: Result<Vec<super::ListApp>, String>,
            total_apps: Result<Option<i64>, String>,
        }

        impl Default for ListAppsResponse {
            fn default() -> Self {
                Self {
                    apps: Ok(Default::default()),
                    total_apps: Ok(Default::default()),
                }
            }
        }

        impl ListAppsResponse {
            pub fn apps<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::ListApp>>,
                T::Error: std::fmt::Display,
            {
                self.apps = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for apps: {}", e));
                self
            }
            pub fn total_apps<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.total_apps = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_apps: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListAppsResponse> for super::ListAppsResponse {
            type Error = super::error::ConversionError;
            fn try_from(value: ListAppsResponse) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    apps: value.apps?,
                    total_apps: value.total_apps?,
                })
            }
        }

        impl From<super::ListAppsResponse> for ListAppsResponse {
            fn from(value: super::ListAppsResponse) -> Self {
                Self {
                    apps: Ok(value.apps),
                    total_apps: Ok(value.total_apps),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListenSocket {
            address: Result<Option<String>, String>,
            proto: Result<Option<String>, String>,
        }

        impl Default for ListenSocket {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    proto: Ok(Default::default()),
                }
            }
        }

        impl ListenSocket {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn proto<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.proto = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for proto: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListenSocket> for super::ListenSocket {
            type Error = super::error::ConversionError;
            fn try_from(value: ListenSocket) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    address: value.address?,
                    proto: value.proto?,
                })
            }
        }

        impl From<super::ListenSocket> for ListenSocket {
            fn from(value: super::ListenSocket) -> Self {
                Self {
                    address: Ok(value.address),
                    proto: Ok(value.proto),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Machine {
            checks: Result<Vec<super::CheckStatus>, String>,
            config: Result<Option<super::FlyMachineConfig>, String>,
            created_at: Result<Option<String>, String>,
            events: Result<Vec<super::MachineEvent>, String>,
            host_status: Result<Option<super::MachineHostStatus>, String>,
            id: Result<Option<String>, String>,
            image_ref: Result<Option<super::ImageRef>, String>,
            instance_id: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            nonce: Result<Option<String>, String>,
            private_ip: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            state: Result<Option<String>, String>,
            updated_at: Result<Option<String>, String>,
        }

        impl Default for Machine {
            fn default() -> Self {
                Self {
                    checks: Ok(Default::default()),
                    config: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    events: Ok(Default::default()),
                    host_status: Ok(Default::default()),
                    id: Ok(Default::default()),
                    image_ref: Ok(Default::default()),
                    instance_id: Ok(Default::default()),
                    name: Ok(Default::default()),
                    nonce: Ok(Default::default()),
                    private_ip: Ok(Default::default()),
                    region: Ok(Default::default()),
                    state: Ok(Default::default()),
                    updated_at: Ok(Default::default()),
                }
            }
        }

        impl Machine {
            pub fn checks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::CheckStatus>>,
                T::Error: std::fmt::Display,
            {
                self.checks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for checks: {}", e));
                self
            }
            pub fn config<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineConfig>>,
                T::Error: std::fmt::Display,
            {
                self.config = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for config: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn events<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::MachineEvent>>,
                T::Error: std::fmt::Display,
            {
                self.events = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for events: {}", e));
                self
            }
            pub fn host_status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::MachineHostStatus>>,
                T::Error: std::fmt::Display,
            {
                self.host_status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for host_status: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn image_ref<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::ImageRef>>,
                T::Error: std::fmt::Display,
            {
                self.image_ref = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for image_ref: {}", e));
                self
            }
            pub fn instance_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.instance_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instance_id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn nonce<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.nonce = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nonce: {}", e));
                self
            }
            pub fn private_ip<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.private_ip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for private_ip: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Machine> for super::Machine {
            type Error = super::error::ConversionError;
            fn try_from(value: Machine) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    checks: value.checks?,
                    config: value.config?,
                    created_at: value.created_at?,
                    events: value.events?,
                    host_status: value.host_status?,
                    id: value.id?,
                    image_ref: value.image_ref?,
                    instance_id: value.instance_id?,
                    name: value.name?,
                    nonce: value.nonce?,
                    private_ip: value.private_ip?,
                    region: value.region?,
                    state: value.state?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl From<super::Machine> for Machine {
            fn from(value: super::Machine) -> Self {
                Self {
                    checks: Ok(value.checks),
                    config: Ok(value.config),
                    created_at: Ok(value.created_at),
                    events: Ok(value.events),
                    host_status: Ok(value.host_status),
                    id: Ok(value.id),
                    image_ref: Ok(value.image_ref),
                    instance_id: Ok(value.instance_id),
                    name: Ok(value.name),
                    nonce: Ok(value.nonce),
                    private_ip: Ok(value.private_ip),
                    region: Ok(value.region),
                    state: Ok(value.state),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MachineEvent {
            id: Result<Option<String>, String>,
            request: Result<serde_json::Map<String, serde_json::Value>, String>,
            source: Result<Option<String>, String>,
            status: Result<Option<String>, String>,
            timestamp: Result<Option<i64>, String>,
            type_: Result<Option<String>, String>,
        }

        impl Default for MachineEvent {
            fn default() -> Self {
                Self {
                    id: Ok(Default::default()),
                    request: Ok(Default::default()),
                    source: Ok(Default::default()),
                    status: Ok(Default::default()),
                    timestamp: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl MachineEvent {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn request<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self.request = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request: {}", e));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn timestamp<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.timestamp = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<MachineEvent> for super::MachineEvent {
            type Error = super::error::ConversionError;
            fn try_from(value: MachineEvent) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    request: value.request?,
                    source: value.source?,
                    status: value.status?,
                    timestamp: value.timestamp?,
                    type_: value.type_?,
                })
            }
        }

        impl From<super::MachineEvent> for MachineEvent {
            fn from(value: super::MachineEvent) -> Self {
                Self {
                    id: Ok(value.id),
                    request: Ok(value.request),
                    source: Ok(value.source),
                    status: Ok(value.status),
                    timestamp: Ok(value.timestamp),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MachineExecRequest {
            cmd: Result<Option<String>, String>,
            command: Result<Vec<String>, String>,
            timeout: Result<Option<i64>, String>,
        }

        impl Default for MachineExecRequest {
            fn default() -> Self {
                Self {
                    cmd: Ok(Default::default()),
                    command: Ok(Default::default()),
                    timeout: Ok(Default::default()),
                }
            }
        }

        impl MachineExecRequest {
            pub fn cmd<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.cmd = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cmd: {}", e));
                self
            }
            pub fn command<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.command = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for command: {}", e));
                self
            }
            pub fn timeout<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.timeout = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timeout: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<MachineExecRequest> for super::MachineExecRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: MachineExecRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cmd: value.cmd?,
                    command: value.command?,
                    timeout: value.timeout?,
                })
            }
        }

        impl From<super::MachineExecRequest> for MachineExecRequest {
            fn from(value: super::MachineExecRequest) -> Self {
                Self {
                    cmd: Ok(value.cmd),
                    command: Ok(value.command),
                    timeout: Ok(value.timeout),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MachineVersion {
            user_config: Result<Option<super::FlyMachineConfig>, String>,
            version: Result<Option<String>, String>,
        }

        impl Default for MachineVersion {
            fn default() -> Self {
                Self {
                    user_config: Ok(Default::default()),
                    version: Ok(Default::default()),
                }
            }
        }

        impl MachineVersion {
            pub fn user_config<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineConfig>>,
                T::Error: std::fmt::Display,
            {
                self.user_config = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_config: {}", e));
                self
            }
            pub fn version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for version: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<MachineVersion> for super::MachineVersion {
            type Error = super::error::ConversionError;
            fn try_from(value: MachineVersion) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    user_config: value.user_config?,
                    version: value.version?,
                })
            }
        }

        impl From<super::MachineVersion> for MachineVersion {
            fn from(value: super::MachineVersion) -> Self {
                Self {
                    user_config: Ok(value.user_config),
                    version: Ok(value.version),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Organization {
            name: Result<Option<String>, String>,
            slug: Result<Option<String>, String>,
        }

        impl Default for Organization {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                    slug: Ok(Default::default()),
                }
            }
        }

        impl Organization {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn slug<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.slug = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for slug: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Organization> for super::Organization {
            type Error = super::error::ConversionError;
            fn try_from(value: Organization) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    slug: value.slug?,
                })
            }
        }

        impl From<super::Organization> for Organization {
            fn from(value: super::Organization) -> Self {
                Self {
                    name: Ok(value.name),
                    slug: Ok(value.slug),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ProcessStat {
            command: Result<Option<String>, String>,
            cpu: Result<Option<i64>, String>,
            directory: Result<Option<String>, String>,
            listen_sockets: Result<Vec<super::ListenSocket>, String>,
            pid: Result<Option<i64>, String>,
            rss: Result<Option<i64>, String>,
            rtime: Result<Option<i64>, String>,
            stime: Result<Option<i64>, String>,
        }

        impl Default for ProcessStat {
            fn default() -> Self {
                Self {
                    command: Ok(Default::default()),
                    cpu: Ok(Default::default()),
                    directory: Ok(Default::default()),
                    listen_sockets: Ok(Default::default()),
                    pid: Ok(Default::default()),
                    rss: Ok(Default::default()),
                    rtime: Ok(Default::default()),
                    stime: Ok(Default::default()),
                }
            }
        }

        impl ProcessStat {
            pub fn command<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.command = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for command: {}", e));
                self
            }
            pub fn cpu<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.cpu = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cpu: {}", e));
                self
            }
            pub fn directory<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.directory = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for directory: {}", e));
                self
            }
            pub fn listen_sockets<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::ListenSocket>>,
                T::Error: std::fmt::Display,
            {
                self.listen_sockets = value.try_into().map_err(|e| {
                    format!("error converting supplied value for listen_sockets: {}", e)
                });
                self
            }
            pub fn pid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.pid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pid: {}", e));
                self
            }
            pub fn rss<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.rss = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rss: {}", e));
                self
            }
            pub fn rtime<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.rtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rtime: {}", e));
                self
            }
            pub fn stime<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.stime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stime: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ProcessStat> for super::ProcessStat {
            type Error = super::error::ConversionError;
            fn try_from(value: ProcessStat) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    command: value.command?,
                    cpu: value.cpu?,
                    directory: value.directory?,
                    listen_sockets: value.listen_sockets?,
                    pid: value.pid?,
                    rss: value.rss?,
                    rtime: value.rtime?,
                    stime: value.stime?,
                })
            }
        }

        impl From<super::ProcessStat> for ProcessStat {
            fn from(value: super::ProcessStat) -> Self {
                Self {
                    command: Ok(value.command),
                    cpu: Ok(value.cpu),
                    directory: Ok(value.directory),
                    listen_sockets: Ok(value.listen_sockets),
                    pid: Ok(value.pid),
                    rss: Ok(value.rss),
                    rtime: Ok(value.rtime),
                    stime: Ok(value.stime),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SignalRequest {
            signal: Result<Option<super::SignalRequestSignal>, String>,
        }

        impl Default for SignalRequest {
            fn default() -> Self {
                Self {
                    signal: Ok(Default::default()),
                }
            }
        }

        impl SignalRequest {
            pub fn signal<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::SignalRequestSignal>>,
                T::Error: std::fmt::Display,
            {
                self.signal = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for signal: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<SignalRequest> for super::SignalRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: SignalRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    signal: value.signal?,
                })
            }
        }

        impl From<super::SignalRequest> for SignalRequest {
            fn from(value: super::SignalRequest) -> Self {
                Self {
                    signal: Ok(value.signal),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct StopRequest {
            signal: Result<Option<String>, String>,
            timeout: Result<Option<super::FlyDuration>, String>,
        }

        impl Default for StopRequest {
            fn default() -> Self {
                Self {
                    signal: Ok(Default::default()),
                    timeout: Ok(Default::default()),
                }
            }
        }

        impl StopRequest {
            pub fn signal<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.signal = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for signal: {}", e));
                self
            }
            pub fn timeout<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyDuration>>,
                T::Error: std::fmt::Display,
            {
                self.timeout = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timeout: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<StopRequest> for super::StopRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: StopRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    signal: value.signal?,
                    timeout: value.timeout?,
                })
            }
        }

        impl From<super::StopRequest> for StopRequest {
            fn from(value: super::StopRequest) -> Self {
                Self {
                    signal: Ok(value.signal),
                    timeout: Ok(value.timeout),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateMachineRequest {
            config: Result<Option<super::FlyMachineConfig>, String>,
            current_version: Result<Option<String>, String>,
            lease_ttl: Result<Option<i64>, String>,
            lsvd: Result<Option<bool>, String>,
            name: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            skip_launch: Result<Option<bool>, String>,
            skip_service_registration: Result<Option<bool>, String>,
        }

        impl Default for UpdateMachineRequest {
            fn default() -> Self {
                Self {
                    config: Ok(Default::default()),
                    current_version: Ok(Default::default()),
                    lease_ttl: Ok(Default::default()),
                    lsvd: Ok(Default::default()),
                    name: Ok(Default::default()),
                    region: Ok(Default::default()),
                    skip_launch: Ok(Default::default()),
                    skip_service_registration: Ok(Default::default()),
                }
            }
        }

        impl UpdateMachineRequest {
            pub fn config<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::FlyMachineConfig>>,
                T::Error: std::fmt::Display,
            {
                self.config = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for config: {}", e));
                self
            }
            pub fn current_version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.current_version = value.try_into().map_err(|e| {
                    format!("error converting supplied value for current_version: {}", e)
                });
                self
            }
            pub fn lease_ttl<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.lease_ttl = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lease_ttl: {}", e));
                self
            }
            pub fn lsvd<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.lsvd = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lsvd: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn skip_launch<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_launch = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for skip_launch: {}", e));
                self
            }
            pub fn skip_service_registration<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_service_registration = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_service_registration: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<UpdateMachineRequest> for super::UpdateMachineRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UpdateMachineRequest,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    config: value.config?,
                    current_version: value.current_version?,
                    lease_ttl: value.lease_ttl?,
                    lsvd: value.lsvd?,
                    name: value.name?,
                    region: value.region?,
                    skip_launch: value.skip_launch?,
                    skip_service_registration: value.skip_service_registration?,
                })
            }
        }

        impl From<super::UpdateMachineRequest> for UpdateMachineRequest {
            fn from(value: super::UpdateMachineRequest) -> Self {
                Self {
                    config: Ok(value.config),
                    current_version: Ok(value.current_version),
                    lease_ttl: Ok(value.lease_ttl),
                    lsvd: Ok(value.lsvd),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    skip_launch: Ok(value.skip_launch),
                    skip_service_registration: Ok(value.skip_service_registration),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateVolumeRequest {
            auto_backup_enabled: Result<Option<bool>, String>,
            snapshot_retention: Result<Option<i64>, String>,
        }

        impl Default for UpdateVolumeRequest {
            fn default() -> Self {
                Self {
                    auto_backup_enabled: Ok(Default::default()),
                    snapshot_retention: Ok(Default::default()),
                }
            }
        }

        impl UpdateVolumeRequest {
            pub fn auto_backup_enabled<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.auto_backup_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_backup_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn snapshot_retention<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.snapshot_retention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snapshot_retention: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<UpdateVolumeRequest> for super::UpdateVolumeRequest {
            type Error = super::error::ConversionError;
            fn try_from(value: UpdateVolumeRequest) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    auto_backup_enabled: value.auto_backup_enabled?,
                    snapshot_retention: value.snapshot_retention?,
                })
            }
        }

        impl From<super::UpdateVolumeRequest> for UpdateVolumeRequest {
            fn from(value: super::UpdateVolumeRequest) -> Self {
                Self {
                    auto_backup_enabled: Ok(value.auto_backup_enabled),
                    snapshot_retention: Ok(value.snapshot_retention),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Volume {
            attached_alloc_id: Result<Option<String>, String>,
            attached_machine_id: Result<Option<String>, String>,
            auto_backup_enabled: Result<Option<bool>, String>,
            block_size: Result<Option<i64>, String>,
            blocks: Result<Option<i64>, String>,
            blocks_avail: Result<Option<i64>, String>,
            blocks_free: Result<Option<i64>, String>,
            created_at: Result<Option<String>, String>,
            encrypted: Result<Option<bool>, String>,
            fstype: Result<Option<String>, String>,
            host_status: Result<Option<super::VolumeHostStatus>, String>,
            id: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            region: Result<Option<String>, String>,
            size_gb: Result<Option<i64>, String>,
            snapshot_retention: Result<Option<i64>, String>,
            state: Result<Option<String>, String>,
            zone: Result<Option<String>, String>,
        }

        impl Default for Volume {
            fn default() -> Self {
                Self {
                    attached_alloc_id: Ok(Default::default()),
                    attached_machine_id: Ok(Default::default()),
                    auto_backup_enabled: Ok(Default::default()),
                    block_size: Ok(Default::default()),
                    blocks: Ok(Default::default()),
                    blocks_avail: Ok(Default::default()),
                    blocks_free: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    encrypted: Ok(Default::default()),
                    fstype: Ok(Default::default()),
                    host_status: Ok(Default::default()),
                    id: Ok(Default::default()),
                    name: Ok(Default::default()),
                    region: Ok(Default::default()),
                    size_gb: Ok(Default::default()),
                    snapshot_retention: Ok(Default::default()),
                    state: Ok(Default::default()),
                    zone: Ok(Default::default()),
                }
            }
        }

        impl Volume {
            pub fn attached_alloc_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.attached_alloc_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for attached_alloc_id: {}",
                        e
                    )
                });
                self
            }
            pub fn attached_machine_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.attached_machine_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for attached_machine_id: {}",
                        e
                    )
                });
                self
            }
            pub fn auto_backup_enabled<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.auto_backup_enabled = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auto_backup_enabled: {}",
                        e
                    )
                });
                self
            }
            pub fn block_size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.block_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for block_size: {}", e));
                self
            }
            pub fn blocks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.blocks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for blocks: {}", e));
                self
            }
            pub fn blocks_avail<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.blocks_avail = value.try_into().map_err(|e| {
                    format!("error converting supplied value for blocks_avail: {}", e)
                });
                self
            }
            pub fn blocks_free<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.blocks_free = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for blocks_free: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn encrypted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.encrypted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for encrypted: {}", e));
                self
            }
            pub fn fstype<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.fstype = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fstype: {}", e));
                self
            }
            pub fn host_status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::VolumeHostStatus>>,
                T::Error: std::fmt::Display,
            {
                self.host_status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for host_status: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn size_gb<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size_gb = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_gb: {}", e));
                self
            }
            pub fn snapshot_retention<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.snapshot_retention = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for snapshot_retention: {}",
                        e
                    )
                });
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn zone<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.zone = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for zone: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Volume> for super::Volume {
            type Error = super::error::ConversionError;
            fn try_from(value: Volume) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attached_alloc_id: value.attached_alloc_id?,
                    attached_machine_id: value.attached_machine_id?,
                    auto_backup_enabled: value.auto_backup_enabled?,
                    block_size: value.block_size?,
                    blocks: value.blocks?,
                    blocks_avail: value.blocks_avail?,
                    blocks_free: value.blocks_free?,
                    created_at: value.created_at?,
                    encrypted: value.encrypted?,
                    fstype: value.fstype?,
                    host_status: value.host_status?,
                    id: value.id?,
                    name: value.name?,
                    region: value.region?,
                    size_gb: value.size_gb?,
                    snapshot_retention: value.snapshot_retention?,
                    state: value.state?,
                    zone: value.zone?,
                })
            }
        }

        impl From<super::Volume> for Volume {
            fn from(value: super::Volume) -> Self {
                Self {
                    attached_alloc_id: Ok(value.attached_alloc_id),
                    attached_machine_id: Ok(value.attached_machine_id),
                    auto_backup_enabled: Ok(value.auto_backup_enabled),
                    block_size: Ok(value.block_size),
                    blocks: Ok(value.blocks),
                    blocks_avail: Ok(value.blocks_avail),
                    blocks_free: Ok(value.blocks_free),
                    created_at: Ok(value.created_at),
                    encrypted: Ok(value.encrypted),
                    fstype: Ok(value.fstype),
                    host_status: Ok(value.host_status),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    region: Ok(value.region),
                    size_gb: Ok(value.size_gb),
                    snapshot_retention: Ok(value.snapshot_retention),
                    state: Ok(value.state),
                    zone: Ok(value.zone),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct VolumeSnapshot {
            created_at: Result<Option<String>, String>,
            digest: Result<Option<String>, String>,
            id: Result<Option<String>, String>,
            size: Result<Option<i64>, String>,
            status: Result<Option<String>, String>,
        }

        impl Default for VolumeSnapshot {
            fn default() -> Self {
                Self {
                    created_at: Ok(Default::default()),
                    digest: Ok(Default::default()),
                    id: Ok(Default::default()),
                    size: Ok(Default::default()),
                    status: Ok(Default::default()),
                }
            }
        }

        impl VolumeSnapshot {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn digest<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.digest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for digest: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<VolumeSnapshot> for super::VolumeSnapshot {
            type Error = super::error::ConversionError;
            fn try_from(value: VolumeSnapshot) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    digest: value.digest?,
                    id: value.id?,
                    size: value.size?,
                    status: value.status?,
                })
            }
        }

        impl From<super::VolumeSnapshot> for VolumeSnapshot {
            fn from(value: super::VolumeSnapshot) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    digest: Ok(value.digest),
                    id: Ok(value.id),
                    size: Ok(value.size),
                    status: Ok(value.status),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Machines API
///
///# Introduction
///
///Fly Machines are the compute behind the Fly.io platform. They are fast-launching VMs that can be started and stopped at subsecond speeds. A Machine is the configuration and state for a single VM running on our platform. Every Machine will belong to a Fly App; Apps can have more than one Machine. Read more [here](https://fly.io/docs/machines/).
///
///The Machines REST API allows you to provison and manage Apps, Machines and Volumes on the Fly.io platform. To manage other Fly.io resources like organizations, use the [GraphQL API](https://fly.io/docs/networking/custom-domains-with-fly/#graphql-api-notes).
///
///## Authentication
///
///All requests must include the Fly API Token in the HTTP Headers as follows:
///
///```
/// Authorization: Bearer [TOKEN]
/// ```
///
///You can get your API token using [flyctl](https://fly.io/docs/hands-on/install-flyctl/) by running `fly auth token`
///
///## Base URL
///
///The easiest (and recommended) way to connect to the Machines API is to use the public `api.machines.dev` endpoint, a simpler and more performant alternative to connecting over WireGuard. You can still access your Machines directly over a WireGuard VPN, and use the private Machines API endpoint: `http://_api.internal:4280`. This method requires more setup.
///
///Follow the [instructions](https://fly.io/docs/networking/private-networking/#private-network-vpn) to set up a permanent WireGuard connection to your Fly.io [IPv6 private network](https://fly.io/docs/networking/private-networking/). Once you’re connected, Fly internal DNS should expose the Machines API endpoint at: `http://_api.internal:4280`
///
///## Response Codes
///
///The API uses conventional HTTP status codes to signal whether a request was
/// successful or not.
///
///Typically, 2xx HTTP status codes denote successful operations, 4xx codes
/// imply failures related to the user, and 5xx codes suggest problems with the
/// infrastructure.
///
///| Status | Description                                 |
///| :----: | ------------------------------------------- |
///| `200`  | Successful request.                         |
///| `201`  | Created successfully.                       |
///| `202`  | Successful request. No content.             |
///| `400`  | Check that the parameters were correct.     |
///| `401`  | The API key used was missing or invalid.    |
///| `404`  | The resource was not found.                 |
///| `5xx`  | Indicates an error with Fly.io API servers. |
///
///
///Version: 1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable. `token` is the token 
    /// received from `flyctl auth token`.
    pub fn new(baseurl: &str, token: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            let mut headers = HeaderMap::new();
            headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .default_headers(headers)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0"
    }
}

impl Client {
    ///List Apps
    ///
    ///List all apps with the ability to filter by organization slug.
    ///
    ///
    ///Sends a `GET` request to `/apps`
    ///
    ///Arguments:
    /// - `org_slug`: The org slug, or 'personal', to filter apps
    ///```ignore
    /// let response = client.apps_list()
    ///    .org_slug(org_slug)
    ///    .send()
    ///    .await;
    /// ```
    pub fn apps_list(&self) -> builder::AppsList {
        builder::AppsList::new(self)
    }

    ///Create App
    ///
    ///Create an app with the specified details in the request body.
    ///
    ///
    ///Sends a `POST` request to `/apps`
    ///
    ///Arguments:
    /// - `body`: App body
    ///```ignore
    /// let response = client.apps_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn apps_create(&self) -> builder::AppsCreate {
        builder::AppsCreate::new(self)
    }

    ///Get App
    ///
    ///Retrieve details about a specific app by its name.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    ///```ignore
    /// let response = client.apps_show()
    ///    .app_name(app_name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn apps_show(&self) -> builder::AppsShow {
        builder::AppsShow::new(self)
    }

    ///Destroy App
    ///
    ///Delete an app by its name.
    ///
    ///
    ///Sends a `DELETE` request to `/apps/{app_name}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    ///```ignore
    /// let response = client.apps_delete()
    ///    .app_name(app_name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn apps_delete(&self) -> builder::AppsDelete {
        builder::AppsDelete::new(self)
    }

    ///List Machines
    ///
    ///List all Machines associated with a specific app, with optional filters
    /// for including deleted Machines and filtering by region.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/machines`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `include_deleted`: Include deleted machines
    /// - `region`: Region filter
    ///```ignore
    /// let response = client.machines_list()
    ///    .app_name(app_name)
    ///    .include_deleted(include_deleted)
    ///    .region(region)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_list(&self) -> builder::MachinesList {
        builder::MachinesList::new(self)
    }

    ///Create Machine
    ///
    ///Create a Machine within a specific app using the details provided in the
    /// request body.
    ///
    ///**Important**: This request can fail, and you’re responsible for
    /// handling that failure. If you ask for a large Machine, or a Machine in a
    /// region we happen to be at capacity for, you might need to retry the
    /// request, or to fall back to another region. If you’re working directly
    /// with the Machines API, you’re taking some responsibility for your own
    /// orchestration!
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/machines`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `body`: Create machine request
    ///```ignore
    /// let response = client.machines_create()
    ///    .app_name(app_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_create(&self) -> builder::MachinesCreate {
        builder::MachinesCreate::new(self)
    }

    ///Get Machine
    ///
    ///Get details of a specific Machine within an app by the Machine ID.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_show()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_show(&self) -> builder::MachinesShow {
        builder::MachinesShow::new(self)
    }

    ///Update Machine
    ///
    ///Update a Machine's configuration using the details provided in the
    /// request body.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/machines/{machine_id}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `body`: Request body
    ///```ignore
    /// let response = client.machines_update()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_update(&self) -> builder::MachinesUpdate {
        builder::MachinesUpdate::new(self)
    }

    ///Destroy Machine
    ///
    ///Delete a specific Machine within an app by Machine ID, with an optional
    /// force parameter to force kill the Machine if it's running.
    ///
    ///
    ///Sends a `DELETE` request to `/apps/{app_name}/machines/{machine_id}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `force`: Force kill the machine if it's running
    ///```ignore
    /// let response = client.machines_delete()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .force(force)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_delete(&self) -> builder::MachinesDelete {
        builder::MachinesDelete::new(self)
    }

    ///Cordon Machine
    ///
    ///“Cordoning” a Machine refers to disabling its services, so the Fly Proxy
    /// won’t route requests to it. In flyctl this is used by blue/green
    /// deployments; one set of Machines is started up with services disabled,
    /// and when they are all healthy, the services are enabled on the new
    /// Machines and disabled on the old ones.
    ///
    ///
    ///Sends a `POST` request to
    /// `/apps/{app_name}/machines/{machine_id}/cordon`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_cordon()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_cordon(&self) -> builder::MachinesCordon {
        builder::MachinesCordon::new(self)
    }

    ///List Events
    ///
    ///List all events associated with a specific Machine within an app.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}/events`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_list_events()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_list_events(&self) -> builder::MachinesListEvents {
        builder::MachinesListEvents::new(self)
    }

    ///Execute Command
    ///
    ///Execute a command on a specific Machine and return the raw command
    /// output bytes.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/machines/{machine_id}/exec`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `body`: Request body
    ///```ignore
    /// let response = client.machines_exec()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_exec(&self) -> builder::MachinesExec {
        builder::MachinesExec::new(self)
    }

    ///Get Lease
    ///
    ///Retrieve the current lease of a specific Machine within an app. Machine
    /// leases can be used to obtain an exclusive lock on modifying a Machine.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}/lease`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_show_lease()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_show_lease(&self) -> builder::MachinesShowLease {
        builder::MachinesShowLease::new(self)
    }

    ///Create Lease
    ///
    ///Create a lease for a specific Machine within an app using the details
    /// provided in the request body. Machine leases can be used to obtain an
    /// exclusive lock on modifying a Machine.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/machines/{machine_id}/lease`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `fly_machine_lease_nonce`: Existing lease nonce to refresh by ttl,
    ///   empty or non-existent to create a new lease
    /// - `body`: Request body
    ///```ignore
    /// let response = client.machines_create_lease()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .fly_machine_lease_nonce(fly_machine_lease_nonce)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_create_lease(&self) -> builder::MachinesCreateLease {
        builder::MachinesCreateLease::new(self)
    }

    ///Release Lease
    ///
    ///Release the lease of a specific Machine within an app. Machine leases
    /// can be used to obtain an exclusive lock on modifying a Machine.
    ///
    ///
    ///Sends a `DELETE` request to
    /// `/apps/{app_name}/machines/{machine_id}/lease`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `fly_machine_lease_nonce`: Existing lease nonce
    ///```ignore
    /// let response = client.machines_release_lease()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .fly_machine_lease_nonce(fly_machine_lease_nonce)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_release_lease(&self) -> builder::MachinesReleaseLease {
        builder::MachinesReleaseLease::new(self)
    }

    ///Get Metadata
    ///
    ///Retrieve metadata for a specific Machine within an app.
    ///
    ///
    ///Sends a `GET` request to
    /// `/apps/{app_name}/machines/{machine_id}/metadata`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_show_metadata()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_show_metadata(&self) -> builder::MachinesShowMetadata {
        builder::MachinesShowMetadata::new(self)
    }

    ///Update Metadata
    ///
    ///Update metadata for a specific machine within an app by providing a
    /// metadata key.
    ///
    ///
    ///Sends a `POST` request to
    /// `/apps/{app_name}/machines/{machine_id}/metadata/{key}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `key`: Metadata Key
    ///```ignore
    /// let response = client.machines_update_metadata()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .key(key)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_update_metadata(&self) -> builder::MachinesUpdateMetadata {
        builder::MachinesUpdateMetadata::new(self)
    }

    ///Delete Metadata
    ///
    ///Delete metadata for a specific Machine within an app by providing a
    /// metadata key.
    ///
    ///
    ///Sends a `DELETE` request to
    /// `/apps/{app_name}/machines/{machine_id}/metadata/{key}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `key`: Metadata Key
    ///```ignore
    /// let response = client.machines_delete_metadata()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .key(key)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_delete_metadata(&self) -> builder::MachinesDeleteMetadata {
        builder::MachinesDeleteMetadata::new(self)
    }

    ///List Processes
    ///
    ///List all processes running on a specific Machine within an app, with
    /// optional sorting parameters.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}/ps`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `order`: Order
    /// - `sort_by`: Sort by
    ///```ignore
    /// let response = client.machines_list_processes()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .order(order)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_list_processes(&self) -> builder::MachinesListProcesses {
        builder::MachinesListProcesses::new(self)
    }

    ///Restart Machine
    ///
    ///Restart a specific Machine within an app, with an optional timeout
    /// parameter.
    ///
    ///
    ///Sends a `POST` request to
    /// `/apps/{app_name}/machines/{machine_id}/restart`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `signal`: Unix signal name
    /// - `timeout`: Restart timeout as a Go duration string or number of
    ///   seconds
    ///```ignore
    /// let response = client.machines_restart()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .signal(signal)
    ///    .timeout(timeout)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_restart(&self) -> builder::MachinesRestart {
        builder::MachinesRestart::new(self)
    }

    ///Signal Machine
    ///
    ///Send a signal to a specific Machine within an app using the details
    /// provided in the request body.
    ///
    ///
    ///Sends a `POST` request to
    /// `/apps/{app_name}/machines/{machine_id}/signal`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `body`: Request body
    ///```ignore
    /// let response = client.machines_signal()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_signal(&self) -> builder::MachinesSignal {
        builder::MachinesSignal::new(self)
    }

    ///Start Machine
    ///
    ///Start a specific Machine within an app.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/machines/{machine_id}/start`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_start()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_start(&self) -> builder::MachinesStart {
        builder::MachinesStart::new(self)
    }

    ///Stop Machine
    ///
    ///Stop a specific Machine within an app, with an optional request body to
    /// specify signal and timeout.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/machines/{machine_id}/stop`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `body`: Optional request body
    ///```ignore
    /// let response = client.machines_stop()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_stop(&self) -> builder::MachinesStop {
        builder::MachinesStop::new(self)
    }

    ///Uncordon Machine
    ///
    ///“Cordoning” a Machine refers to disabling its services, so the Fly Proxy
    /// won’t route requests to it. In flyctl this is used by blue/green
    /// deployments; one set of Machines is started up with services disabled,
    /// and when they are all healthy, the services are enabled on the new
    /// Machines and disabled on the old ones.
    ///
    ///
    ///Sends a `POST` request to
    /// `/apps/{app_name}/machines/{machine_id}/uncordon`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_uncordon()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_uncordon(&self) -> builder::MachinesUncordon {
        builder::MachinesUncordon::new(self)
    }

    ///List Versions
    ///
    ///List all versions of the configuration for a specific Machine within an
    /// app.
    ///
    ///
    ///Sends a `GET` request to
    /// `/apps/{app_name}/machines/{machine_id}/versions`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    ///```ignore
    /// let response = client.machines_list_versions()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_list_versions(&self) -> builder::MachinesListVersions {
        builder::MachinesListVersions::new(self)
    }

    ///Wait for State
    ///
    ///Wait for a Machine to reach a specific state. Specify the desired state with the state parameter. See the [Machine states table](https://fly.io/docs/machines/working-with-machines/#machine-states) for a list of possible states. The default for this parameter is `started`.
    ///
    ///This request will block for up to 60 seconds. Set a shorter timeout with
    /// the timeout parameter.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}/wait`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `machine_id`: Machine ID
    /// - `instance_id`: instance? version? TODO
    /// - `state`: desired state
    /// - `timeout`: wait timeout. default 60s
    ///```ignore
    /// let response = client.machines_wait()
    ///    .app_name(app_name)
    ///    .machine_id(machine_id)
    ///    .instance_id(instance_id)
    ///    .state(state)
    ///    .timeout(timeout)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_wait(&self) -> builder::MachinesWait {
        builder::MachinesWait::new(self)
    }

    ///List Volumes
    ///
    ///List all volumes associated with a specific app.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/volumes`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    ///```ignore
    /// let response = client.volumes_list()
    ///    .app_name(app_name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_list(&self) -> builder::VolumesList {
        builder::VolumesList::new(self)
    }

    ///Create Volume
    ///
    ///Create a volume for a specific app using the details provided in the
    /// request body.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/volumes`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `body`: Request body
    ///```ignore
    /// let response = client.volumes_create()
    ///    .app_name(app_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_create(&self) -> builder::VolumesCreate {
        builder::VolumesCreate::new(self)
    }

    ///Get Volume
    ///
    ///Retrieve details about a specific volume by its ID within an app.
    ///
    ///
    ///Sends a `GET` request to `/apps/{app_name}/volumes/{volume_id}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `volume_id`: Volume ID
    ///```ignore
    /// let response = client.volumes_get_by_id()
    ///    .app_name(app_name)
    ///    .volume_id(volume_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_get_by_id(&self) -> builder::VolumesGetById {
        builder::VolumesGetById::new(self)
    }

    ///Update Volume
    ///
    ///Update a volume's configuration using the details provided in the
    /// request body.
    ///
    ///
    ///Sends a `POST` request to `/apps/{app_name}/volumes/{volume_id}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `volume_id`: Volume ID
    /// - `body`: Request body
    ///```ignore
    /// let response = client.volumes_update()
    ///    .app_name(app_name)
    ///    .volume_id(volume_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_update(&self) -> builder::VolumesUpdate {
        builder::VolumesUpdate::new(self)
    }

    ///Destroy Volume
    ///
    ///Delete a specific volume within an app by volume ID.
    ///
    ///
    ///Sends a `DELETE` request to `/apps/{app_name}/volumes/{volume_id}`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `volume_id`: Volume ID
    ///```ignore
    /// let response = client.volume_delete()
    ///    .app_name(app_name)
    ///    .volume_id(volume_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_delete(&self) -> builder::VolumeDelete {
        builder::VolumeDelete::new(self)
    }

    ///Extend Volume
    ///
    ///Extend a volume's size within an app using the details provided in the
    /// request body.
    ///
    ///
    ///Sends a `PUT` request to `/apps/{app_name}/volumes/{volume_id}/extend`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `volume_id`: Volume ID
    /// - `body`: Request body
    ///```ignore
    /// let response = client.volumes_extend()
    ///    .app_name(app_name)
    ///    .volume_id(volume_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_extend(&self) -> builder::VolumesExtend {
        builder::VolumesExtend::new(self)
    }

    ///List Snapshots
    ///
    ///List all snapshots for a specific volume within an app.
    ///
    ///
    ///Sends a `GET` request to
    /// `/apps/{app_name}/volumes/{volume_id}/snapshots`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `volume_id`: Volume ID
    ///```ignore
    /// let response = client.volumes_list_snapshots()
    ///    .app_name(app_name)
    ///    .volume_id(volume_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_list_snapshots(&self) -> builder::VolumesListSnapshots {
        builder::VolumesListSnapshots::new(self)
    }

    ///Create Snapshot
    ///
    ///Create a snapshot for a specific volume within an app.
    ///
    ///
    ///Sends a `POST` request to
    /// `/apps/{app_name}/volumes/{volume_id}/snapshots`
    ///
    ///Arguments:
    /// - `app_name`: Fly App Name
    /// - `volume_id`: Volume ID
    ///```ignore
    /// let response = client.create_volume_snapshot()
    ///    .app_name(app_name)
    ///    .volume_id(volume_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_volume_snapshot(&self) -> builder::CreateVolumeSnapshot {
        builder::CreateVolumeSnapshot::new(self)
    }

    ///Sends a `POST` request to `/v1/tokens/oidc`
    ///
    ///Arguments:
    /// - `body`: Request body
    ///```ignore
    /// let response = client.machines_get_oidc_token()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_get_oidc_token(&self) -> builder::MachinesGetOidcToken {
        builder::MachinesGetOidcToken::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`Client::apps_list`]
    ///
    ///[`Client::apps_list`]: super::Client::apps_list
    #[derive(Debug, Clone)]
    pub struct AppsList<'a> {
        client: &'a super::Client,
        org_slug: Result<String, String>,
    }

    impl<'a> AppsList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                org_slug: Err("org_slug was not initialized".to_string()),
            }
        }

        pub fn org_slug<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.org_slug = value
                .try_into()
                .map_err(|_| "conversion to `String` for org_slug failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps`
        pub async fn send(self) -> Result<ResponseValue<types::ListAppsResponse>, Error<()>> {
            let Self { client, org_slug } = self;
            let org_slug = org_slug.map_err(Error::InvalidRequest)?;
            let url = format!("{}/apps", client.baseurl,);
            let mut query = Vec::with_capacity(1usize);
            query.push(("org_slug", org_slug.to_string()));
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::apps_create`]
    ///
    ///[`Client::apps_create`]: super::Client::apps_create
    #[derive(Debug, Clone)]
    pub struct AppsCreate<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateAppRequest, String>,
    }

    impl<'a> AppsCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateAppRequest::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateAppRequest>,
            <V as std::convert::TryInto<types::CreateAppRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateAppRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateAppRequest,
            ) -> types::builder::CreateAppRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/apps`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::CreateAppRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/apps", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::apps_show`]
    ///
    ///[`Client::apps_show`]: super::Client::apps_show
    #[derive(Debug, Clone)]
    pub struct AppsShow<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
    }

    impl<'a> AppsShow<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps/{app_name}`
        pub async fn send(self) -> Result<ResponseValue<types::App>, Error<()>> {
            let Self { client, app_name } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::apps_delete`]
    ///
    ///[`Client::apps_delete`]: super::Client::apps_delete
    #[derive(Debug, Clone)]
    pub struct AppsDelete<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
    }

    impl<'a> AppsDelete<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/apps/{app_name}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, app_name } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                202u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_list`]
    ///
    ///[`Client::machines_list`]: super::Client::machines_list
    #[derive(Debug, Clone)]
    pub struct MachinesList<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        include_deleted: Result<Option<bool>, String>,
        region: Result<Option<String>, String>,
    }

    impl<'a> MachinesList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                include_deleted: Ok(None),
                region: Ok(None),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn include_deleted<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.include_deleted = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for include_deleted failed".to_string());
            self
        }

        pub fn region<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.region = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for region failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps/{app_name}/machines`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Machine>>, Error<()>> {
            let Self {
                client,
                app_name,
                include_deleted,
                region,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let include_deleted = include_deleted.map_err(Error::InvalidRequest)?;
            let region = region.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines",
                client.baseurl,
                encode_path(&app_name.to_string()),
            );
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &include_deleted {
                query.push(("include_deleted", v.to_string()));
            }
            if let Some(v) = &region {
                query.push(("region", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_create`]
    ///
    ///[`Client::machines_create`]: super::Client::machines_create
    #[derive(Debug, Clone)]
    pub struct MachinesCreate<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        body: Result<types::builder::CreateMachineRequest, String>,
    }

    impl<'a> MachinesCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                body: Ok(types::builder::CreateMachineRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateMachineRequest>,
            <V as std::convert::TryInto<types::CreateMachineRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `CreateMachineRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateMachineRequest,
            ) -> types::builder::CreateMachineRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/apps/{app_name}/machines`
        pub async fn send(self) -> Result<ResponseValue<types::Machine>, Error<()>> {
            let Self {
                client,
                app_name,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::CreateMachineRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines",
                client.baseurl,
                encode_path(&app_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_show`]
    ///
    ///[`Client::machines_show`]: super::Client::machines_show
    #[derive(Debug, Clone)]
    pub struct MachinesShow<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesShow<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Machine>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_update`]
    ///
    ///[`Client::machines_update`]: super::Client::machines_update
    #[derive(Debug, Clone)]
    pub struct MachinesUpdate<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        body: Result<types::builder::UpdateMachineRequest, String>,
    }

    impl<'a> MachinesUpdate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                body: Ok(types::builder::UpdateMachineRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateMachineRequest>,
            <V as std::convert::TryInto<types::UpdateMachineRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `UpdateMachineRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateMachineRequest,
            ) -> types::builder::UpdateMachineRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/apps/{app_name}/machines/{machine_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Machine>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::UpdateMachineRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_delete`]
    ///
    ///[`Client::machines_delete`]: super::Client::machines_delete
    #[derive(Debug, Clone)]
    pub struct MachinesDelete<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        force: Result<Option<bool>, String>,
    }

    impl<'a> MachinesDelete<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                force: Ok(None),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn force<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.force = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for force failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/apps/{app_name}/machines/{machine_id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                force,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let force = force.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            let mut query = Vec::with_capacity(1usize);
            if let Some(v) = &force {
                query.push(("force", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).query(&query).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_cordon`]
    ///
    ///[`Client::machines_cordon`]: super::Client::machines_cordon
    #[derive(Debug, Clone)]
    pub struct MachinesCordon<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesCordon<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/cordon`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/cordon",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_list_events`]
    ///
    ///[`Client::machines_list_events`]: super::Client::machines_list_events
    #[derive(Debug, Clone)]
    pub struct MachinesListEvents<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesListEvents<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/apps/{app_name}/machines/{machine_id}/events`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::MachineEvent>>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/events",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_exec`]
    ///
    ///[`Client::machines_exec`]: super::Client::machines_exec
    #[derive(Debug, Clone)]
    pub struct MachinesExec<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        body: Result<types::builder::MachineExecRequest, String>,
    }

    impl<'a> MachinesExec<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                body: Ok(types::builder::MachineExecRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::MachineExecRequest>,
            <V as std::convert::TryInto<types::MachineExecRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `MachineExecRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::MachineExecRequest,
            ) -> types::builder::MachineExecRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/exec`
        pub async fn send(self) -> Result<ResponseValue<String>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::MachineExecRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/exec",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_show_lease`]
    ///
    ///[`Client::machines_show_lease`]: super::Client::machines_show_lease
    #[derive(Debug, Clone)]
    pub struct MachinesShowLease<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesShowLease<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/apps/{app_name}/machines/{machine_id}/lease`
        pub async fn send(self) -> Result<ResponseValue<types::Lease>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/lease",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_create_lease`]
    ///
    ///[`Client::machines_create_lease`]: super::Client::machines_create_lease
    #[derive(Debug, Clone)]
    pub struct MachinesCreateLease<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        fly_machine_lease_nonce: Result<Option<String>, String>,
        body: Result<types::builder::CreateLeaseRequest, String>,
    }

    impl<'a> MachinesCreateLease<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                fly_machine_lease_nonce: Ok(None),
                body: Ok(types::builder::CreateLeaseRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn fly_machine_lease_nonce<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.fly_machine_lease_nonce = value.try_into().map(Some).map_err(|_| {
                "conversion to `String` for fly_machine_lease_nonce failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateLeaseRequest>,
            <V as std::convert::TryInto<types::CreateLeaseRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateLeaseRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateLeaseRequest,
            ) -> types::builder::CreateLeaseRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/lease`
        pub async fn send(self) -> Result<ResponseValue<types::Lease>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                fly_machine_lease_nonce,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let fly_machine_lease_nonce = fly_machine_lease_nonce.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::CreateLeaseRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/lease",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = fly_machine_lease_nonce {
                header_map.append("fly-machine-lease-nonce", HeaderValue::try_from(v)?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_release_lease`]
    ///
    ///[`Client::machines_release_lease`]: super::Client::machines_release_lease
    #[derive(Debug, Clone)]
    pub struct MachinesReleaseLease<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        fly_machine_lease_nonce: Result<String, String>,
    }

    impl<'a> MachinesReleaseLease<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                fly_machine_lease_nonce: Err(
                    "fly_machine_lease_nonce was not initialized".to_string()
                ),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn fly_machine_lease_nonce<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.fly_machine_lease_nonce = value.try_into().map_err(|_| {
                "conversion to `String` for fly_machine_lease_nonce failed".to_string()
            });
            self
        }

        ///Sends a `DELETE` request to
        /// `/apps/{app_name}/machines/{machine_id}/lease`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                fly_machine_lease_nonce,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let fly_machine_lease_nonce = fly_machine_lease_nonce.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/lease",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append(
                "fly-machine-lease-nonce",
                HeaderValue::try_from(fly_machine_lease_nonce)?,
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).headers(header_map).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_show_metadata`]
    ///
    ///[`Client::machines_show_metadata`]: super::Client::machines_show_metadata
    #[derive(Debug, Clone)]
    pub struct MachinesShowMetadata<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesShowMetadata<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/apps/{app_name}/machines/{machine_id}/metadata`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<std::collections::HashMap<String, String>>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/metadata",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_update_metadata`]
    ///
    ///[`Client::machines_update_metadata`]: super::Client::machines_update_metadata
    #[derive(Debug, Clone)]
    pub struct MachinesUpdateMetadata<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        key: Result<String, String>,
    }

    impl<'a> MachinesUpdateMetadata<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                key: Err("key was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.key = value
                .try_into()
                .map_err(|_| "conversion to `String` for key failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/metadata/{key}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                key,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let key = key.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/metadata/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
                encode_path(&key.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_delete_metadata`]
    ///
    ///[`Client::machines_delete_metadata`]: super::Client::machines_delete_metadata
    #[derive(Debug, Clone)]
    pub struct MachinesDeleteMetadata<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        key: Result<String, String>,
    }

    impl<'a> MachinesDeleteMetadata<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                key: Err("key was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.key = value
                .try_into()
                .map_err(|_| "conversion to `String` for key failed".to_string());
            self
        }

        ///Sends a `DELETE` request to
        /// `/apps/{app_name}/machines/{machine_id}/metadata/{key}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                key,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let key = key.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/metadata/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
                encode_path(&key.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.delete(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_list_processes`]
    ///
    ///[`Client::machines_list_processes`]: super::Client::machines_list_processes
    #[derive(Debug, Clone)]
    pub struct MachinesListProcesses<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        order: Result<Option<String>, String>,
        sort_by: Result<Option<String>, String>,
    }

    impl<'a> MachinesListProcesses<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                order: Ok(None),
                sort_by: Ok(None),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn order<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.order = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for order failed".to_string());
            self
        }

        pub fn sort_by<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.sort_by = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for sort_by failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps/{app_name}/machines/{machine_id}/ps`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::ProcessStat>>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                order,
                sort_by,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let order = order.map_err(Error::InvalidRequest)?;
            let sort_by = sort_by.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/ps",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &order {
                query.push(("order", v.to_string()));
            }
            if let Some(v) = &sort_by {
                query.push(("sort_by", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_restart`]
    ///
    ///[`Client::machines_restart`]: super::Client::machines_restart
    #[derive(Debug, Clone)]
    pub struct MachinesRestart<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        signal: Result<Option<String>, String>,
        timeout: Result<Option<String>, String>,
    }

    impl<'a> MachinesRestart<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                signal: Ok(None),
                timeout: Ok(None),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn signal<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.signal = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for signal failed".to_string());
            self
        }

        pub fn timeout<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.timeout = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for timeout failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/restart`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                signal,
                timeout,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let signal = signal.map_err(Error::InvalidRequest)?;
            let timeout = timeout.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/restart",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &signal {
                query.push(("signal", v.to_string()));
            }
            if let Some(v) = &timeout {
                query.push(("timeout", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client.client.post(url).query(&query).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_signal`]
    ///
    ///[`Client::machines_signal`]: super::Client::machines_signal
    #[derive(Debug, Clone)]
    pub struct MachinesSignal<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        body: Result<types::builder::SignalRequest, String>,
    }

    impl<'a> MachinesSignal<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                body: Ok(types::builder::SignalRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SignalRequest>,
            <V as std::convert::TryInto<types::SignalRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `SignalRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::SignalRequest) -> types::builder::SignalRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/signal`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::SignalRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/signal",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_start`]
    ///
    ///[`Client::machines_start`]: super::Client::machines_start
    #[derive(Debug, Clone)]
    pub struct MachinesStart<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesStart<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/start`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/start",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_stop`]
    ///
    ///[`Client::machines_stop`]: super::Client::machines_stop
    #[derive(Debug, Clone)]
    pub struct MachinesStop<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        body: Result<types::builder::StopRequest, String>,
    }

    impl<'a> MachinesStop<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                body: Ok(types::builder::StopRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::StopRequest>,
            <V as std::convert::TryInto<types::StopRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `StopRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::StopRequest) -> types::builder::StopRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/stop`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::StopRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/stop",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_uncordon`]
    ///
    ///[`Client::machines_uncordon`]: super::Client::machines_uncordon
    #[derive(Debug, Clone)]
    pub struct MachinesUncordon<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesUncordon<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/machines/{machine_id}/uncordon`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/uncordon",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_list_versions`]
    ///
    ///[`Client::machines_list_versions`]: super::Client::machines_list_versions
    #[derive(Debug, Clone)]
    pub struct MachinesListVersions<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
    }

    impl<'a> MachinesListVersions<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/apps/{app_name}/machines/{machine_id}/versions`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::MachineVersion>>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/versions",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_wait`]
    ///
    ///[`Client::machines_wait`]: super::Client::machines_wait
    #[derive(Debug, Clone)]
    pub struct MachinesWait<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        machine_id: Result<String, String>,
        instance_id: Result<Option<String>, String>,
        state: Result<Option<types::MachinesWaitState>, String>,
        timeout: Result<Option<i64>, String>,
    }

    impl<'a> MachinesWait<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                machine_id: Err("machine_id was not initialized".to_string()),
                instance_id: Ok(None),
                state: Ok(None),
                timeout: Ok(None),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn machine_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.machine_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for machine_id failed".to_string());
            self
        }

        pub fn instance_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.instance_id = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for instance_id failed".to_string());
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::MachinesWaitState>,
        {
            self.state = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `MachinesWaitState` for state failed".to_string());
            self
        }

        pub fn timeout<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.timeout = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for timeout failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/apps/{app_name}/machines/{machine_id}/wait`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                machine_id,
                instance_id,
                state,
                timeout,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let machine_id = machine_id.map_err(Error::InvalidRequest)?;
            let instance_id = instance_id.map_err(Error::InvalidRequest)?;
            let state = state.map_err(Error::InvalidRequest)?;
            let timeout = timeout.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/machines/{}/wait",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&machine_id.to_string()),
            );
            let mut query = Vec::with_capacity(3usize);
            if let Some(v) = &instance_id {
                query.push(("instance_id", v.to_string()));
            }
            if let Some(v) = &state {
                query.push(("state", v.to_string()));
            }
            if let Some(v) = &timeout {
                query.push(("timeout", v.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client.client.get(url).query(&query).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volumes_list`]
    ///
    ///[`Client::volumes_list`]: super::Client::volumes_list
    #[derive(Debug, Clone)]
    pub struct VolumesList<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
    }

    impl<'a> VolumesList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps/{app_name}/volumes`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Volume>>, Error<()>> {
            let Self { client, app_name } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes",
                client.baseurl,
                encode_path(&app_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volumes_create`]
    ///
    ///[`Client::volumes_create`]: super::Client::volumes_create
    #[derive(Debug, Clone)]
    pub struct VolumesCreate<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        body: Result<types::builder::CreateVolumeRequest, String>,
    }

    impl<'a> VolumesCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                body: Ok(types::builder::CreateVolumeRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateVolumeRequest>,
            <V as std::convert::TryInto<types::CreateVolumeRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateVolumeRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateVolumeRequest,
            ) -> types::builder::CreateVolumeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/apps/{app_name}/volumes`
        pub async fn send(self) -> Result<ResponseValue<types::Volume>, Error<()>> {
            let Self {
                client,
                app_name,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::CreateVolumeRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes",
                client.baseurl,
                encode_path(&app_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volumes_get_by_id`]
    ///
    ///[`Client::volumes_get_by_id`]: super::Client::volumes_get_by_id
    #[derive(Debug, Clone)]
    pub struct VolumesGetById<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        volume_id: Result<String, String>,
    }

    impl<'a> VolumesGetById<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                volume_id: Err("volume_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn volume_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.volume_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for volume_id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/apps/{app_name}/volumes/{volume_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Volume>, Error<()>> {
            let Self {
                client,
                app_name,
                volume_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let volume_id = volume_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&volume_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volumes_update`]
    ///
    ///[`Client::volumes_update`]: super::Client::volumes_update
    #[derive(Debug, Clone)]
    pub struct VolumesUpdate<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        volume_id: Result<String, String>,
        body: Result<types::builder::UpdateVolumeRequest, String>,
    }

    impl<'a> VolumesUpdate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                volume_id: Err("volume_id was not initialized".to_string()),
                body: Ok(types::builder::UpdateVolumeRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn volume_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.volume_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for volume_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateVolumeRequest>,
            <V as std::convert::TryInto<types::UpdateVolumeRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `UpdateVolumeRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateVolumeRequest,
            ) -> types::builder::UpdateVolumeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/apps/{app_name}/volumes/{volume_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Volume>, Error<()>> {
            let Self {
                client,
                app_name,
                volume_id,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let volume_id = volume_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::UpdateVolumeRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&volume_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volume_delete`]
    ///
    ///[`Client::volume_delete`]: super::Client::volume_delete
    #[derive(Debug, Clone)]
    pub struct VolumeDelete<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        volume_id: Result<String, String>,
    }

    impl<'a> VolumeDelete<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                volume_id: Err("volume_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn volume_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.volume_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for volume_id failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/apps/{app_name}/volumes/{volume_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Volume>, Error<()>> {
            let Self {
                client,
                app_name,
                volume_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let volume_id = volume_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes/{}",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&volume_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volumes_extend`]
    ///
    ///[`Client::volumes_extend`]: super::Client::volumes_extend
    #[derive(Debug, Clone)]
    pub struct VolumesExtend<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        volume_id: Result<String, String>,
        body: Result<types::builder::ExtendVolumeRequest, String>,
    }

    impl<'a> VolumesExtend<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                volume_id: Err("volume_id was not initialized".to_string()),
                body: Ok(types::builder::ExtendVolumeRequest::default()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn volume_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.volume_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for volume_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ExtendVolumeRequest>,
            <V as std::convert::TryInto<types::ExtendVolumeRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ExtendVolumeRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ExtendVolumeRequest,
            ) -> types::builder::ExtendVolumeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to
        /// `/apps/{app_name}/volumes/{volume_id}/extend`
        pub async fn send(self) -> Result<ResponseValue<types::ExtendVolumeResponse>, Error<()>> {
            let Self {
                client,
                app_name,
                volume_id,
                body,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let volume_id = volume_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ExtendVolumeRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes/{}/extend",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&volume_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::volumes_list_snapshots`]
    ///
    ///[`Client::volumes_list_snapshots`]: super::Client::volumes_list_snapshots
    #[derive(Debug, Clone)]
    pub struct VolumesListSnapshots<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        volume_id: Result<String, String>,
    }

    impl<'a> VolumesListSnapshots<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                volume_id: Err("volume_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn volume_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.volume_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for volume_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/apps/{app_name}/volumes/{volume_id}/snapshots`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::VolumeSnapshot>>, Error<()>> {
            let Self {
                client,
                app_name,
                volume_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let volume_id = volume_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes/{}/snapshots",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&volume_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::create_volume_snapshot`]
    ///
    ///[`Client::create_volume_snapshot`]: super::Client::create_volume_snapshot
    #[derive(Debug, Clone)]
    pub struct CreateVolumeSnapshot<'a> {
        client: &'a super::Client,
        app_name: Result<String, String>,
        volume_id: Result<String, String>,
    }

    impl<'a> CreateVolumeSnapshot<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                app_name: Err("app_name was not initialized".to_string()),
                volume_id: Err("volume_id was not initialized".to_string()),
            }
        }

        pub fn app_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.app_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for app_name failed".to_string());
            self
        }

        pub fn volume_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.volume_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for volume_id failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/apps/{app_name}/volumes/{volume_id}/snapshots`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                app_name,
                volume_id,
            } = self;
            let app_name = app_name.map_err(Error::InvalidRequest)?;
            let volume_id = volume_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/apps/{}/volumes/{}/snapshots",
                client.baseurl,
                encode_path(&app_name.to_string()),
                encode_path(&volume_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::machines_get_oidc_token`]
    ///
    ///[`Client::machines_get_oidc_token`]: super::Client::machines_get_oidc_token
    #[derive(Debug, Clone)]
    pub struct MachinesGetOidcToken<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateOidcTokenRequest, String>,
    }

    impl<'a> MachinesGetOidcToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateOidcTokenRequest::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateOidcTokenRequest>,
            <V as std::convert::TryInto<types::CreateOidcTokenRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `CreateOidcTokenRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateOidcTokenRequest,
            ) -> types::builder::CreateOidcTokenRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/tokens/oidc`
        pub async fn send(self) -> Result<ResponseValue<String>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::CreateOidcTokenRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/v1/tokens/oidc", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
