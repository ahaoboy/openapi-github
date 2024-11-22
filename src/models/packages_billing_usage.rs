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
pub struct PackagesBillingUsage {
    /// Sum of the free and paid storage space (GB) for GitHuub Packages.
    #[serde(rename = "total_gigabytes_bandwidth_used")]
    pub total_gigabytes_bandwidth_used: i32,
    /// Total paid storage space (GB) for GitHuub Packages.
    #[serde(rename = "total_paid_gigabytes_bandwidth_used")]
    pub total_paid_gigabytes_bandwidth_used: i32,
    /// Free storage space (GB) for GitHub Packages.
    #[serde(rename = "included_gigabytes_bandwidth")]
    pub included_gigabytes_bandwidth: i32,
}

impl PackagesBillingUsage {
    pub fn new(total_gigabytes_bandwidth_used: i32, total_paid_gigabytes_bandwidth_used: i32, included_gigabytes_bandwidth: i32) -> PackagesBillingUsage {
        PackagesBillingUsage {
            total_gigabytes_bandwidth_used,
            total_paid_gigabytes_bandwidth_used,
            included_gigabytes_bandwidth,
        }
    }
}
