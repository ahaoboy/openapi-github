# WebhookIssueCommentCreatedIssue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_lock_reason** | Option<**String**> |  | 
**assignee** | Option<[**models::User1**](User_1.md)> |  | 
**assignees** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**author_association** | **String** |  | 
**body** | Option<**String**> |  | 
**closed_at** | Option<**String**> |  | 
**comments** | **i32** |  | 
**comments_url** | **String** |  | 
**created_at** | **String** |  | 
**draft** | Option<**bool**> |  | [optional]
**events_url** | **String** |  | 
**html_url** | **String** |  | 
**id** | **i32** |  | 
**labels** | [**Vec<models::Label>**](Label.md) |  | 
**labels_url** | **String** |  | 
**locked** | **bool** |  | 
**milestone** | Option<[**serde_json::Value**](.md)> |  | 
**node_id** | **String** |  | 
**number** | **i32** |  | 
**performed_via_github_app** | Option<[**serde_json::Value**](.md)> |  | [optional]
**pull_request** | Option<[**models::WebhooksIssuePullRequest**](webhooks_issue_pull_request.md)> |  | [optional]
**reactions** | [**models::WebhookIssueCommentCreatedIssueAllOfReactions**](webhook_issue_comment_created_issue_allOf_reactions.md) |  | 
**repository_url** | **String** |  | 
**state** | **String** | State of the issue; either 'open' or 'closed' | 
**state_reason** | Option<**String**> |  | [optional]
**timeline_url** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**user** | [**models::WebhooksSponsorshipMaintainer**](webhooks_sponsorship_maintainer.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

