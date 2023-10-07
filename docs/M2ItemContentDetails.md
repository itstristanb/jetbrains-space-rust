# M2ItemContentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**team** | [**crate::models::TdTeam**](TD_Team.md) |  | 
**sticker** | [**crate::models::Sticker**](Sticker.md) |  | 
**pack** | Option<[**crate::models::StickerPackInfo**](StickerPackInfo.md)> |  | [optional]
**absence** | [**crate::models::AbsenceRecord**](AbsenceRecord.md) |  | 
**by** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**approve** | **bool** |  | 
**code_discussion** | [**crate::models::CodeDiscussionRecord**](CodeDiscussionRecord.md) |  | 
**accepted** | Option<**bool**> |  | [optional]
**uid** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**change_type** | [**crate::models::ReviewRevisionsChangedType**](ReviewRevisionsChangedType.md) |  | 
**project_key** | Option<**String**> |  | 
**review_id** | **String** |  | 
**review_number** | **i32** |  | 
**review_type** | [**crate::models::ReviewType**](ReviewType.md) |  | 
**description** | Option<**String**> |  | [optional]
**description_edited_by_profile_ids** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**commits** | [**Vec<crate::models::RepositoryCommitRecord>**](RepositoryCommitRecord.md) |  | 
**code_review** | [**crate::models::CodeReviewRecord**](CodeReviewRecord.md) |  | 
**discussion** | [**crate::models::CodeReviewDiscussionRecord**](CodeReviewDiscussionRecord.md) |  | 
**repository** | **String** |  | 
**branch** | **String** |  | 
**branch_type** | [**crate::models::MergeRequestBranchType**](MergeRequestBranchType.md) |  | 
**source_branch** | **String** |  | 
**target_branch** | **String** |  | 
**state** | [**crate::models::ReviewerState**](ReviewerState.md) |  | 
**review** | Option<[**crate::models::CodeReviewRecord**](CodeReviewRecord.md)> |  | [optional]
**revision** | **String** |  | 
**revision_link** | **String** |  | 
**tool_name** | **String** |  | 
**issues_count_by_level** | [**Vec<crate::models::MergeRequestCodeIssueStatsForLevel>**](MergeRequestCodeIssueStatsForLevel.md) |  | 
**old_title** | **String** |  | 
**new_title** | **String** |  | 
**track** | **bool** |  | 
**style** | Option<[**crate::models::MessageStyle**](MessageStyle.md)> |  | [optional]
**outline** | Option<[**crate::models::McOutline**](MCOutline.md)> |  | [optional]
**content** | [**Vec<crate::models::McElement>**](MCElement.md) |  | 
**extension** | Option<[**crate::models::M2ItemContentDetails**](M2ItemContentDetails.md)> |  | [optional]
**old_assignee** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**new_assignee** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**issue** | Option<[**crate::models::Issue**](Issue.md)> |  | [optional]
**origin_message** | Option<[**crate::models::MessageLink**](MessageLink.md)> |  | [optional]
**added_tags** | Option<[**Vec<crate::models::PlanningTag>**](PlanningTag.md)> |  | [optional]
**removed_tags** | Option<[**Vec<crate::models::PlanningTag>**](PlanningTag.md)> |  | [optional]
**added_names** | Option<**Vec<String>**> |  | [optional]
**removed_names** | Option<**Vec<String>**> |  | [optional]
**old_status** | [**crate::models::IssueStatus**](IssueStatus.md) |  | 
**new_status** | [**crate::models::IssueStatus**](IssueStatus.md) |  | 
**old_due_date** | Option<**String**> |  | [optional]
**new_due_date** | Option<**String**> |  | [optional]
**old_description** | Option<**String**> |  | [optional]
**new_description** | Option<**String**> |  | [optional]
**message** | **String** |  | 
**added_checklists** | Option<[**Vec<crate::models::Checklist>**](Checklist.md)> |  | [optional]
**removed_checklists** | Option<[**Vec<crate::models::Checklist>**](Checklist.md)> |  | [optional]
**id** | **String** |  | 
**title** | **String** |  | 
**deployment** | [**crate::models::DeploymentRecord**](DeploymentRecord.md) |  | 
**task_execution_id** | **String** |  | 
**task_execution_name** | **String** |  | 
**repo_name** | **String** |  | 
**branch_name** | **String** |  | 
**commit** | **String** |  | 
**short_commit_message** | **String** |  | 
**project** | [**crate::models::ProjectKey**](ProjectKey.md) |  | 
**finish_date_time** | **i64** |  | 
**trigger_info** | **String** |  | 
**details** | **String** |  | 
**member** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**markdown** | Option<**bool**> |  | [optional]
**mentions** | Option<[**Vec<crate::models::EntityMention>**](EntityMention.md)> |  | [optional]
**project_id** | Option<**String**> |  | [optional]
**revision_info** | Option<[**crate::models::RevisionAuthorInfo**](RevisionAuthorInfo.md)> |  | [optional]
**changes_info** | Option<[**crate::models::LastChanges**](LastChanges.md)> |  | [optional]
**url** | **String** |  | 
**external_service_name** | **String** |  | 
**task_name** | **String** |  | 
**timestamp** | Option<**i64**> |  | [optional]
**principals** | [**Vec<crate::models::CPrincipal>**](CPrincipal.md) |  | 
**others_display_names** | **Vec<String>** |  | 
**poll** | [**crate::models::PollRecord**](PollRecord.md) |  | 
**article** | [**crate::models::ArticleRecord**](ArticleRecord.md) |  | 
**article_preview** | [**crate::models::ArticlePreviewRecord**](ArticlePreviewRecord.md) |  | 
**article_details** | [**crate::models::ArticleDetailsRecord**](ArticleDetailsRecord.md) |  | 
**article_channel** | [**crate::models::ArticleChannelRecord**](ArticleChannelRecord.md) |  | 
**article_content** | [**crate::models::ArticleContentRecord**](ArticleContentRecord.md) |  | 
**action** | **String** |  | 
**success** | **bool** |  | 
**membership** | [**crate::models::TdMembership**](TD_Membership.md) |  | 
**leave** | **bool** |  | 
**session** | [**crate::models::CallSession**](CallSession.md) |  | 
**completed_text** | Option<**String**> |  | [optional]
**failed** | Option<**String**> |  | [optional]
**trace** | Option<**String**> |  | [optional]
**reason** | Option<[**crate::models::AbsenceWebhookEventReason**](AbsenceWebhookEvent_reason.md)> |  | [optional]
**since** | Option<[**crate::models::AbsenceWebhookEventSince**](AbsenceWebhookEvent_since.md)> |  | [optional]
**till** | Option<[**crate::models::AbsenceWebhookEventSince**](AbsenceWebhookEvent_since.md)> |  | [optional]
**pkg** | [**crate::models::PackageVersionInfo**](PackageVersionInfo.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


