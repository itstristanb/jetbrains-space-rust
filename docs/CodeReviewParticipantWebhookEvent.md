# CodeReviewParticipantWebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | Option<[**crate::models::KMetaMod**](KMetaMod.md)> |  | [optional]
**review** | [**crate::models::CodeReviewRecord**](CodeReviewRecord.md) |  | 
**is_merge_request** | **bool** |  | 
**participant** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**reviewer_state** | Option<[**crate::models::CodeReviewParticipantWebhookEventReviewerState**](CodeReviewParticipantWebhookEvent_reviewerState.md)> |  | [optional]
**their_turn** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


