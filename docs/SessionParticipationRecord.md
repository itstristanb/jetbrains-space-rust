# SessionParticipationRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**connection_id** | **i64** |  | 
**session_id** | **String** |  | 
**member** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**state** | [**crate::models::ParticipationState**](ParticipationState.md) |  | 
**participant** | Option<[**crate::models::TdCallParticipant**](TD_CallParticipant.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**producers** | [**Vec<crate::models::ProducerOptions>**](ProducerOptions.md) |  | 
**data_producers** | [**Vec<crate::models::DataProducerOptions>**](DataProducerOptions.md) |  | 
**version** | **i64** |  | 
**data** | [**crate::models::ParticipantStateData**](ParticipantStateData.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


