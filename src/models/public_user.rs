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

/// PublicUser : Public User
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id", deserialize_with = "Option::deserialize")]
    pub gravatar_id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    #[serde(rename = "company", deserialize_with = "Option::deserialize")]
    pub company: Option<String>,
    #[serde(rename = "blog", deserialize_with = "Option::deserialize")]
    pub blog: Option<String>,
    #[serde(rename = "location", deserialize_with = "Option::deserialize")]
    pub location: Option<String>,
    #[serde(rename = "email", deserialize_with = "Option::deserialize")]
    pub email: Option<String>,
    #[serde(rename = "hireable", deserialize_with = "Option::deserialize")]
    pub hireable: Option<bool>,
    #[serde(rename = "bio", deserialize_with = "Option::deserialize")]
    pub bio: Option<String>,
    #[serde(rename = "twitter_username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<Option<String>>,
    #[serde(rename = "public_repos")]
    pub public_repos: i32,
    #[serde(rename = "public_gists")]
    pub public_gists: i32,
    #[serde(rename = "followers")]
    pub followers: i32,
    #[serde(rename = "following")]
    pub following: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<models::PublicUserPlan>>,
    #[serde(rename = "suspended_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<Option<String>>,
    #[serde(rename = "private_gists", skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<i32>,
    #[serde(rename = "total_private_repos", skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i32>,
    #[serde(rename = "owned_private_repos", skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i32>,
    #[serde(rename = "disk_usage", skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<i32>,
    #[serde(rename = "collaborators", skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<i32>,
}

impl PublicUser {
    /// Public User
    pub fn new(login: String, id: i32, node_id: String, avatar_url: String, gravatar_id: Option<String>, url: String, html_url: String, followers_url: String, following_url: String, gists_url: String, starred_url: String, subscriptions_url: String, organizations_url: String, repos_url: String, events_url: String, received_events_url: String, r#type: String, site_admin: bool, name: Option<String>, company: Option<String>, blog: Option<String>, location: Option<String>, email: Option<String>, hireable: Option<bool>, bio: Option<String>, public_repos: i32, public_gists: i32, followers: i32, following: i32, created_at: String, updated_at: String) -> PublicUser {
        PublicUser {
            login,
            id,
            node_id,
            avatar_url,
            gravatar_id,
            url,
            html_url,
            followers_url,
            following_url,
            gists_url,
            starred_url,
            subscriptions_url,
            organizations_url,
            repos_url,
            events_url,
            received_events_url,
            r#type,
            site_admin,
            name,
            company,
            blog,
            location,
            email,
            hireable,
            bio,
            twitter_username: None,
            public_repos,
            public_gists,
            followers,
            following,
            created_at,
            updated_at,
            plan: None,
            suspended_at: None,
            private_gists: None,
            total_private_repos: None,
            owned_private_repos: None,
            disk_usage: None,
            collaborators: None,
        }
    }
}

