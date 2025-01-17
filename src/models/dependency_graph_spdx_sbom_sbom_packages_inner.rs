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
pub struct DependencyGraphSpdxSbomSbomPackagesInner {
    /// A unique SPDX identifier for the package.
    #[serde(rename = "SPDXID", skip_serializing_if = "Option::is_none")]
    pub spdxid: Option<String>,
    /// The name of the package.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the package. If the package does not have an exact version specified, a version range is given.
    #[serde(rename = "versionInfo", skip_serializing_if = "Option::is_none")]
    pub version_info: Option<String>,
    /// The location where the package can be downloaded, or NOASSERTION if this has not been determined.
    #[serde(rename = "downloadLocation", skip_serializing_if = "Option::is_none")]
    pub download_location: Option<String>,
    /// Whether the package's file content has been subjected to analysis during the creation of the SPDX document.
    #[serde(rename = "filesAnalyzed", skip_serializing_if = "Option::is_none")]
    pub files_analyzed: Option<bool>,
    /// The license of the package as determined while creating the SPDX document.
    #[serde(rename = "licenseConcluded", skip_serializing_if = "Option::is_none")]
    pub license_concluded: Option<String>,
    /// The license of the package as declared by its author, or NOASSERTION if this information was not available when the SPDX document was created.
    #[serde(rename = "licenseDeclared", skip_serializing_if = "Option::is_none")]
    pub license_declared: Option<String>,
    /// The distribution source of this package, or NOASSERTION if this was not determined.
    #[serde(rename = "supplier", skip_serializing_if = "Option::is_none")]
    pub supplier: Option<String>,
    #[serde(rename = "externalRefs", skip_serializing_if = "Option::is_none")]
    pub external_refs: Option<Vec<models::DependencyGraphSpdxSbomSbomPackagesInnerExternalRefsInner>>,
}

impl DependencyGraphSpdxSbomSbomPackagesInner {
    pub fn new() -> DependencyGraphSpdxSbomSbomPackagesInner {
        DependencyGraphSpdxSbomSbomPackagesInner {
            spdxid: None,
            name: None,
            version_info: None,
            download_location: None,
            files_analyzed: None,
            license_concluded: None,
            license_declared: None,
            supplier: None,
            external_refs: None,
        }
    }
}

