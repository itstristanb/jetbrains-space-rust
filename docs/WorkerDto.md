# WorkerDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**owner** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**owner_principal** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**info** | [**crate::models::WorkerInfoDto**](WorkerInfoDTO.md) |  | 
**compute_pool** | [**crate::models::WorkerComputePoolDto**](WorkerComputePoolDTO.md) |  | 
**version** | [**crate::models::WorkerVersionDto**](WorkerVersionDTO.md) |  | 
**last_access_time** | Option<**String**> |  | [optional]
**status** | [**crate::models::WorkerStatus**](WorkerStatus.md) |  | 
**scope** | [**crate::models::WorkerScope**](WorkerScope.md) |  | 
**suspended** | **bool** |  | 
**projects** | Option<[**Vec<crate::models::PrProject>**](PR_Project.md)> |  | [optional]
**tags** | [**Vec<crate::models::WorkerTagDto>**](WorkerTagDTO.md) |  | 
**steps_stats** | [**crate::models::WorkerStepsStatsDto**](WorkerStepsStatsDTO.md) |  | 
**worker_stats** | [**crate::models::WorkerStatsDto**](WorkerStatsDTO.md) |  | 
**worker_capabilities** | [**Vec<crate::models::WorkerCapability>**](WorkerCapability.md) |  | 
**permanent_token_info** | Option<[**crate::models::WorkerPermanentTokenInfoDto**](WorkerPermanentTokenInfoDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


