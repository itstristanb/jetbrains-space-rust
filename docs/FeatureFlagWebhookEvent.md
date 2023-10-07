# FeatureFlagWebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | [**crate::models::KMetaMod**](KMetaMod.md) |  | 
**name** | **String** |  | 
**issue_number** | Option<**i32**> |  | [optional]
**enabled_for_all** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**self_manageable** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**added_teams** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**added_profiles** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**removed_teams** | Option<[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)> |  | [optional]
**removed_profiles** | Option<[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


