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
pub struct WebhookTeamEditedChangesRepository {
    #[serde(rename = "permissions")]
    pub permissions: Box<models::WebhookTeamEditedChangesRepositoryPermissions>,
}

impl WebhookTeamEditedChangesRepository {
    pub fn new(permissions: models::WebhookTeamEditedChangesRepositoryPermissions) -> WebhookTeamEditedChangesRepository {
        WebhookTeamEditedChangesRepository {
            permissions: Box::new(permissions),
        }
    }
}

