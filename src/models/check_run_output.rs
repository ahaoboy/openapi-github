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
pub struct CheckRunOutput {
    #[serde(rename = "title", deserialize_with = "Option::deserialize")]
    pub title: Option<String>,
    #[serde(rename = "summary", deserialize_with = "Option::deserialize")]
    pub summary: Option<String>,
    #[serde(rename = "text", deserialize_with = "Option::deserialize")]
    pub text: Option<String>,
    #[serde(rename = "annotations_count")]
    pub annotations_count: i32,
    #[serde(rename = "annotations_url")]
    pub annotations_url: String,
}

impl CheckRunOutput {
    pub fn new(title: Option<String>, summary: Option<String>, text: Option<String>, annotations_count: i32, annotations_url: String) -> CheckRunOutput {
        CheckRunOutput {
            title,
            summary,
            text,
            annotations_count,
            annotations_url,
        }
    }
}
