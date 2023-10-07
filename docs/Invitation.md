# Invitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**expires_at** | **String** |  | 
**invitee_email** | **String** |  | 
**invitee_email_blocked** | **bool** |  | 
**invitee_email_blocked_reason** | Option<**String**> |  | [optional]
**invitee_first_name** | Option<**String**> |  | [optional]
**invitee_last_name** | Option<**String**> |  | [optional]
**invitee** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**inviter** | [**crate::models::CPrincipal**](CPrincipal.md) |  | 
**team** | Option<[**crate::models::TdTeam**](TD_Team.md)> |  | [optional]
**role** | Option<[**crate::models::TdRole**](TD_Role.md)> |  | [optional]
**project** | Option<[**crate::models::PrProject**](PR_Project.md)> |  | [optional]
**project_role** | Option<[**crate::models::ProjectTeamRole**](ProjectTeamRole.md)> |  | [optional]
**global_role** | Option<[**crate::models::GlobalRole**](GlobalRole.md)> |  | [optional]
**revoked** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


