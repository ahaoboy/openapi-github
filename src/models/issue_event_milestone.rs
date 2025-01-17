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

/// IssueEventMilestone : Issue Event Milestone
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueEventMilestone {
    #[serde(rename = "title")]
    pub title: String,
}

impl IssueEventMilestone {
    /// Issue Event Milestone
    pub fn new(title: String) -> IssueEventMilestone {
        IssueEventMilestone {
            title,
        }
    }
}

