# ChannelItemRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** |  | 
**details** | Option<[**crate::models::M2ItemContentDetails**](M2ItemContentDetails.md)> |  | [optional]
**author** | [**crate::models::CPrincipal**](CPrincipal.md) |  | 
**created** | **String** |  | 
**time** | **i64** |  | 
**reactions** | Option<[**crate::models::AllReactionsToItemRecord**](AllReactionsToItemRecord.md)> |  | [optional]
**thread** | Option<[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)> |  | [optional]
**projected_item** | Option<[**crate::models::ChannelItemRecord**](ChannelItemRecord.md)> |  | [optional]
**attachments** | Option<[**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md)> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**pending** | Option<**bool**> |  | [optional]
**id** | **String** |  | 
**archived** | **bool** |  | 
**edited** | Option<**String**> |  | [optional]
**pinned** | Option<**bool**> |  | [optional]
**suggested_participants** | Option<[**Vec<crate::models::CPrincipal>**](CPrincipal.md)> |  | [optional]
**mentions** | Option<[**Vec<crate::models::EntityMention>**](EntityMention.md)> |  | [optional]
**channel_id** | Option<**String**> |  | [optional]
**importer_app_id** | Option<**String**> |  | [optional]
**issues** | [**Vec<crate::models::Issue>**](Issue.md) |  | 
**external_issues** | Option<[**Vec<crate::models::ExternalIssue>**](ExternalIssue.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


