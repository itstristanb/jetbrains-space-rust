# DeploymentInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
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
**description** | Option<**String**> |  | [optional]
**channel** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**target** | [**crate::models::DeployTargetRecord**](DeployTargetRecord.md) |  | 
**job_ids** | Option<**Vec<String>**> |  | [optional]
**archived** | **bool** |  | 
**commits_added** | Option<**i32**> |  | [optional]
**merges_added** | Option<**i32**> |  | [optional]
**issues_added** | Option<**i32**> |  | [optional]
**commits_reverted** | Option<**i32**> |  | [optional]
**merges_reverted** | Option<**i32**> |  | [optional]
**issues_reverted** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


