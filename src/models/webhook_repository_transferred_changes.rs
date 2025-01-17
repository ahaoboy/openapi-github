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
pub struct WebhookRepositoryTransferredChanges {
    #[serde(rename = "owner")]
    pub owner: Box<models::WebhookRepositoryTransferredChangesOwner>,
}

impl WebhookRepositoryTransferredChanges {
    pub fn new(owner: models::WebhookRepositoryTransferredChangesOwner) -> WebhookRepositoryTransferredChanges {
        WebhookRepositoryTransferredChanges {
            owner: Box::new(owner),
        }
    }
}

