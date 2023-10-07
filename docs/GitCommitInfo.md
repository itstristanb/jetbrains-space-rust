# GitCommitInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**message** | **String** |  | 
**author_date** | **i64** |  | 
**commit_date** | **i64** |  | 
**author** | [**crate::models::GitAuthorInfo**](GitAuthorInfo.md) |  | 
**author_profile** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**committer** | [**crate::models::GitAuthorInfo**](GitAuthorInfo.md) |  | 
**committer_profile** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**parents** | **Vec<String>** |  | 
**heads** | **Vec<String>** |  | 
**signature** | Option<[**crate::models::GitCommitSignature**](GitCommitSignature.md)> |  | [optional]
**committer_is_space** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


