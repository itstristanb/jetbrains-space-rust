# CalendarsMeetingsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**occurrence_rule** | [**crate::models::CalendarEventSpec**](CalendarEventSpec.md) |  | 
**locations** | Option<**Vec<String>**> |  | [optional][default to []]
**profiles** | Option<**Vec<String>**> |  | [optional][default to []]
**external_participants** | Option<**Vec<String>**> |  | [optional][default to []]
**teams** | Option<**Vec<String>**> |  | [optional][default to []]
**visibility** | Option<[**crate::models::MeetingVisibility**](MeetingVisibility.md)> |  | [optional]
**modification_preference** | Option<[**crate::models::MeetingModificationPreference**](MeetingModificationPreference.md)> |  | [optional]
**joining_preference** | Option<[**crate::models::MeetingJoiningPreference**](MeetingJoiningPreference.md)> |  | [optional]
**notify_on_export** | Option<**bool**> |  | [optional][default to true]
**organizer** | Option<**String**> |  | [optional]
**conference_data** | Option<[**crate::models::EventConferenceData**](EventConferenceData.md)> |  | [optional]
**attachments** | Option<[**Vec<crate::models::MeetingAttachment>**](MeetingAttachment.md)> |  | [optional]
**calendar_id** | Option<[**crate::models::CalendarIdentifier**](CalendarIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


