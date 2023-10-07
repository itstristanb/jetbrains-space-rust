# ChatContactRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**key** | **String** |  | 
**details** | [**crate::models::ChatContactDetails**](ChatContactDetails.md) |  | 
**channel_type** | **String** |  | 
**last_message** | Option<[**crate::models::MessageInfo**](MessageInfo.md)> |  | [optional]
**unread_status** | Option<[**crate::models::M2UnreadStatus**](M2UnreadStatus.md)> |  | [optional]
**read_time** | Option<**String**> |  | [optional]
**subscribed_since** | **String** |  | 
**pinned** | **bool** |  | 
**pinned_prev_id** | Option<**String**> |  | [optional]
**draft** | Option<**String**> |  | [optional]
**draft_time** | Option<**i64**> |  | [optional]
**last_child_message_time** | Option<**i64**> |  | [optional]
**deleted** | Option<**bool**> |  | [optional]
**muted** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


