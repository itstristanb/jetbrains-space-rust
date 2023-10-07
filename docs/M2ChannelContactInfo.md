# M2ChannelContactInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**code_review_discussion** | **String** |  | 
**notification_defaults** | [**crate::models::ChannelSpecificDefaults**](ChannelSpecificDefaults.md) |  | 
**code_review_id** | **String** |  | 
**code_review** | Option<[**crate::models::CodeReviewRecord**](CodeReviewRecord.md)> |  | [optional]
**participants** | Option<[**crate::models::CodeReviewParticipants**](CodeReviewParticipants.md)> |  | [optional]
**pending_message_counter** | Option<[**crate::models::CodeReviewPendingMessageCounter**](CodeReviewPendingMessageCounter.md)> |  | [optional]
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**app** | [**crate::models::EsApp**](ES_App.md) |  | 
**deploy_target** | [**crate::models::DeployTargetRecord**](DeployTargetRecord.md) |  | 
**name** | **String** |  | 
**can_have_threads** | **bool** |  | 
**color** | Option<[**crate::models::PrivateFeedColor**](PrivateFeedColor.md)> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**job_subscription** | [**crate::models::JobSubscription**](JobSubscription.md) |  | 
**job_name** | **String** |  | 
**repo_name** | Option<**String**> |  | [optional]
**channel_id** | **String** |  | 
**subject** | Option<**String**> |  | [optional]
**members** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**session_id** | **String** |  | 
**session_name** | **String** |  | 
**llm_provider** | **String** |  | 
**group** | **String** |  | 
**access** | [**crate::models::M2JbsAccess**](M2JbsAccess.md) |  | 
**description** | **String** |  | 
**icon_id** | Option<**String**> |  | [optional]
**teams** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**can_edit** | Option<**bool**> |  | [optional]
**workspace_id** | **String** |  | 
**workspace_name** | **String** |  | 
**deployment** | [**crate::models::DeploymentRecord**](DeploymentRecord.md) |  | 
**target** | Option<[**crate::models::DeployTargetRecord**](DeployTargetRecord.md)> |  | [optional]
**project_key** | Option<[**crate::models::ProjectKey**](ProjectKey.md)> |  | [optional]
**issue** | [**crate::models::Issue**](Issue.md) |  | 
**member** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**member_teams** | Option<[**crate::models::ProfileMembershipRecord**](ProfileMembershipRecord.md)> |  | [optional]
**parent** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**text** | Option<**String**> |  | [optional]
**message_id** | **String** |  | 
**message_author** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**attachments** | Option<**String**> |  | [optional]
**code_discussion** | [**crate::models::CodeDiscussionRecord**](CodeDiscussionRecord.md) |  | 
**article** | [**crate::models::ArticleRecord**](ArticleRecord.md) |  | 
**support_replies** | Option<**bool**> |  | [optional]
**code_discussion_id** | **String** |  | 
**cause** | Option<[**crate::models::M2ObsoleteCause**](M2ObsoleteCause.md)> |  | [optional]
**session** | [**crate::models::CallSession**](CallSession.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


