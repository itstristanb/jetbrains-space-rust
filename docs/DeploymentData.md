# DeploymentData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**version** | **String** |  | 
**created_at** | **String** |  | 
**scheduled_start** | Option<**String**> |  | [optional]
**started_at** | Option<**String**> |  | [optional]
**finished_at** | Option<**String**> |  | [optional]
**target_key** | **String** |  | 
**status** | [**crate::models::DeploymentStatus**](DeploymentStatus.md) |  | 
**sync_status** | [**crate::models::DeploymentSyncStatus**](DeploymentSyncStatus.md) |  | 
**external_link** | Option<[**crate::models::ExternalLink**](ExternalLink.md)> |  | [optional]
**commit_refs** | [**Vec<crate::models::DeploymentCommitRefDetails>**](DeploymentCommitRefDetails.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


