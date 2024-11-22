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
pub struct ActionsListSelfHostedRunnersForOrg200Response {
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "runners")]
    pub runners: Vec<models::Runner>,
}

impl ActionsListSelfHostedRunnersForOrg200Response {
    pub fn new(total_count: i32, runners: Vec<models::Runner>) -> ActionsListSelfHostedRunnersForOrg200Response {
        ActionsListSelfHostedRunnersForOrg200Response {
            total_count,
            runners,
        }
    }
}
