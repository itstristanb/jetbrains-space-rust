# Issue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**project_id** | Option<**String**> |  | [optional]
**project_ref** | [**crate::models::PrProject**](PR_Project.md) |  | 
**tracker_ref** | Option<[**crate::models::IssueTracker**](IssueTracker.md)> |  | [optional]
**number** | **i32** |  | 
**created_by** | [**crate::models::CPrincipal**](CPrincipal.md) |  | 
**creation_time** | **String** |  | 
**assignee** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**status** | [**crate::models::IssueStatus**](IssueStatus.md) |  | 
**due_date** | Option<**String**> |  | [optional]
**external_entity_info** | Option<[**crate::models::ExternalEntityInfoRecord**](ExternalEntityInfoRecord.md)> |  | [optional]
**tags** | [**Vec<crate::models::PlanningTag>**](PlanningTag.md) |  | 
**title** | **String** |  | 
**attachments_count** | Option<**i32**> |  | [optional]
**sub_items_count** | Option<**i32**> |  | [optional]
**done_sub_items_count** | Option<**i32**> |  | [optional]
**comments_count** | Option<**i32**> |  | [optional]
**deleted_by** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**deleted_time** | Option<**String**> |  | [optional]
**sprints** | [**Vec<crate::models::SprintRecord>**](SprintRecord.md) |  | 
**description** | Option<**String**> |  | [optional]
**channel** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**attachments** | [**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md) |  | 
**unfurls** | Option<[**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md)> |  | [optional]
**rt_description** | Option<[**crate::models::RtContent**](RtContent.md)> |  | [optional]
**parents** | [**Vec<crate::models::Issue>**](Issue.md) |  | 
**backlogs** | [**Vec<crate::models::BoardBacklog>**](BoardBacklog.md) |  | 
**checklists** | [**Vec<crate::models::Checklist>**](Checklist.md) |  | 
**sub_items_list** | [**crate::models::Checklist**](Checklist.md) |  | 
**message_permalink** | Option<**String**> |  | [optional]
**custom_fields** | [**::std::collections::HashMap<String, crate::models::CfValue>**](CFValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


