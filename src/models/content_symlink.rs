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

/// ContentSymlink : An object describing a symlink
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentSymlink {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "git_url", deserialize_with = "Option::deserialize")]
    pub git_url: Option<String>,
    #[serde(rename = "html_url", deserialize_with = "Option::deserialize")]
    pub html_url: Option<String>,
    #[serde(rename = "download_url", deserialize_with = "Option::deserialize")]
    pub download_url: Option<String>,
    #[serde(rename = "_links")]
    pub _links: Box<models::ContentTreeEntriesInnerLinks>,
}

impl ContentSymlink {
    /// An object describing a symlink
    pub fn new(r#type: Type, target: String, size: i32, name: String, path: String, sha: String, url: String, git_url: Option<String>, html_url: Option<String>, download_url: Option<String>, _links: models::ContentTreeEntriesInnerLinks) -> ContentSymlink {
        ContentSymlink {
            r#type,
            target,
            size,
            name,
            path,
            sha,
            url,
            git_url,
            html_url,
            download_url,
            _links: Box::new(_links),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "symlink")]
    Symlink,
}

impl Default for Type {
    fn default() -> Type {
        Self::Symlink
    }
}
