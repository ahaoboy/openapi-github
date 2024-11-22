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

/// SimpleRepository : A GitHub repository.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleRepository {
    /// A unique identifier of the repository.
    #[serde(rename = "id")]
    pub id: i32,
    /// The GraphQL identifier of the repository.
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The name of the repository.
    #[serde(rename = "name")]
    pub name: String,
    /// The full, globally unique, name of the repository.
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "owner")]
    pub owner: Box<models::SimpleUser>,
    /// Whether the repository is private.
    #[serde(rename = "private")]
    pub private: bool,
    /// The URL to view the repository on GitHub.com.
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// The repository description.
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Whether the repository is a fork.
    #[serde(rename = "fork")]
    pub fork: bool,
    /// The URL to get more information about the repository from the GitHub API.
    #[serde(rename = "url")]
    pub url: String,
    /// A template for the API URL to download the repository as an archive.
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    /// A template for the API URL to list the available assignees for issues in the repository.
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    /// A template for the API URL to create or retrieve a raw Git blob in the repository.
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    /// A template for the API URL to get information about branches in the repository.
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    /// A template for the API URL to get information about collaborators of the repository.
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    /// A template for the API URL to get information about comments on the repository.
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    /// A template for the API URL to get information about commits on the repository.
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    /// A template for the API URL to compare two commits or refs.
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    /// A template for the API URL to get the contents of the repository.
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    /// A template for the API URL to list the contributors to the repository.
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    /// The API URL to list the deployments of the repository.
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    /// The API URL to list the downloads on the repository.
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    /// The API URL to list the events of the repository.
    #[serde(rename = "events_url")]
    pub events_url: String,
    /// The API URL to list the forks of the repository.
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    /// A template for the API URL to get information about Git commits of the repository.
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    /// A template for the API URL to get information about Git refs of the repository.
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    /// A template for the API URL to get information about Git tags of the repository.
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    /// A template for the API URL to get information about issue comments on the repository.
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    /// A template for the API URL to get information about issue events on the repository.
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    /// A template for the API URL to get information about issues on the repository.
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    /// A template for the API URL to get information about deploy keys on the repository.
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    /// A template for the API URL to get information about labels of the repository.
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    /// The API URL to get information about the languages of the repository.
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    /// The API URL to merge branches in the repository.
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    /// A template for the API URL to get information about milestones of the repository.
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    /// A template for the API URL to get information about notifications on the repository.
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    /// A template for the API URL to get information about pull requests on the repository.
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    /// A template for the API URL to get information about releases on the repository.
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    /// The API URL to list the stargazers on the repository.
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    /// A template for the API URL to get information about statuses of a commit.
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    /// The API URL to list the subscribers on the repository.
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    /// The API URL to subscribe to notifications for this repository.
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    /// The API URL to get information about tags on the repository.
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    /// The API URL to list the teams on the repository.
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    /// A template for the API URL to create or retrieve a raw Git tree of the repository.
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    /// The API URL to list the hooks on the repository.
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
}

impl SimpleRepository {
    /// A GitHub repository.
    pub fn new(id: i32, node_id: String, name: String, full_name: String, owner: models::SimpleUser, private: bool, html_url: String, description: Option<String>, fork: bool, url: String, archive_url: String, assignees_url: String, blobs_url: String, branches_url: String, collaborators_url: String, comments_url: String, commits_url: String, compare_url: String, contents_url: String, contributors_url: String, deployments_url: String, downloads_url: String, events_url: String, forks_url: String, git_commits_url: String, git_refs_url: String, git_tags_url: String, issue_comment_url: String, issue_events_url: String, issues_url: String, keys_url: String, labels_url: String, languages_url: String, merges_url: String, milestones_url: String, notifications_url: String, pulls_url: String, releases_url: String, stargazers_url: String, statuses_url: String, subscribers_url: String, subscription_url: String, tags_url: String, teams_url: String, trees_url: String, hooks_url: String) -> SimpleRepository {
        SimpleRepository {
            id,
            node_id,
            name,
            full_name,
            owner: Box::new(owner),
            private,
            html_url,
            description,
            fork,
            url,
            archive_url,
            assignees_url,
            blobs_url,
            branches_url,
            collaborators_url,
            comments_url,
            commits_url,
            compare_url,
            contents_url,
            contributors_url,
            deployments_url,
            downloads_url,
            events_url,
            forks_url,
            git_commits_url,
            git_refs_url,
            git_tags_url,
            issue_comment_url,
            issue_events_url,
            issues_url,
            keys_url,
            labels_url,
            languages_url,
            merges_url,
            milestones_url,
            notifications_url,
            pulls_url,
            releases_url,
            stargazers_url,
            statuses_url,
            subscribers_url,
            subscription_url,
            tags_url,
            teams_url,
            trees_url,
            hooks_url,
        }
    }
}
