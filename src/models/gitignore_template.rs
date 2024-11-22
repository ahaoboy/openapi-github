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

/// GitignoreTemplate : Gitignore Template
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitignoreTemplate {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "source")]
    pub source: String,
}

impl GitignoreTemplate {
    /// Gitignore Template
    pub fn new(name: String, source: String) -> GitignoreTemplate {
        GitignoreTemplate {
            name,
            source,
        }
    }
}
