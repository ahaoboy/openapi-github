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

/// WebhooksRule : The branch protection rule. Includes a `name` and all the [branch protection settings](https://docs.github.com/github/administering-a-repository/defining-the-mergeability-of-pull-requests/about-protected-branches#about-branch-protection-settings) applied to branches that match the name. Binary settings are boolean. Multi-level configurations are one of `off`, `non_admins`, or `everyone`. Actor and build lists are arrays of strings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksRule {
    #[serde(rename = "admin_enforced")]
    pub admin_enforced: bool,
    #[serde(rename = "allow_deletions_enforcement_level")]
    pub allow_deletions_enforcement_level: AllowDeletionsEnforcementLevel,
    #[serde(rename = "allow_force_pushes_enforcement_level")]
    pub allow_force_pushes_enforcement_level: AllowForcePushesEnforcementLevel,
    #[serde(rename = "authorized_actor_names")]
    pub authorized_actor_names: Vec<String>,
    #[serde(rename = "authorized_actors_only")]
    pub authorized_actors_only: bool,
    #[serde(rename = "authorized_dismissal_actors_only")]
    pub authorized_dismissal_actors_only: bool,
    #[serde(rename = "create_protected", skip_serializing_if = "Option::is_none")]
    pub create_protected: Option<bool>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dismiss_stale_reviews_on_push")]
    pub dismiss_stale_reviews_on_push: bool,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "ignore_approvals_from_contributors")]
    pub ignore_approvals_from_contributors: bool,
    #[serde(rename = "linear_history_requirement_enforcement_level")]
    pub linear_history_requirement_enforcement_level: LinearHistoryRequirementEnforcementLevel,
    #[serde(rename = "merge_queue_enforcement_level")]
    pub merge_queue_enforcement_level: MergeQueueEnforcementLevel,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pull_request_reviews_enforcement_level")]
    pub pull_request_reviews_enforcement_level: PullRequestReviewsEnforcementLevel,
    #[serde(rename = "repository_id")]
    pub repository_id: i32,
    #[serde(rename = "require_code_owner_review")]
    pub require_code_owner_review: bool,
    /// Whether the most recent push must be approved by someone other than the person who pushed it
    #[serde(rename = "require_last_push_approval", skip_serializing_if = "Option::is_none")]
    pub require_last_push_approval: Option<bool>,
    #[serde(rename = "required_approving_review_count")]
    pub required_approving_review_count: i32,
    #[serde(rename = "required_conversation_resolution_level")]
    pub required_conversation_resolution_level: RequiredConversationResolutionLevel,
    #[serde(rename = "required_deployments_enforcement_level")]
    pub required_deployments_enforcement_level: RequiredDeploymentsEnforcementLevel,
    #[serde(rename = "required_status_checks")]
    pub required_status_checks: Vec<String>,
    #[serde(rename = "required_status_checks_enforcement_level")]
    pub required_status_checks_enforcement_level: RequiredStatusChecksEnforcementLevel,
    #[serde(rename = "signature_requirement_enforcement_level")]
    pub signature_requirement_enforcement_level: SignatureRequirementEnforcementLevel,
    #[serde(rename = "strict_required_status_checks_policy")]
    pub strict_required_status_checks_policy: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl WebhooksRule {
    /// The branch protection rule. Includes a `name` and all the [branch protection settings](https://docs.github.com/github/administering-a-repository/defining-the-mergeability-of-pull-requests/about-protected-branches#about-branch-protection-settings) applied to branches that match the name. Binary settings are boolean. Multi-level configurations are one of `off`, `non_admins`, or `everyone`. Actor and build lists are arrays of strings.
    pub fn new(admin_enforced: bool, allow_deletions_enforcement_level: AllowDeletionsEnforcementLevel, allow_force_pushes_enforcement_level: AllowForcePushesEnforcementLevel, authorized_actor_names: Vec<String>, authorized_actors_only: bool, authorized_dismissal_actors_only: bool, created_at: String, dismiss_stale_reviews_on_push: bool, id: i32, ignore_approvals_from_contributors: bool, linear_history_requirement_enforcement_level: LinearHistoryRequirementEnforcementLevel, merge_queue_enforcement_level: MergeQueueEnforcementLevel, name: String, pull_request_reviews_enforcement_level: PullRequestReviewsEnforcementLevel, repository_id: i32, require_code_owner_review: bool, required_approving_review_count: i32, required_conversation_resolution_level: RequiredConversationResolutionLevel, required_deployments_enforcement_level: RequiredDeploymentsEnforcementLevel, required_status_checks: Vec<String>, required_status_checks_enforcement_level: RequiredStatusChecksEnforcementLevel, signature_requirement_enforcement_level: SignatureRequirementEnforcementLevel, strict_required_status_checks_policy: bool, updated_at: String) -> WebhooksRule {
        WebhooksRule {
            admin_enforced,
            allow_deletions_enforcement_level,
            allow_force_pushes_enforcement_level,
            authorized_actor_names,
            authorized_actors_only,
            authorized_dismissal_actors_only,
            create_protected: None,
            created_at,
            dismiss_stale_reviews_on_push,
            id,
            ignore_approvals_from_contributors,
            linear_history_requirement_enforcement_level,
            merge_queue_enforcement_level,
            name,
            pull_request_reviews_enforcement_level,
            repository_id,
            require_code_owner_review,
            require_last_push_approval: None,
            required_approving_review_count,
            required_conversation_resolution_level,
            required_deployments_enforcement_level,
            required_status_checks,
            required_status_checks_enforcement_level,
            signature_requirement_enforcement_level,
            strict_required_status_checks_policy,
            updated_at,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowDeletionsEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for AllowDeletionsEnforcementLevel {
    fn default() -> AllowDeletionsEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowForcePushesEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for AllowForcePushesEnforcementLevel {
    fn default() -> AllowForcePushesEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinearHistoryRequirementEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for LinearHistoryRequirementEnforcementLevel {
    fn default() -> LinearHistoryRequirementEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeQueueEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for MergeQueueEnforcementLevel {
    fn default() -> MergeQueueEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PullRequestReviewsEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for PullRequestReviewsEnforcementLevel {
    fn default() -> PullRequestReviewsEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiredConversationResolutionLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for RequiredConversationResolutionLevel {
    fn default() -> RequiredConversationResolutionLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiredDeploymentsEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for RequiredDeploymentsEnforcementLevel {
    fn default() -> RequiredDeploymentsEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiredStatusChecksEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for RequiredStatusChecksEnforcementLevel {
    fn default() -> RequiredStatusChecksEnforcementLevel {
        Self::Off
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignatureRequirementEnforcementLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "everyone")]
    Everyone,
}

impl Default for SignatureRequirementEnforcementLevel {
    fn default() -> SignatureRequirementEnforcementLevel {
        Self::Off
    }
}

