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
pub struct WebhookPackagePublishedPackageRegistry {
    #[serde(rename = "about_url")]
    pub about_url: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "vendor")]
    pub vendor: String,
}

impl WebhookPackagePublishedPackageRegistry {
    pub fn new(about_url: String, name: String, r#type: String, url: String, vendor: String) -> WebhookPackagePublishedPackageRegistry {
        WebhookPackagePublishedPackageRegistry {
            about_url,
            name,
            r#type,
            url,
            vendor,
        }
    }
}

