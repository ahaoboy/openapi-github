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

/// CodeScanningAlertDismissedReason : **Required when the state is dismissed.** The reason for dismissing or closing the alert.
/// **Required when the state is dismissed.** The reason for dismissing or closing the alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodeScanningAlertDismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "won't fix")]
    WonQuoteTFix,
    #[serde(rename = "used in tests")]
    UsedInTests,

}

impl std::fmt::Display for CodeScanningAlertDismissedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FalsePositive => write!(f, "false positive"),
            Self::WonQuoteTFix => write!(f, "won't fix"),
            Self::UsedInTests => write!(f, "used in tests"),
        }
    }
}

impl Default for CodeScanningAlertDismissedReason {
    fn default() -> CodeScanningAlertDismissedReason {
        Self::FalsePositive
    }
}

