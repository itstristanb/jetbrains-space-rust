# MergeRequestRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**project** | [**crate::models::ProjectKey**](ProjectKey.md) |  | 
**project_id** | **String** |  | 
**number** | **i32** |  | 
**title** | **String** |  | 
**state** | [**crate::models::CodeReviewState**](CodeReviewState.md) |  | 
**can_be_reopened** | Option<**bool**> |  | [optional]
**created_at** | **i64** |  | 
**created_by** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**timestamp** | Option<**i64**> |  | [optional]
**turn_based** | Option<**bool**> |  | [optional]
**feed_channel** | Option<[**crate::models::M2ChannelRecord**](M2ChannelRecord.md)> |  | [optional]
**feed_channel_id** | Option<**String**> |  | [optional]
**branch_pairs** | [**Vec<crate::models::MergeRequestBranchPair>**](MergeRequestBranchPair.md) |  | 
**read_only** | Option<**bool**> |  | [optional]
**external_link** | Option<[**crate::models::ExternalCodeReviewLink**](ExternalCodeReviewLink.md)> |  | [optional]
**participants** | Option<[**Vec<crate::models::CodeReviewParticipant>**](CodeReviewParticipant.md)> |  | [optional]
**reviewers** | [**Vec<crate::models::CodeReviewParticipantRecord>**](CodeReviewParticipantRecord.md) |  | 
**authors** | [**Vec<crate::models::CodeReviewParticipantRecord>**](CodeReviewParticipantRecord.md) |  | 
**watchers** | [**Vec<crate::models::CodeReviewParticipantRecord>**](CodeReviewParticipantRecord.md) |  | 
**discussion_counter** | [**crate::models::DiscussionCounter**](DiscussionCounter.md) |  | 
**commits** | [**Vec<crate::models::ReviewCommit>**](ReviewCommit.md) |  | 
**issue_ids** | **Vec<String>** |  | 
**external_issues** | Option<[**Vec<crate::models::ExternalIssueIdOut>**](ExternalIssueIdOut.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**unfurls** | [**Vec<crate::models::Attachment>**](Attachment.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


