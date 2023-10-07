# UnfurlDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**session** | [**crate::models::CallSession**](CallSession.md) |  | 
**icon** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**image** | [**crate::models::ImageAttachment**](ImageAttachment.md) |  | 
**id** | [**crate::models::ExternalIssueId**](ExternalIssueId.md) |  | 
**channel_id** | Option<**String**> |  | [optional]
**text** | **String** |  | 
**details** | Option<[**crate::models::ArticleDetailsRecord**](ArticleDetailsRecord.md)> |  | [optional]
**author** | [**crate::models::GitAuthorInfo**](GitAuthorInfo.md) |  | 
**created** | **String** |  | 
**time** | **i64** |  | 
**attachments** | Option<[**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md)> |  | [optional]
**mentions** | Option<[**Vec<crate::models::EntityMention>**](EntityMention.md)> |  | [optional]
**projected_item** | Option<[**crate::models::ChannelItemSnapshot**](ChannelItemSnapshot.md)> |  | [optional]
**article** | [**crate::models::ArticleRecord**](ArticleRecord.md) |  | 
**content** | Option<[**crate::models::ArticleContentRecord**](ArticleContentRecord.md)> |  | [optional]
**channel** | **String** |  | 
**message** | **String** |  | 
**inline_unfurls** | Option<[**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md)> |  | [optional]
**target_ref** | [**crate::models::DeployTargetRecord**](DeployTargetRecord.md) |  | 
**target_name** | Option<**String**> |  | [optional]
**show_link_icon** | Option<**bool**> |  | [optional]
**skip_details_render** | Option<**bool**> |  | [optional]
**review** | [**crate::models::CodeReviewRecord**](CodeReviewRecord.md) |  | 
**commits** | [**Vec<crate::models::UnfurlDetailsCommit>**](UnfurlDetailsCommit.md) |  | 
**total_commits_count** | **i32** |  | 
**job_execution_id** | **String** |  | 
**project_ref** | [**crate::models::PrProject**](PR_Project.md) |  | 
**repo_name** | **String** |  | 
**job_id** | **String** |  | 
**job_name** | **String** |  | 
**job_execution_display_status_filter** | Option<[**crate::models::JobExecutionDisplayStatus**](JobExecutionDisplayStatus.md)> |  | [optional]
**job_trigger_filter** | Option<[**crate::models::JobTriggerType**](JobTriggerType.md)> |  | [optional]
**branch** | Option<[**crate::models::Branch**](Branch.md)> |  | [optional]
**meeting** | [**crate::models::DtoMeeting**](DTO_Meeting.md) |  | 
**compact** | Option<**bool**> |  | [optional]
**header** | [**crate::models::McMessage**](MCMessage.md) |  | 
**header_attachments** | [**Vec<crate::models::AttachmentInfo>**](AttachmentInfo.md) |  | 
**lines** | [**Vec<crate::models::CodeLine>**](CodeLine.md) |  | 
**selected_line_index** | **i32** |  | 
**selected_lines_count** | **i32** |  | 
**source_branch** | Option<**String**> |  | [optional]
**target_branch** | Option<**String**> |  | [optional]
**anchor** | [**crate::models::CodeSnippetAnchor**](CodeSnippetAnchor.md) |  | 
**issue** | [**crate::models::ExternalIssue**](ExternalIssue.md) |  | 
**strike_through** | Option<**bool**> |  | 
**location** | [**crate::models::TdLocation**](TD_Location.md) |  | 
**checklist** | [**crate::models::Checklist**](Checklist.md) |  | 
**snapshot_id** | **String** |  | 
**base_snapshot_id** | **String** |  | 
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**repository** | **String** |  | 
**branch_head** | **String** |  | 
**deleted** | **bool** |  | 
**is_default** | Option<**bool**> |  | [optional]
**tag_size** | Option<[**crate::models::UnfurlDetailsRepositoryBranchJbsBranchTagSize**](UnfurlDetailsRepositoryBranchJbsBranchTagSize.md)> |  | [optional]
**name** | **String** |  | 
**root** | **bool** |  | 
**folder** | Option<[**crate::models::DocumentFolder**](DocumentFolder.md)> |  | [optional]
**text_before** | **String** |  | 
**text_after** | **String** |  | 
**commit_id** | **String** |  | 
**team** | [**crate::models::TdTeam**](TD_Team.md) |  | 
**contact_key** | **String** |  | 
**profile** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**with_icon** | Option<**bool**> |  | [optional]
**with_branch_pair** | **bool** |  | 
**default_branch_in_repo** | Option<**String**> |  | [optional]
**hide_if_cannot_resolve** | Option<**bool**> |  | [optional]
**review_state** | Option<[**crate::models::CodeReviewState**](CodeReviewState.md)> |  | [optional]
**is_merged** | Option<**bool**> |  | [optional]
**sprint** | [**crate::models::SprintRecord**](SprintRecord.md) |  | 
**removed** | Option<**bool**> |  | [optional]
**tag** | [**crate::models::PlanningTag**](PlanningTag.md) |  | 
**text_size** | Option<[**crate::models::MessageTextSize**](MessageTextSize.md)> |  | [optional]
**document** | [**crate::models::Document**](Document.md) |  | 
**version2** | Option<**String**> |  | [optional]
**base2** | Option<**String**> |  | [optional]
**preview2** | Option<**String**> |  | [optional]
**project_id** | **String** |  | 
**issue_id** | **String** |  | 
**status** | [**crate::models::IssueStatus**](IssueStatus.md) |  | 
**draft** | **String** |  | 
**import_transaction** | [**crate::models::ImportTransactionRecord**](ImportTransactionRecord.md) |  | 
**marketplace_app_id** | Option<**String**> |  | [optional]
**utc_milliseconds** | **i64** |  | 
**params** | Option<[**crate::models::DateTimeViewParams**](DateTimeViewParams.md)> |  | [optional]
**repo_ref** | [**crate::models::ProjectPackageRepository**](ProjectPackageRepository.md) |  | 
**package_name** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**role** | [**crate::models::TdRole**](TD_Role.md) |  | 
**repository_id** | Option<**String**> |  | [optional]
**message_unfurls** | Option<[**crate::models::CommitMessageUnfurlsRecord**](CommitMessageUnfurlsRecord.md)> |  | [optional]
**commit_date** | **String** |  | 
**author_date** | Option<**String**> |  | [optional]
**author_profile** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**hide_author_and_date** | Option<**bool**> |  | [optional]
**with_branch_tags** | Option<**bool**> |  | [optional]
**review_id** | Option<**String**> |  | [optional]
**app** | [**crate::models::EsApp**](ES_App.md) |  | 
**deployment_ref** | [**crate::models::DeploymentRecord**](DeploymentRecord.md) |  | 
**show_details** | Option<**bool**> |  | [optional]
**show_status** | Option<**bool**> |  | [optional]
**since** | **i64** |  | 
**till** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


