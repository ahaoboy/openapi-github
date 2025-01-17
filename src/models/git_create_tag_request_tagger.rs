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

/// GitCreateTagRequestTagger : An object with information about the individual creating the tag.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitCreateTagRequestTagger {
    /// The name of the author of the tag
    #[serde(rename = "name")]
    pub name: String,
    /// The email of the author of the tag
    #[serde(rename = "email")]
    pub email: String,
    /// When this object was tagged. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl GitCreateTagRequestTagger {
    /// An object with information about the individual creating the tag.
    pub fn new(name: String, email: String) -> GitCreateTagRequestTagger {
        GitCreateTagRequestTagger {
            name,
            email,
            date: None,
        }
    }
}

