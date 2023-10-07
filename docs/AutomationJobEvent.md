# AutomationJobEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | [**crate::models::KMetaMod**](KMetaMod.md) |  | 
**execution_id** | **String** |  | 
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**repository_name** | **String** |  | 
**job_name** | **String** |  | 
**execution_number** | **i64** |  | 
**trigger** | [**crate::models::JobExecutionTrigger**](JobExecutionTrigger.md) |  | 
**trigger_time** | **String** |  | 
**status** | Option<[**crate::models::AutomationJobEventStatus**](AutomationJobEvent_status.md)> |  | [optional]
**failure_reasons** | Option<[**crate::models::AutomationJobEventFailureReasons**](AutomationJobEvent_failureReasons.md)> |  | [optional]
**stopped_by** | Option<[**crate::models::AutomationJobEventStoppedBy**](AutomationJobEvent_stoppedBy.md)> |  | [optional]
**start_time** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**end_time** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**repositories** | Option<[**Vec<crate::models::GitCheckout>**](GitCheckout.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


