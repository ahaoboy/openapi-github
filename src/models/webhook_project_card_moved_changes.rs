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
pub struct WebhookProjectCardMovedChanges {
    #[serde(rename = "column_id")]
    pub column_id: Box<models::WebhookProjectCardMovedChangesColumnId>,
}

impl WebhookProjectCardMovedChanges {
    pub fn new(column_id: models::WebhookProjectCardMovedChangesColumnId) -> WebhookProjectCardMovedChanges {
        WebhookProjectCardMovedChanges {
            column_id: Box::new(column_id),
        }
    }
}

