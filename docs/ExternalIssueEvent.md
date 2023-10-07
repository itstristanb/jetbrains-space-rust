# ExternalIssueEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**repository_id** | **String** |  | 
**commit_id** | **String** |  | 
**issue_prefix** | **String** |  | 
**issue_id** | **String** |  | 
**repositories** | [**Vec<crate::models::ExternalIssueLinkedCommitsForRepo>**](ExternalIssueLinkedCommitsForRepo.md) |  | 
**review** | [**crate::models::MergeRequestRecord**](MergeRequestRecord.md) |  | 
**issues** | [**Vec<crate::models::ExternalIssueIdOut>**](ExternalIssueIdOut.md) |  | 
**repository_name** | Option<**String**> |  | 
**commit** | [**crate::models::GitCommitInfo**](GitCommitInfo.md) |  | 
**commit_changes** | Option<[**crate::models::GitCommitChanges**](GitCommitChanges.md)> |  | [optional]
**commit_url** | **String** |  | 
**changes** | Option<[**crate::models::GitCommitChanges**](GitCommitChanges.md)> |  | [optional]
**url** | Option<**String**> |  | [optional]
**reviews** | [**Vec<crate::models::CodeReviewRecord>**](CodeReviewRecord.md) |  | 
**new_status** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


