# ArticleRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**title** | **String** |  | 
**created** | **String** |  | 
**author** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**aliases** | [**Vec<crate::models::BgArticleAlias>**](BG_ArticleAlias.md) |  | 
**archived_by** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**archived_at** | Option<**String**> |  | [optional]
**channel** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**channel_content** | Option<[**crate::models::M2ChannelContentRecord**](M2ChannelContentRecord.md)> |  | [optional]
**reactions** | [**crate::models::AllReactionsToItemRecord**](AllReactionsToItemRecord.md) |  | 
**content** | **String** |  | 
**event** | Option<[**crate::models::MeetingRecord**](MeetingRecord.md)> |  | [optional]
**teams** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**locations** | Option<[**Vec<crate::models::TdLocation>**](TD_Location.md)> |  | [optional]
**external_entity_info** | Option<[**crate::models::ExternalEntityInfoRecord**](ExternalEntityInfoRecord.md)> |  | [optional]
**doc_content** | [**crate::models::MdTextDocumentContent**](MdTextDocumentContent.md) |  | 
**attachments** | [**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md) |  | 
**preview_images** | [**Vec<crate::models::ArticleMarkdownImage>**](ArticleMarkdownImage.md) |  | 
**preview** | **String** |  | 
**preview_attachments** | Option<[**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md)> |  | [optional]
**words_number** | Option<**i32**> |  | [optional]
**cut** | Option<**bool**> |  | [optional]
**editable** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


