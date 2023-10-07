# FeedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**uid** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**change_type** | [**crate::models::ReviewRevisionsChangedType**](ReviewRevisionsChangedType.md) |  | 
**project_key** | Option<**String**> |  | 
**review_id** | **String** |  | 
**review_number** | **i32** |  | 
**review_type** | [**crate::models::ReviewType**](ReviewType.md) |  | 
**description** | Option<[**crate::models::CodeReviewDescription**](CodeReviewDescription.md)> |  | [optional]
**description_edited_by_profile_ids** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**commits** | [**Vec<crate::models::RepositoryCommitRecord>**](RepositoryCommitRecord.md) |  | 
**code_discussion** | [**crate::models::CodeDiscussionRecord**](CodeDiscussionRecord.md) |  | 
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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


