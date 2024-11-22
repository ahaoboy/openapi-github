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

/// ChecksCreateRequestOutput : Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChecksCreateRequestOutput {
    /// The title of the check run.
    #[serde(rename = "title")]
    pub title: String,
    /// The summary of the check run. This parameter supports Markdown. **Maximum length**: 65535 characters.
    #[serde(rename = "summary")]
    pub summary: String,
    /// The details of the check run. This parameter supports Markdown. **Maximum length**: 65535 characters.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Adds information from your analysis to specific lines of code. Annotations are visible on GitHub in the **Checks** and **Files changed** tab of the pull request. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. GitHub Actions are limited to 10 warning annotations and 10 error annotations per step. For details about how you can view annotations on GitHub, see \"[About status checks](https://docs.github.com/articles/about-status-checks#checks)\".
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<models::ChecksCreateRequestOutputAnnotationsInner>>,
    /// Adds images to the output displayed in the GitHub pull request UI.
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<models::ChecksCreateRequestOutputImagesInner>>,
}

impl ChecksCreateRequestOutput {
    /// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run.
    pub fn new(title: String, summary: String) -> ChecksCreateRequestOutput {
        ChecksCreateRequestOutput {
            title,
            summary,
            text: None,
            annotations: None,
            images: None,
        }
    }
}
