# InlineUnfurlDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**session** | [**crate::models::CallSession**](CallSession.md) |  | 
**location** | [**crate::models::TdLocation**](TD_Location.md) |  | 
**strike_through** | Option<**bool**> |  | 
**checklist** | [**crate::models::Checklist**](Checklist.md) |  | 
**snapshot_id** | **String** |  | 
**base_snapshot_id** | **String** |  | 
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**repository** | **String** |  | 
**branch_head** | **String** |  | 
**deleted** | **bool** |  | 
**is_default** | Option<**bool**> |  | [optional]
**tag_size** | Option<[**crate::models::UnfurlDetailsRepositoryBranchJbsBranchTagSize**](UnfurlDetailsRepositoryBranchJbsBranchTagSize.md)> |  | [optional]
**article** | [**crate::models::ArticleRecord**](ArticleRecord.md) |  | 
**content** | Option<[**crate::models::ArticleContentRecord**](ArticleContentRecord.md)> |  | [optional]
**channel** | **String** |  | 
**details** | Option<[**crate::models::ArticleDetailsRecord**](ArticleDetailsRecord.md)> |  | [optional]
**name** | **String** |  | 
**root** | **bool** |  | 
**folder** | Option<[**crate::models::DocumentFolder**](DocumentFolder.md)> |  | [optional]
**title** | **String** |  | 
**target_ref** | [**crate::models::DeployTargetRecord**](DeployTargetRecord.md) |  | 
**target_name** | Option<**String**> |  | [optional]
**show_link_icon** | Option<**bool**> |  | [optional]
**skip_details_render** | Option<**bool**> |  | [optional]
**text_before** | **String** |  | 
**text_after** | **String** |  | 
**commit_id** | **String** |  | 
**message** | **String** |  | 
**job_id** | **String** |  | 
**job_name** | **String** |  | 
**project_ref** | [**crate::models::PrProject**](PR_Project.md) |  | 
**repo_name** | **String** |  | 
**job_execution_display_status_filter** | Option<[**crate::models::JobExecutionDisplayStatus**](JobExecutionDisplayStatus.md)> |  | [optional]
**job_trigger_filter** | Option<[**crate::models::JobTriggerType**](JobTriggerType.md)> |  | [optional]
**branch** | Option<[**crate::models::Branch**](Branch.md)> |  | [optional]
**team** | [**crate::models::TdTeam**](TD_Team.md) |  | 
**issue** | [**crate::models::ExternalIssue**](ExternalIssue.md) |  | 
**compact** | Option<**bool**> |  | [optional]
**contact_key** | **String** |  | 
**profile** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**review** | [**crate::models::CodeReviewRecord**](CodeReviewRecord.md) |  | 
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
**id** | [**crate::models::ExternalIssueId**](ExternalIssueId.md) |  | 
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
**author** | [**crate::models::GitAuthorInfo**](GitAuthorInfo.md) |  | 
**author_profile** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**hide_author_and_date** | Option<**bool**> |  | [optional]
**with_branch_tags** | Option<**bool**> |  | [optional]
**review_id** | Option<**String**> |  | [optional]
**app** | [**crate::models::EsApp**](ES_App.md) |  | 
**deployment_ref** | [**crate::models::DeploymentRecord**](DeploymentRecord.md) |  | 
**show_details** | Option<**bool**> |  | [optional]
**show_status** | Option<**bool**> |  | [optional]
**job_execution_id** | **String** |  | 
**meeting** | [**crate::models::DtoMeeting**](DTO_Meeting.md) |  | 
**since** | **i64** |  | 
**till** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


