# CalendarsMeetingsIdPatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**occurrence_rule** | Option<[**crate::models::CalendarEventSpec**](CalendarEventSpec.md)> |  | [optional]
**locations_diff** | Option<[**crate::models::Diff**](Diff.md)> |  | [optional]
**profiles_diff** | Option<[**crate::models::Diff**](Diff.md)> |  | [optional]
**external_participants_diff** | Option<[**crate::models::Diff**](Diff.md)> |  | [optional]
**teams_diff** | Option<[**crate::models::Diff**](Diff.md)> |  | [optional]
**visibility** | Option<[**crate::models::MeetingVisibility**](MeetingVisibility.md)> |  | [optional]
**modification_preference** | Option<[**crate::models::MeetingModificationPreference**](MeetingModificationPreference.md)> |  | [optional]
**joining_preference** | Option<[**crate::models::MeetingJoiningPreference**](MeetingJoiningPreference.md)> |  | [optional]
**notify_on_export** | Option<**bool**> |  | [optional][default to true]
**organizer** | Option<**String**> |  | [optional]
**target_date** | Option<**String**> |  | [optional]
**modification_kind** | Option<[**crate::models::RecurrentModification**](RecurrentModification.md)> |  | [optional]
**conference_data** | Option<[**crate::models::EventConferenceData**](EventConferenceData.md)> |  | [optional]
**attachments** | Option<[**Vec<crate::models::MeetingAttachment>**](MeetingAttachment.md)> |  | [optional]
**calendar_id** | Option<[**crate::models::CalendarIdentifier**](CalendarIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


