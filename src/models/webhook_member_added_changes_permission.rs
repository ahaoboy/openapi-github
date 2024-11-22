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

/// WebhookMemberAddedChangesPermission : This field is included for legacy purposes; use the `role_name` field instead. The `maintain` role is mapped to `write` and the `triage` role is mapped to `read`. To determine the role assigned to the collaborator, use the `role_name` field instead, which will provide the full role name, including custom roles.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookMemberAddedChangesPermission {
    #[serde(rename = "to")]
    pub to: To,
}

impl WebhookMemberAddedChangesPermission {
    /// This field is included for legacy purposes; use the `role_name` field instead. The `maintain` role is mapped to `write` and the `triage` role is mapped to `read`. To determine the role assigned to the collaborator, use the `role_name` field instead, which will provide the full role name, including custom roles.
    pub fn new(to: To) -> WebhookMemberAddedChangesPermission {
        WebhookMemberAddedChangesPermission {
            to,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum To {
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
}

impl Default for To {
    fn default() -> To {
        Self::Write
    }
}
