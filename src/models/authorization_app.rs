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
pub struct AuthorizationApp {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl AuthorizationApp {
    pub fn new(client_id: String, name: String, url: String) -> AuthorizationApp {
        AuthorizationApp {
            client_id,
            name,
            url,
        }
    }
}
