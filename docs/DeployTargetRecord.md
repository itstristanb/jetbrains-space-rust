# DeployTargetRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**project_id** | **String** |  | 
**name** | **String** |  | 
**key** | **String** |  | 
**description** | **String** |  | 
**created_at** | **String** |  | 
**last_updated** | Option<**String**> |  | [optional]
**last_deployed** | Option<**String**> |  | [optional]
**channel** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**number** | Option<**i32**> |  | [optional]
**full_number_id** | Option<**String**> |  | [optional]
**archived** | **bool** |  | 
**links** | [**Vec<crate::models::DeployTargetLink>**](DeployTargetLink.md) |  | 
**current** | Option<[**crate::models::DeploymentInfo**](DeploymentInfo.md)> |  | [optional]
**next** | Option<[**crate::models::DeploymentInfo**](DeploymentInfo.md)> |  | [optional]
**created_by** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**channel_id** | **String** |  | 
**repositories** | [**Vec<crate::models::DeployTargetRepositoryDto>**](DeployTargetRepositoryDTO.md) |  | 
**manual_control** | **bool** |  | 
**single_scheduled** | Option<**bool**> |  | [optional]
**hang_timeout_minutes** | Option<**i32**> |  | [optional]
**fail_timeout_minutes** | Option<**i32**> |  | [optional]
**connected_channel_id** | Option<**String**> |  | [optional]
**users** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**teams** | [**Vec<crate::models::TdTeam>**](TD_Team.md) |  | 
**custom_fields** | [**::std::collections::HashMap<String, crate::models::CfValue>**](CFValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


