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
pub struct WebhookRepositoryTransferredChangesOwnerFrom {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::Organization>>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<Box<models::User>>>,
}

impl WebhookRepositoryTransferredChangesOwnerFrom {
    pub fn new() -> WebhookRepositoryTransferredChangesOwnerFrom {
        WebhookRepositoryTransferredChangesOwnerFrom {
            organization: None,
            user: None,
        }
    }
}
