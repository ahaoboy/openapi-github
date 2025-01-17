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

/// OrgMembership : Org Membership
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgMembership {
    #[serde(rename = "url")]
    pub url: String,
    /// The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
    #[serde(rename = "state")]
    pub state: State,
    /// The user's membership type in the organization.
    #[serde(rename = "role")]
    pub role: Role,
    #[serde(rename = "organization_url")]
    pub organization_url: String,
    #[serde(rename = "organization")]
    pub organization: Box<models::OrganizationSimple>,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::OrgMembershipPermissions>>,
}

impl OrgMembership {
    /// Org Membership
    pub fn new(url: String, state: State, role: Role, organization_url: String, organization: models::OrganizationSimple, user: Option<models::NullableSimpleUser>) -> OrgMembership {
        OrgMembership {
            url,
            state,
            role,
            organization_url,
            organization: Box::new(organization),
            user: user.map(Box::new),
            permissions: None,
        }
    }
}
/// The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
/// The user's membership type in the organization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "billing_manager")]
    BillingManager,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}

