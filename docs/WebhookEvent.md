# WebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**meta** | [**crate::models::KMetaMod**](KMetaMod.md) |  | 
**document** | **String** |  | 
**deleted** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**published** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**emoji** | **String** |  | 
**owner** | Option<[**crate::models::ApplicationWebhookEventOwner**](ApplicationWebhookEvent_owner.md)> |  | 
**attachment_id** | **String** |  | 
**uploaded_at** | **String** |  | 
**message_id** | **String** |  | 
**channel_id** | **String** |  | 
**thread_id** | Option<**String**> |  | [optional]
**actor** | [**crate::models::CPrincipal**](CPrincipal.md) |  | 
**new_count** | **i32** |  | 
**message** | [**crate::models::ChannelItemRecord**](ChannelItemRecord.md) |  | 
**meeting** | [**crate::models::DtoMeeting**](DTO_Meeting.md) |  | 
**project_key** | [**crate::models::ProjectKey**](ProjectKey.md) |  | 
**repository** | **String** |  | 
**commit** | [**crate::models::GitCommitInfoWithChanges**](GitCommitInfoWithChanges.md) |  | 
**channel** | [**crate::models::M2ChannelRecord**](M2ChannelRecord.md) |  | 
**name** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | 
**description** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**icon** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**restored** | Option<**bool**> |  | [optional]
**archived** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**webhook_name** | **String** |  | 
**review** | [**crate::models::CodeReviewRecord**](CodeReviewRecord.md) |  | 
**folder** | **String** |  | 
**article** | [**crate::models::ArticleRecord**](ArticleRecord.md) |  | 
**action** | [**crate::models::PackageRepositoryEventAction**](PackageRepositoryEventAction.md) |  | 
**author** | Option<[**crate::models::ApplicationWebhookEventOwner**](ApplicationWebhookEvent_owner.md)> |  | [optional]
**created** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**title** | **String** |  | 
**text_changed** | **bool** |  | 
**unpublished** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**teams** | Option<[**crate::models::BlogWebhookEventTeams**](BlogWebhookEvent_teams.md)> |  | [optional]
**locations** | Option<[**crate::models::BlogWebhookEventLocations**](BlogWebhookEvent_locations.md)> |  | [optional]
**external_entity_info** | Option<[**crate::models::BlogWebhookEventExternalEntityInfo**](BlogWebhookEvent_externalEntityInfo.md)> |  | [optional]
**member** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**issue_number** | Option<**i32**> |  | [optional]
**enabled_for_all** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**self_manageable** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**added_teams** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**added_profiles** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**removed_teams** | Option<[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)> |  | [optional]
**removed_profiles** | Option<[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)> |  | [optional]
**is_merge_request** | **bool** |  | 
**participant** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**reviewer_state** | Option<[**crate::models::CodeReviewParticipantWebhookEventReviewerState**](CodeReviewParticipantWebhookEvent_reviewerState.md)> |  | [optional]
**their_turn** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**target_id** | **String** |  | 
**deployment_id** | **String** |  | 
**status_mod** | Option<[**crate::models::DeploymentWebhookEventStatusMod**](DeploymentWebhookEvent_statusMod.md)> |  | [optional]
**repository_type** | [**crate::models::PackageType**](PackageType.md) |  | 
**package_info** | [**crate::models::PackageVersionRef**](PackageVersionRef.md) |  | 
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**location** | Option<[**crate::models::AbsenceWebhookEventLocation**](AbsenceWebhookEvent_location.md)> |  | 
**issue** | [**crate::models::Issue**](Issue.md) |  | 
**assignee** | Option<[**crate::models::ApplicationWebhookEventOwner**](ApplicationWebhookEvent_owner.md)> |  | [optional]
**status** | Option<[**crate::models::AutomationJobEventStatus**](AutomationJobEvent_status.md)> |  | [optional]
**due_date** | Option<[**crate::models::AbsenceWebhookEventSince**](AbsenceWebhookEvent_since.md)> |  | [optional]
**tag_delta** | Option<[**crate::models::IssueWebhookEventTagDelta**](IssueWebhookEvent_tagDelta.md)> |  | [optional]
**checklist_delta** | Option<[**crate::models::IssueWebhookEventChecklistDelta**](IssueWebhookEvent_checklistDelta.md)> |  | [optional]
**sprint_delta** | Option<[**crate::models::IssueWebhookEventSprintDelta**](IssueWebhookEvent_sprintDelta.md)> |  | [optional]
**custom_field_update** | Option<[**crate::models::IssueWebhookCustomFieldUpdate**](IssueWebhookCustomFieldUpdate.md)> |  | [optional]
**email** | **String** |  | 
**reason** | Option<[**crate::models::AbsenceWebhookEventReason**](AbsenceWebhookEvent_reason.md)> |  | 
**absence** | [**crate::models::AbsenceRecord**](AbsenceRecord.md) |  | 
**since** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**till** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**available** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**application** | [**crate::models::EsApp**](ES_App.md) |  | 
**fingerprint** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**comment** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**changes** | [**crate::models::RepoChanges**](RepoChanges.md) |  | 
**edited** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**pinned** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**team** | Option<[**crate::models::TeamMembershipEventTeam**](TeamMembershipEvent_team.md)> |  | 
**discussion** | [**crate::models::CodeReviewDiscussion**](CodeReviewDiscussion.md) |  | 
**resolved** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**approved** | [**crate::models::AbsenceApprovalWebhookEventApproved**](AbsenceApprovalWebhookEvent_approved.md) |  | 
**endpoint_uri** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**client_id_changed** | **bool** |  | 
**client_secret_changed** | **bool** |  | 
**verification_token_changed** | **bool** |  | 
**signing_key_changed** | **bool** |  | 
**warmup_execution** | [**crate::models::RdWarmupExec**](Rd_WarmupExec.md) |  | 
**review_id** | **String** |  | 
**target** | [**crate::models::ApplicationUnfurlTarget**](ApplicationUnfurlTarget.md) |  | 
**relation** | **String** |  | 
**entity** | **String** |  | 
**scope** | [**crate::models::AuthScope**](AuthScope.md) |  | 
**title_mod** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**description_mod** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**target_branch_mod** | Option<[**crate::models::AbsenceWebhookEventIcon**](AbsenceWebhookEvent_icon.md)> |  | [optional]
**document_ref** | Option<[**crate::models::Document**](Document.md)> |  | [optional]
**change_authors** | [**Vec<crate::models::CPrincipal>**](CPrincipal.md) |  | 
**version** | **String** |  | 
**base** | **String** |  | 
**joined_organization** | **bool** |  | 
**left_organization** | **bool** |  | 
**import_transaction** | [**crate::models::ImportTransactionRecord**](ImportTransactionRecord.md) |  | 
**membership** | [**crate::models::TdMembership**](TD_Membership.md) |  | 
**role** | Option<[**crate::models::TeamMembershipEventRole**](TeamMembershipEvent_role.md)> |  | [optional]
**lead** | Option<[**crate::models::AbsenceWebhookEventAvailable**](AbsenceWebhookEvent_available.md)> |  | [optional]
**manager** | Option<[**crate::models::ApplicationWebhookEventOwner**](ApplicationWebhookEvent_owner.md)> |  | [optional]
**approved_by** | Option<[**crate::models::ApplicationWebhookEventOwner**](ApplicationWebhookEvent_owner.md)> |  | [optional]
**execution_id** | **String** |  | 
**repository_name** | **String** |  | 
**job_name** | **String** |  | 
**execution_number** | **i64** |  | 
**trigger** | [**crate::models::JobExecutionTrigger**](JobExecutionTrigger.md) |  | 
**trigger_time** | **String** |  | 
**failure_reasons** | Option<[**crate::models::AutomationJobEventFailureReasons**](AutomationJobEvent_failureReasons.md)> |  | [optional]
**stopped_by** | Option<[**crate::models::AutomationJobEventStoppedBy**](AutomationJobEvent_stoppedBy.md)> |  | [optional]
**start_time** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**end_time** | Option<[**crate::models::AutomationJobEventStartTime**](AutomationJobEvent_startTime.md)> |  | [optional]
**repositories** | Option<[**Vec<crate::models::GitCheckout>**](GitCheckout.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


