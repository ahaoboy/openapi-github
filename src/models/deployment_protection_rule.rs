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

/// DeploymentProtectionRule : Deployment protection rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProtectionRule {
    /// The unique identifier for the deployment protection rule.
    #[serde(rename = "id")]
    pub id: i32,
    /// The node ID for the deployment protection rule.
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Whether the deployment protection rule is enabled for the environment.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "app")]
    pub app: Box<models::CustomDeploymentRuleApp>,
}

impl DeploymentProtectionRule {
    /// Deployment protection rule
    pub fn new(id: i32, node_id: String, enabled: bool, app: models::CustomDeploymentRuleApp) -> DeploymentProtectionRule {
        DeploymentProtectionRule {
            id,
            node_id,
            enabled,
            app: Box::new(app),
        }
    }
}
