# ProjectsProjectCodeReviewsCodeDiscussionsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | **String** |  | 
**attachments** | Option<[**Vec<crate::models::AttachmentIn>**](AttachmentIn.md)> |  | [optional]
**diff_context** | Option<[**crate::models::DiffContext**](DiffContext.md)> |  | [optional]
**repository** | **String** |  | 
**revision** | Option<**String**> |  | [optional]
**filename** | Option<**String**> |  | [optional]
**line** | Option<**i32**> |  | [optional]
**old_line** | Option<**i32**> |  | [optional]
**anchor** | Option<[**crate::models::LocalCodeDiscussionAnchorIn**](LocalCodeDiscussionAnchorIn.md)> |  | [optional]
**end_anchor** | Option<[**crate::models::LocalCodeDiscussionAnchorIn**](LocalCodeDiscussionAnchorIn.md)> |  | [optional]
**pending** | Option<**bool**> |  | [optional][default to false]
**review_id** | [**crate::models::ReviewIdentifier**](ReviewIdentifier.md) |  | 
**suggested_edit** | Option<[**crate::models::CodeDiscussionSuggestedEditRequest**](CodeDiscussionSuggestedEditRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


