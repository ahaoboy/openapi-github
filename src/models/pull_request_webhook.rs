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
pub struct PullRequestWebhook {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "review_comments_url")]
    pub review_comments_url: String,
    #[serde(rename = "review_comment_url")]
    pub review_comment_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    /// Number uniquely identifying the pull request within its repository.
    #[serde(rename = "number")]
    pub number: i32,
    /// State of this Pull Request. Either `open` or `closed`.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "locked")]
    pub locked: bool,
    /// The title of the pull request.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "user")]
    pub user: Box<models::SimpleUser>,
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "labels")]
    pub labels: Vec<models::PullRequestLabelsInner>,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<Box<models::NullableMilestone>>,
    #[serde(rename = "active_lock_reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<Option<String>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "merged_at", deserialize_with = "Option::deserialize")]
    pub merged_at: Option<String>,
    #[serde(rename = "merge_commit_sha", deserialize_with = "Option::deserialize")]
    pub merge_commit_sha: Option<String>,
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "assignees", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Option<Vec<models::SimpleUser>>>,
    #[serde(rename = "requested_reviewers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub requested_reviewers: Option<Option<Vec<models::SimpleUser>>>,
    #[serde(rename = "requested_teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub requested_teams: Option<Option<Vec<models::TeamSimple>>>,
    #[serde(rename = "head")]
    pub head: Box<models::PullRequestHead>,
    #[serde(rename = "base")]
    pub base: Box<models::PullRequestBase>,
    #[serde(rename = "_links")]
    pub _links: Box<models::PullRequestSimpleLinks>,
    #[serde(rename = "author_association")]
    pub author_association: models::AuthorAssociation,
    #[serde(rename = "auto_merge", deserialize_with = "Option::deserialize")]
    pub auto_merge: Option<Box<models::AutoMerge>>,
    /// Indicates whether or not the pull request is a draft.
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "merged")]
    pub merged: bool,
    #[serde(rename = "mergeable", deserialize_with = "Option::deserialize")]
    pub mergeable: Option<bool>,
    #[serde(rename = "rebaseable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<Option<bool>>,
    #[serde(rename = "mergeable_state")]
    pub mergeable_state: String,
    #[serde(rename = "merged_by", deserialize_with = "Option::deserialize")]
    pub merged_by: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "comments")]
    pub comments: i32,
    #[serde(rename = "review_comments")]
    pub review_comments: i32,
    /// Indicates whether maintainers can modify the pull request.
    #[serde(rename = "maintainer_can_modify")]
    pub maintainer_can_modify: bool,
    #[serde(rename = "commits")]
    pub commits: i32,
    #[serde(rename = "additions")]
    pub additions: i32,
    #[serde(rename = "deletions")]
    pub deletions: i32,
    #[serde(rename = "changed_files")]
    pub changed_files: i32,
    /// Whether to allow auto-merge for pull requests.
    #[serde(rename = "allow_auto_merge", skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    /// Whether to allow updating the pull request's branch.
    #[serde(rename = "allow_update_branch", skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    /// Whether to delete head branches when pull requests are merged.
    #[serde(rename = "delete_branch_on_merge", skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// The default value for a merge commit message. - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message.
    #[serde(rename = "merge_commit_message", skip_serializing_if = "Option::is_none")]
    pub merge_commit_message: Option<MergeCommitMessage>,
    /// The default value for a merge commit title. - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., \"Merge pull request #123 from branch-name\").
    #[serde(rename = "merge_commit_title", skip_serializing_if = "Option::is_none")]
    pub merge_commit_title: Option<MergeCommitTitle>,
    /// The default value for a squash merge commit message: - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message.
    #[serde(rename = "squash_merge_commit_message", skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    /// The default value for a squash merge commit title: - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
    #[serde(rename = "squash_merge_commit_title", skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    /// Whether a squash merge commit can use the pull request title as default. **This property has been deprecated. Please use `squash_merge_commit_title` instead.**
    #[serde(rename = "use_squash_pr_title_as_default", skip_serializing_if = "Option::is_none")]
    pub use_squash_pr_title_as_default: Option<bool>,
}

impl PullRequestWebhook {
    pub fn new(url: String, id: i32, node_id: String, html_url: String, diff_url: String, patch_url: String, issue_url: String, commits_url: String, review_comments_url: String, review_comment_url: String, comments_url: String, statuses_url: String, number: i32, state: State, locked: bool, title: String, user: models::SimpleUser, body: Option<String>, labels: Vec<models::PullRequestLabelsInner>, milestone: Option<models::NullableMilestone>, created_at: String, updated_at: String, closed_at: Option<String>, merged_at: Option<String>, merge_commit_sha: Option<String>, assignee: Option<models::NullableSimpleUser>, head: models::PullRequestHead, base: models::PullRequestBase, _links: models::PullRequestSimpleLinks, author_association: models::AuthorAssociation, auto_merge: Option<models::AutoMerge>, merged: bool, mergeable: Option<bool>, mergeable_state: String, merged_by: Option<models::NullableSimpleUser>, comments: i32, review_comments: i32, maintainer_can_modify: bool, commits: i32, additions: i32, deletions: i32, changed_files: i32) -> PullRequestWebhook {
        PullRequestWebhook {
            url,
            id,
            node_id,
            html_url,
            diff_url,
            patch_url,
            issue_url,
            commits_url,
            review_comments_url,
            review_comment_url,
            comments_url,
            statuses_url,
            number,
            state,
            locked,
            title,
            user: Box::new(user),
            body,
            labels,
            milestone: milestone.map(Box::new),
            active_lock_reason: None,
            created_at,
            updated_at,
            closed_at,
            merged_at,
            merge_commit_sha,
            assignee: assignee.map(Box::new),
            assignees: None,
            requested_reviewers: None,
            requested_teams: None,
            head: Box::new(head),
            base: Box::new(base),
            _links: Box::new(_links),
            author_association,
            auto_merge: auto_merge.map(Box::new),
            draft: None,
            merged,
            mergeable,
            rebaseable: None,
            mergeable_state,
            merged_by: merged_by.map(Box::new),
            comments,
            review_comments,
            maintainer_can_modify,
            commits,
            additions,
            deletions,
            changed_files,
            allow_auto_merge: None,
            allow_update_branch: None,
            delete_branch_on_merge: None,
            merge_commit_message: None,
            merge_commit_title: None,
            squash_merge_commit_message: None,
            squash_merge_commit_title: None,
            use_squash_pr_title_as_default: None,
        }
    }
}
/// State of this Pull Request. Either `open` or `closed`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}
/// The default value for a merge commit message. - `PR_TITLE` - default to the pull request's title. - `PR_BODY` - default to the pull request's body. - `BLANK` - default to a blank commit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeCommitMessage {
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "BLANK")]
    Blank,
}

impl Default for MergeCommitMessage {
    fn default() -> MergeCommitMessage {
        Self::PrBody
    }
}
/// The default value for a merge commit title. - `PR_TITLE` - default to the pull request's title. - `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., \"Merge pull request #123 from branch-name\").
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeCommitTitle {
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "MERGE_MESSAGE")]
    MergeMessage,
}

impl Default for MergeCommitTitle {
    fn default() -> MergeCommitTitle {
        Self::PrTitle
    }
}
/// The default value for a squash merge commit message: - `PR_BODY` - default to the pull request's body. - `COMMIT_MESSAGES` - default to the branch's commit messages. - `BLANK` - default to a blank commit message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SquashMergeCommitMessage {
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "COMMIT_MESSAGES")]
    CommitMessages,
    #[serde(rename = "BLANK")]
    Blank,
}

impl Default for SquashMergeCommitMessage {
    fn default() -> SquashMergeCommitMessage {
        Self::PrBody
    }
}
/// The default value for a squash merge commit title: - `PR_TITLE` - default to the pull request's title. - `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SquashMergeCommitTitle {
    #[serde(rename = "PR_TITLE")]
    PrTitle,
    #[serde(rename = "COMMIT_OR_PR_TITLE")]
    CommitOrPrTitle,
}

impl Default for SquashMergeCommitTitle {
    fn default() -> SquashMergeCommitTitle {
        Self::PrTitle
    }
}

