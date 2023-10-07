# CodeDiscussionRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**project_id** | **String** |  | 
**project** | Option<[**crate::models::PrProject**](PR_Project.md)> |  | [optional]
**anchor** | [**crate::models::CodeDiscussionAnchor**](CodeDiscussionAnchor.md) |  | 
**end_anchor** | Option<[**crate::models::CodeDiscussionAnchor**](CodeDiscussionAnchor.md)> |  | [optional]
**created** | **String** |  | 
**channel** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**resolvable** | Option<**bool**> |  | [optional]
**resolved** | **bool** |  | 
**snippet** | Option<[**crate::models::CodeDiscussionSnippet**](CodeDiscussionSnippet.md)> |  | [optional]
**suggested_edit** | Option<[**crate::models::CodeDiscussionSuggestedEdit**](CodeDiscussionSuggestedEdit.md)> |  | [optional]
**resolved_by** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**pending** | Option<**bool**> |  | [optional]
**review** | Option<[**crate::models::CodeReviewRecord**](CodeReviewRecord.md)> |  | [optional]
**feed_item_id** | Option<**String**> |  | [optional]
**reviews** | Option<[**Vec<crate::models::CodeReviewRecord>**](CodeReviewRecord.md)> |  | [optional]
**archived** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


