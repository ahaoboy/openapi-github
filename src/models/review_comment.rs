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

/// ReviewComment : Legacy Review Comment
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewComment {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "pull_request_review_id", deserialize_with = "Option::deserialize")]
    pub pull_request_review_id: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "diff_hunk")]
    pub diff_hunk: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "position", deserialize_with = "Option::deserialize")]
    pub position: Option<i32>,
    #[serde(rename = "original_position")]
    pub original_position: i32,
    #[serde(rename = "commit_id")]
    pub commit_id: String,
    #[serde(rename = "original_commit_id")]
    pub original_commit_id: String,
    #[serde(rename = "in_reply_to_id", skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i32>,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub pull_request_url: String,
    #[serde(rename = "author_association")]
    pub author_association: models::AuthorAssociation,
    #[serde(rename = "_links")]
    pub _links: Box<models::ReviewCommentLinks>,
    #[serde(rename = "body_text", skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(rename = "body_html", skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<models::ReactionRollup>>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "start_side", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_side: Option<Option<StartSide>>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    /// The original line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "original_line", skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i32>,
    /// The first line of the range for a multi-line comment.
    #[serde(rename = "start_line", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_line: Option<Option<i32>>,
    /// The original first line of the range for a multi-line comment.
    #[serde(rename = "original_start_line", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<Option<i32>>,
}

impl ReviewComment {
    /// Legacy Review Comment
    pub fn new(url: String, pull_request_review_id: Option<i32>, id: i32, node_id: String, diff_hunk: String, path: String, position: Option<i32>, original_position: i32, commit_id: String, original_commit_id: String, user: Option<models::NullableSimpleUser>, body: String, created_at: String, updated_at: String, html_url: String, pull_request_url: String, author_association: models::AuthorAssociation, _links: models::ReviewCommentLinks) -> ReviewComment {
        ReviewComment {
            url,
            pull_request_review_id,
            id,
            node_id,
            diff_hunk,
            path,
            position,
            original_position,
            commit_id,
            original_commit_id,
            in_reply_to_id: None,
            user: if let Some(x) = user {Some(Box::new(x))} else {None},
            body,
            created_at,
            updated_at,
            html_url,
            pull_request_url,
            author_association,
            _links: Box::new(_links),
            body_text: None,
            body_html: None,
            reactions: None,
            side: None,
            start_side: None,
            line: None,
            original_line: None,
            start_line: None,
            original_start_line: None,
        }
    }
}
/// The side of the first line of the range for a multi-line comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}

impl Default for Side {
    fn default() -> Side {
        Self::Left
    }
}
/// The side of the first line of the range for a multi-line comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}

impl Default for StartSide {
    fn default() -> StartSide {
        Self::Left
    }
}
