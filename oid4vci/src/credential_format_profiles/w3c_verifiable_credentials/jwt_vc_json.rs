use crate::credential_format;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::CredentialSubject;

credential_format!("jwt_vc_json", JwtVcJson, {
    credential_definition: CredentialDefinition,
    order: Option<StringOrVec>
});
credential_format!("vc+sd-jwt", JwtVcSdJwt, {
    credential_definition: Option<CredentialDefinition>,
    vct: String,
    order: Option<StringOrVec>
});

credential_format!("vc+sd-jwt", SdJwtW3CVcdm, {
    credential_definition: Option<CredentialDefinition>,
    order: Option<StringOrVec>
});

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SdJwtVc;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum SdJwtParameters {
    SdJwt(<JwtVcSdJwt as crate::credential_format_profiles::Format>::Parameters),
    W3C(<SdJwtW3CVcdm as crate::credential_format_profiles::Format>::Parameters),
}

impl crate::credential_format_profiles::Format for SdJwtVc {
    type Parameters = SdJwtParameters;

    type Credential = serde_json::Value;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum StringOrVec {
    One(String),
    Many(Vec<String>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CredentialDefinition {
    #[serde(rename = "type")]
    pub type_: StringOrVec,
    #[serde(flatten)]
    pub credential_subject: CredentialSubject,
}
