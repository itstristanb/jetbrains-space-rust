# CallSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**room** | [**crate::models::Room**](Room.md) |  | 
**description** | **String** |  | 
**start** | **String** |  | 
**end** | Option<**String**> |  | [optional]
**channel** | Option<[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)> |  | [optional]
**topology** | Option<[**crate::models::ConnectionTopology**](ConnectionTopology.md)> |  | [optional]
**resources_prepared** | **bool** |  | 
**active** | Option<**bool**> |  | [optional]
**initial_session** | Option<[**crate::models::CallSession**](CallSession.md)> |  | [optional]
**sub_sessions** | [**Vec<crate::models::CallSession>**](CallSession.md) |  | 
**participations** | [**Vec<crate::models::SessionParticipationRecord>**](SessionParticipationRecord.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


