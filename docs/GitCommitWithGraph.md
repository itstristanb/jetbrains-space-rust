# GitCommitWithGraph

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**repository_name** | **String** |  | 
**commit** | [**crate::models::GitCommitInfo**](GitCommitInfo.md) |  | 
**commit_message_unfurls** | [**Vec<crate::models::Unfurl>**](Unfurl.md) |  | 
**reviews** | [**Vec<crate::models::CodeReviewRecord>**](CodeReviewRecord.md) |  | 
**issue_ids** | **Vec<String>** |  | 
**linked_issues** | Option<[**Vec<crate::models::GenericIssueId>**](GenericIssueId.md)> |  | [optional]
**deployments** | [**Vec<crate::models::GitCommitWithGraphDeploymentsInner>**](GitCommitWithGraph_deployments_inner.md) |  | 
**layout** | Option<[**crate::models::GitGraphLayoutLine**](GitGraphLayoutLine.md)> |  | [optional]
**unreachable** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


