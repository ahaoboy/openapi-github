/*
 * GitHub's official OpenAPI spec + Octokit extension
 *
 * OpenAPI specs from https://github.com/github/rest-api-description with the 'x-octokit' extension required by the Octokit SDKs
 *
 * The version of the OpenAPI document: 16.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookStatusCommitCommitVerification {
    #[serde(rename = "payload", deserialize_with = "Option::deserialize")]
    pub payload: Option<String>,
    #[serde(rename = "reason")]
    pub reason: Reason,
    #[serde(rename = "signature", deserialize_with = "Option::deserialize")]
    pub signature: Option<String>,
    #[serde(rename = "verified")]
    pub verified: bool,
}

impl WebhookStatusCommitCommitVerification {
    pub fn new(payload: Option<String>, reason: Reason, signature: Option<String>, verified: bool) -> WebhookStatusCommitCommitVerification {
        WebhookStatusCommitCommitVerification {
            payload,
            reason,
            signature,
            verified,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "expired_key")]
    ExpiredKey,
    #[serde(rename = "not_signing_key")]
    NotSigningKey,
    #[serde(rename = "gpgverify_error")]
    GpgverifyError,
    #[serde(rename = "gpgverify_unavailable")]
    GpgverifyUnavailable,
    #[serde(rename = "unsigned")]
    Unsigned,
    #[serde(rename = "unknown_signature_type")]
    UnknownSignatureType,
    #[serde(rename = "no_user")]
    NoUser,
    #[serde(rename = "unverified_email")]
    UnverifiedEmail,
    #[serde(rename = "bad_email")]
    BadEmail,
    #[serde(rename = "unknown_key")]
    UnknownKey,
    #[serde(rename = "malformed_signature")]
    MalformedSignature,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "bad_cert")]
    BadCert,
    #[serde(rename = "ocsp_pending")]
    OcspPending,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::ExpiredKey
    }
}
