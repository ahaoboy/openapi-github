# WebhooksRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_enforced** | **bool** |  | 
**allow_deletions_enforcement_level** | **String** |  | 
**allow_force_pushes_enforcement_level** | **String** |  | 
**authorized_actor_names** | **Vec<String>** |  | 
**authorized_actors_only** | **bool** |  | 
**authorized_dismissal_actors_only** | **bool** |  | 
**create_protected** | Option<**bool**> |  | [optional]
**created_at** | **String** |  | 
**dismiss_stale_reviews_on_push** | **bool** |  | 
**id** | **i32** |  | 
**ignore_approvals_from_contributors** | **bool** |  | 
**linear_history_requirement_enforcement_level** | **String** |  | 
**merge_queue_enforcement_level** | **String** |  | 
**name** | **String** |  | 
**pull_request_reviews_enforcement_level** | **String** |  | 
**repository_id** | **i32** |  | 
**require_code_owner_review** | **bool** |  | 
**require_last_push_approval** | Option<**bool**> | Whether the most recent push must be approved by someone other than the person who pushed it | [optional]
**required_approving_review_count** | **i32** |  | 
**required_conversation_resolution_level** | **String** |  | 
**required_deployments_enforcement_level** | **String** |  | 
**required_status_checks** | **Vec<String>** |  | 
**required_status_checks_enforcement_level** | **String** |  | 
**signature_requirement_enforcement_level** | **String** |  | 
**strict_required_status_checks_policy** | **bool** |  | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


