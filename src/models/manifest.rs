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
pub struct Manifest {
    /// The name of the manifest.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<models::ManifestFile>>,
    /// User-defined metadata to store domain-specific information limited to 8 keys with scalar values.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, models::MetadataValue>>,
    /// A collection of resolved package dependencies.
    #[serde(rename = "resolved", skip_serializing_if = "Option::is_none")]
    pub resolved: Option<std::collections::HashMap<String, models::Dependency>>,
}

impl Manifest {
    pub fn new(name: String) -> Manifest {
        Manifest {
            name,
            file: None,
            metadata: None,
            resolved: None,
        }
    }
}
