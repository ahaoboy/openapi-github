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
pub struct WebhookRegistryPackagePublishedRegistryPackageRegistry {
    #[serde(rename = "about_url", skip_serializing_if = "Option::is_none")]
    pub about_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

impl WebhookRegistryPackagePublishedRegistryPackageRegistry {
    pub fn new() -> WebhookRegistryPackagePublishedRegistryPackageRegistry {
        WebhookRegistryPackagePublishedRegistryPackageRegistry {
            about_url: None,
            name: None,
            r#type: None,
            url: None,
            vendor: None,
        }
    }
}

