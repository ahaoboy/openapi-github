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

/// Codespace : A codespace.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Codespace {
    #[serde(rename = "id")]
    pub id: i32,
    /// Automatically generated name of this codespace.
    #[serde(rename = "name")]
    pub name: String,
    /// Display name for this codespace.
    #[serde(rename = "display_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Option<String>>,
    /// UUID identifying this codespace's environment.
    #[serde(rename = "environment_id", deserialize_with = "Option::deserialize")]
    pub environment_id: Option<String>,
    #[serde(rename = "owner")]
    pub owner: Box<models::SimpleUser>,
    #[serde(rename = "billable_owner")]
    pub billable_owner: Box<models::SimpleUser>,
    #[serde(rename = "repository")]
    pub repository: Box<models::MinimalRepository>,
    #[serde(rename = "machine", deserialize_with = "Option::deserialize")]
    pub machine: Option<Box<models::NullableCodespaceMachine>>,
    /// Path to devcontainer.json from repo root used to create Codespace.
    #[serde(rename = "devcontainer_path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub devcontainer_path: Option<Option<String>>,
    /// Whether the codespace was created from a prebuild.
    #[serde(rename = "prebuild", deserialize_with = "Option::deserialize")]
    pub prebuild: Option<bool>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Last known time this codespace was started.
    #[serde(rename = "last_used_at")]
    pub last_used_at: String,
    /// State of this codespace.
    #[serde(rename = "state")]
    pub state: State,
    /// API URL for this codespace.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "git_status")]
    pub git_status: Box<models::CodespaceGitStatus>,
    /// The initally assigned location of a new codespace.
    #[serde(rename = "location")]
    pub location: Location,
    /// The number of minutes of inactivity after which this codespace will be automatically stopped.
    #[serde(rename = "idle_timeout_minutes", deserialize_with = "Option::deserialize")]
    pub idle_timeout_minutes: Option<i32>,
    /// URL to access this codespace on the web.
    #[serde(rename = "web_url")]
    pub web_url: String,
    /// API URL to access available alternate machine types for this codespace.
    #[serde(rename = "machines_url")]
    pub machines_url: String,
    /// API URL to start this codespace.
    #[serde(rename = "start_url")]
    pub start_url: String,
    /// API URL to stop this codespace.
    #[serde(rename = "stop_url")]
    pub stop_url: String,
    /// API URL to publish this codespace to a new repository.
    #[serde(rename = "publish_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub publish_url: Option<Option<String>>,
    /// API URL for the Pull Request associated with this codespace, if any.
    #[serde(rename = "pulls_url", deserialize_with = "Option::deserialize")]
    pub pulls_url: Option<String>,
    #[serde(rename = "recent_folders")]
    pub recent_folders: Vec<String>,
    #[serde(rename = "runtime_constraints", skip_serializing_if = "Option::is_none")]
    pub runtime_constraints: Option<Box<models::CodespaceRuntimeConstraints>>,
    /// Whether or not a codespace has a pending async operation. This would mean that the codespace is temporarily unavailable. The only thing that you can do with a codespace in this state is delete it.
    #[serde(rename = "pending_operation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pending_operation: Option<Option<bool>>,
    /// Text to show user when codespace is disabled by a pending operation
    #[serde(rename = "pending_operation_disabled_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pending_operation_disabled_reason: Option<Option<String>>,
    /// Text to show user when codespace idle timeout minutes has been overriden by an organization policy
    #[serde(rename = "idle_timeout_notice", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub idle_timeout_notice: Option<Option<String>>,
    /// Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days).
    #[serde(rename = "retention_period_minutes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub retention_period_minutes: Option<Option<i32>>,
    /// When a codespace will be auto-deleted based on the \"retention_period_minutes\" and \"last_used_at\"
    #[serde(rename = "retention_expires_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub retention_expires_at: Option<Option<String>>,
    /// The text to display to a user when a codespace has been stopped for a potentially actionable reason.
    #[serde(rename = "last_known_stop_notice", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_known_stop_notice: Option<Option<String>>,
}

impl Codespace {
    /// A codespace.
    pub fn new(id: i32, name: String, environment_id: Option<String>, owner: models::SimpleUser, billable_owner: models::SimpleUser, repository: models::MinimalRepository, machine: Option<models::NullableCodespaceMachine>, prebuild: Option<bool>, created_at: String, updated_at: String, last_used_at: String, state: State, url: String, git_status: models::CodespaceGitStatus, location: Location, idle_timeout_minutes: Option<i32>, web_url: String, machines_url: String, start_url: String, stop_url: String, pulls_url: Option<String>, recent_folders: Vec<String>) -> Codespace {
        Codespace {
            id,
            name,
            display_name: None,
            environment_id,
            owner: Box::new(owner),
            billable_owner: Box::new(billable_owner),
            repository: Box::new(repository),
            machine: machine.map(Box::new),
            devcontainer_path: None,
            prebuild,
            created_at,
            updated_at,
            last_used_at,
            state,
            url,
            git_status: Box::new(git_status),
            location,
            idle_timeout_minutes,
            web_url,
            machines_url,
            start_url,
            stop_url,
            publish_url: None,
            pulls_url,
            recent_folders,
            runtime_constraints: None,
            pending_operation: None,
            pending_operation_disabled_reason: None,
            idle_timeout_notice: None,
            retention_period_minutes: None,
            retention_expires_at: None,
            last_known_stop_notice: None,
        }
    }
}
/// State of this codespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Provisioning")]
    Provisioning,
    #[serde(rename = "Available")]
    Available,
    #[serde(rename = "Awaiting")]
    Awaiting,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "Moved")]
    Moved,
    #[serde(rename = "Shutdown")]
    Shutdown,
    #[serde(rename = "Archived")]
    Archived,
    #[serde(rename = "Starting")]
    Starting,
    #[serde(rename = "ShuttingDown")]
    ShuttingDown,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Exporting")]
    Exporting,
    #[serde(rename = "Updating")]
    Updating,
    #[serde(rename = "Rebuilding")]
    Rebuilding,
}

impl Default for State {
    fn default() -> State {
        Self::Unknown
    }
}
/// The initally assigned location of a new codespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Location {
    #[serde(rename = "EastUs")]
    EastUs,
    #[serde(rename = "SouthEastAsia")]
    SouthEastAsia,
    #[serde(rename = "WestEurope")]
    WestEurope,
    #[serde(rename = "WestUs2")]
    WestUs2,
}

impl Default for Location {
    fn default() -> Location {
        Self::EastUs
    }
}

