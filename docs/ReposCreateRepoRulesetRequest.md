# ReposCreateRepoRulesetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the ruleset. | 
**target** | Option<**String**> | The target of the ruleset  **Note**: The `push` target is in beta and is subject to change. | [optional]
**enforcement** | [**models::RepositoryRuleEnforcement**](repository-rule-enforcement.md) |  | 
**bypass_actors** | Option<[**Vec<models::RepositoryRulesetBypassActor>**](repository-ruleset-bypass-actor.md)> | The actors that can bypass the rules in this ruleset | [optional]
**conditions** | Option<[**models::RepositoryRulesetConditions**](repository-ruleset-conditions.md)> |  | [optional]
**rules** | Option<[**Vec<models::RepositoryRule>**](repository-rule.md)> | An array of rules within the ruleset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


