# JobExecutionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_id** | **String** |  | 
**execution_number** | **i64** |  | 
**job_id** | **String** |  | 
**job_name** | **String** |  | 
**project_id** | **String** |  | 
**branch** | **String** |  | 
**status** | [**crate::models::ExecutionStatus**](ExecutionStatus.md) |  | 
**triggered_time** | **i64** |  | 
**started_time** | Option<**i64**> |  | [optional]
**finished_time** | Option<**i64**> |  | [optional]
**changes_count** | **i32** |  | 
**failure_conditions** | [**Vec<crate::models::FailureConditionDto>**](FailureConditionDTO.md) |  | 
**commit_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


