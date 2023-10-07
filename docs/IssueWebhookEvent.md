# IssueWebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | [**crate::models::KMetaMod**](KMetaMod.md) |  | 
**issue** | [**crate::models::Issue**](Issue.md) |  | 
**title** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**description** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**assignee** | Option<[**crate::models::ApplicationWebhookEventOwner**](ApplicationWebhookEvent_owner.md)> |  | [optional]
**status** | Option<[**crate::models::IssueWebhookEventStatus**](IssueWebhookEvent_status.md)> |  | [optional]
**due_date** | Option<[**crate::models::AbsenceWebhookEventSince**](AbsenceWebhookEvent_since.md)> |  | [optional]
**tag_delta** | Option<[**crate::models::IssueWebhookEventTagDelta**](IssueWebhookEvent_tagDelta.md)> |  | [optional]
**checklist_delta** | Option<[**crate::models::IssueWebhookEventChecklistDelta**](IssueWebhookEvent_checklistDelta.md)> |  | [optional]
**sprint_delta** | Option<[**crate::models::IssueWebhookEventSprintDelta**](IssueWebhookEvent_sprintDelta.md)> |  | [optional]
**custom_field_update** | Option<[**crate::models::IssueWebhookCustomFieldUpdate**](IssueWebhookCustomFieldUpdate.md)> |  | [optional]
**deleted** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


