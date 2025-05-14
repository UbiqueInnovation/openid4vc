use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Token Response as described here: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-13.html#name-successful-token-response
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<StringOrInt>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub c_nonce: Option<String>,
    pub c_nonce_expires_in: Option<StringOrInt>,
    // TODO: add `authorization_details` field when support for Authorization Code Flow is added.
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrInt {
    String(String),
    Int(u64),
}

impl From<StringOrInt> for u64 {
    fn from(value: StringOrInt) -> Self {
        match value {
            StringOrInt::String(s) => u64::from_str_radix(s.as_str(), 10).unwrap_or(0),
            StringOrInt::Int(i) => i,
        }
    }
}
