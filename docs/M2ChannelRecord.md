# M2ChannelRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**contact** | [**crate::models::M2ChannelContact**](M2ChannelContact.md) |  | 
**total_messages** | **i32** |  | 
**last_message** | Option<[**crate::models::MessageInfo**](MessageInfo.md)> |  | [optional]
**participants** | Option<[**Vec<crate::models::ChannelParticipant>**](ChannelParticipant.md)> |  | [optional]
**channel_archived** | Option<**bool**> |  | [optional]
**archived** | **bool** |  | 
**pinned_messages** | Option<[**Vec<crate::models::ChannelItemRecord>**](ChannelItemRecord.md)> |  | [optional]
**content** | Option<[**crate::models::M2ChannelContentInfo**](M2ChannelContentInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


