# ReviewCreatedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_key** | **String** |  | 
**review_id** | **String** |  | 
**review_number** | **i32** |  | 
**review_type** | [**crate::models::ReviewType**](ReviewType.md) |  | 
**description** | Option<[**crate::models::CodeReviewDescription**](CodeReviewDescription.md)> |  | [optional]
**description_edited_by_profile_ids** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**commits** | Option<[**Vec<crate::models::UnfurlDetailsCommit>**](UnfurlDetailsCommit.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


