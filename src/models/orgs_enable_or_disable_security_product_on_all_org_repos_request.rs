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
pub struct OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest {
    /// CodeQL query suite to be used. If you specify the `query_suite` parameter, the default setup will be configured with this query suite only on all repositories that didn't have default setup already configured. It will not change the query suite on repositories that already have default setup configured. If you don't specify any `query_suite` in your request, the preferred query suite of the organization will be applied.
    #[serde(rename = "query_suite", skip_serializing_if = "Option::is_none")]
    pub query_suite: Option<QuerySuite>,
}

impl OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest {
    pub fn new() -> OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest {
        OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest {
            query_suite: None,
        }
    }
}
/// CodeQL query suite to be used. If you specify the `query_suite` parameter, the default setup will be configured with this query suite only on all repositories that didn't have default setup already configured. It will not change the query suite on repositories that already have default setup configured. If you don't specify any `query_suite` in your request, the preferred query suite of the organization will be applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuerySuite {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "extended")]
    Extended,
}

impl Default for QuerySuite {
    fn default() -> QuerySuite {
        Self::Default
    }
}

