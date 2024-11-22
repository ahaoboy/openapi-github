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

/// WebhookCheckRunCompletedFormEncoded : The check_run.completed webhook encoded with URL encoding
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookCheckRunCompletedFormEncoded {
    /// A URL-encoded string of the check_run.completed JSON payload. The decoded payload is a JSON object.
    #[serde(rename = "payload")]
    pub payload: String,
}

impl WebhookCheckRunCompletedFormEncoded {
    /// The check_run.completed webhook encoded with URL encoding
    pub fn new(payload: String) -> WebhookCheckRunCompletedFormEncoded {
        WebhookCheckRunCompletedFormEncoded {
            payload,
        }
    }
}
