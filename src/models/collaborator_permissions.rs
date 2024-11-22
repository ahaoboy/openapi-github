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
pub struct CollaboratorPermissions {
    #[serde(rename = "pull")]
    pub pull: bool,
    #[serde(rename = "triage", skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
    #[serde(rename = "push")]
    pub push: bool,
    #[serde(rename = "maintain", skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    #[serde(rename = "admin")]
    pub admin: bool,
}

impl CollaboratorPermissions {
    pub fn new(pull: bool, push: bool, admin: bool) -> CollaboratorPermissions {
        CollaboratorPermissions {
            pull,
            triage: None,
            push,
            maintain: None,
            admin,
        }
    }
}
