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

/// CodeScanningAlertState : State of a code scanning alert.
/// State of a code scanning alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodeScanningAlertState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,

}

impl std::fmt::Display for CodeScanningAlertState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Dismissed => write!(f, "dismissed"),
            Self::Fixed => write!(f, "fixed"),
        }
    }
}

impl Default for CodeScanningAlertState {
    fn default() -> CodeScanningAlertState {
        Self::Open
    }
}
