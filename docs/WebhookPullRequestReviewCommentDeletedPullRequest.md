# WebhookPullRequestReviewCommentDeletedPullRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_links** | [**models::WebhooksPullRequest5Links**](webhooks_pull_request_5__links.md) |  | 
**active_lock_reason** | Option<**String**> |  | 
**assignee** | Option<[**models::User**](User.md)> |  | 
**assignees** | [**Vec<models::User>**](User.md) |  | 
**author_association** | **String** | How the author is associated with the repository. | 
**auto_merge** | Option<[**models::PullRequestAutoMerge**](PullRequestAutoMerge.md)> |  | [optional]
**base** | [**models::PullRequestBase**](Pull_Request_base.md) |  | 
**body** | Option<**String**> |  | 
**closed_at** | Option<**String**> |  | 
**comments_url** | **String** |  | 
**commits_url** | **String** |  | 
**created_at** | **String** |  | 
**diff_url** | **String** |  | 
**draft** | Option<**bool**> |  | [optional]
**head** | [**models::WebhookPullRequestReviewCommentDeletedPullRequestHead**](webhook_pull_request_review_comment_deleted_pull_request_head.md) |  | 
**html_url** | **String** |  | 
**id** | **i32** |  | 
**issue_url** | **String** |  | 
**labels** | [**Vec<models::Label>**](Label.md) |  | 
**locked** | **bool** |  | 
**merge_commit_sha** | Option<**String**> |  | 
**merged_at** | Option<**String**> |  | 
**milestone** | Option<[**models::Milestone1**](Milestone_1.md)> |  | 
**node_id** | **String** |  | 
**number** | **i32** |  | 
**patch_url** | **String** |  | 
**requested_reviewers** | [**Vec<models::WebhooksPullRequest5RequestedReviewersInner>**](webhooks_pull_request_5_requested_reviewers_inner.md) |  | 
**requested_teams** | [**Vec<models::Team>**](Team.md) |  | 
**review_comment_url** | **String** |  | 
**review_comments_url** | **String** |  | 
**state** | **String** |  | 
**statuses_url** | **String** |  | 
**title** | **String** |  | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**user** | Option<[**models::User1**](User_1.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

