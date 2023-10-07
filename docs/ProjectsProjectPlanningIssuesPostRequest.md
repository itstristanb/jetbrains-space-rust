# ProjectsProjectPlanningIssuesPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**assignee** | Option<[**crate::models::ProfileIdentifier**](ProfileIdentifier.md)> |  | [optional]
**status** | **String** |  | 
**due_date** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional][default to []]
**checklists** | Option<**Vec<String>**> |  | [optional][default to []]
**sprints** | Option<[**Vec<crate::models::SprintIdentifier>**](SprintIdentifier.md)> |  | [optional][default to []]
**attachments** | Option<[**Vec<crate::models::AttachmentIn>**](AttachmentIn.md)> |  | [optional][default to []]
**from_message** | Option<[**crate::models::MessageLink**](MessageLink.md)> |  | [optional]
**custom_fields** | Option<[**Vec<crate::models::CustomFieldInputValue>**](CustomFieldInputValue.md)> |  | [optional]
**parents** | Option<[**Vec<crate::models::IssueIdentifier>**](IssueIdentifier.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


