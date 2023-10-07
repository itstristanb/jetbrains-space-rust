# DtoMeeting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**summary** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**locations** | [**Vec<crate::models::TdLocation>**](TD_Location.md) |  | 
**profiles** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**teams** | [**Vec<crate::models::TdTeam>**](TD_Team.md) |  | 
**occurrence_rule** | [**crate::models::CalendarEventSpec**](CalendarEventSpec.md) |  | 
**origin** | [**crate::models::MeetingOrigin**](MeetingOrigin.md) |  | 
**conference_link** | Option<**String**> |  | [optional]
**visibility** | [**crate::models::MeetingVisibility**](MeetingVisibility.md) |  | 
**modification_preference** | [**crate::models::MeetingModificationPreference**](MeetingModificationPreference.md) |  | 
**joining_preference** | Option<[**crate::models::MeetingJoiningPreference**](MeetingJoiningPreference.md)> |  | [optional]
**organizer** | Option<[**crate::models::MeetingOrganizer**](MeetingOrganizer.md)> |  | [optional]
**etag** | **i64** |  | 
**private_data_substituted** | **bool** |  | 
**can_modify** | **bool** |  | 
**can_delete** | **bool** |  | 
**can_join** | **bool** |  | 
**external_participants** | **Vec<String>** |  | 
**link_to_external_source** | Option<**String**> |  | [optional]
**event_attachments** | Option<[**Vec<crate::models::MeetingAttachment>**](MeetingAttachment.md)> |  | [optional]
**conference_data** | Option<[**crate::models::EventConferenceData**](EventConferenceData.md)> |  | [optional]
**channel_ref** | Option<[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)> |  | [optional]
**external_source** | Option<[**crate::models::EventExternalSource**](EventExternalSource.md)> |  | [optional]
**calendar** | Option<[**crate::models::CalendarInfo**](CalendarInfo.md)> |  | [optional]
**can_leave_or_rsvp** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


