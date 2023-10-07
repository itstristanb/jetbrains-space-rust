# ProjectsProjectPlanningIssuesImportPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | [**crate::models::ImportMetadata**](ImportMetadata.md) |  | 
**issues** | [**Vec<crate::models::ImportIssue>**](ImportIssue.md) |  | 
**assignee_missing_policy** | Option<[**crate::models::ImportMissingPolicy**](ImportMissingPolicy.md)> |  | [optional]
**status_missing_policy** | Option<[**crate::models::ImportMissingPolicy**](ImportMissingPolicy.md)> |  | [optional]
**on_exists_policy** | Option<[**crate::models::ImportExistsPolicy**](ImportExistsPolicy.md)> |  | [optional]
**dry_run** | **bool** |  | 
**notify_subscribers** | Option<**bool**> |  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


