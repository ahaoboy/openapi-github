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
pub struct MigrationsSetLfsPreferenceRequest {
    /// Whether to store large files during the import. `opt_in` means large files will be stored using Git LFS. `opt_out` means large files will be removed during the import.
    #[serde(rename = "use_lfs")]
    pub use_lfs: UseLfs,
}

impl MigrationsSetLfsPreferenceRequest {
    pub fn new(use_lfs: UseLfs) -> MigrationsSetLfsPreferenceRequest {
        MigrationsSetLfsPreferenceRequest {
            use_lfs,
        }
    }
}
/// Whether to store large files during the import. `opt_in` means large files will be stored using Git LFS. `opt_out` means large files will be removed during the import.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UseLfs {
    #[serde(rename = "opt_in")]
    In,
    #[serde(rename = "opt_out")]
    Out,
}

impl Default for UseLfs {
    fn default() -> UseLfs {
        Self::In
    }
}

