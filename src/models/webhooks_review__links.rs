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
pub struct WebhooksReviewLinks {
    #[serde(rename = "html")]
    pub html: Box<models::Link>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::Link>,
}

impl WebhooksReviewLinks {
    pub fn new(html: models::Link, pull_request: models::Link) -> WebhooksReviewLinks {
        WebhooksReviewLinks {
            html: Box::new(html),
            pull_request: Box::new(pull_request),
        }
    }
}
