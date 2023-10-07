# TdMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**member** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**team** | [**crate::models::TdTeam**](TD_Team.md) |  | 
**role** | [**crate::models::TdRole**](TD_Role.md) |  | 
**lead** | **bool** |  | 
**manager** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**since** | Option<**String**> |  | [optional]
**till** | Option<**String**> |  | [optional]
**active_since** | Option<**String**> |  | [optional]
**active_till** | Option<**String**> |  | [optional]
**requires_approval** | **bool** |  | 
**archived** | **bool** |  | 
**edit_for** | Option<[**crate::models::TdMembership**](TD_Membership.md)> |  | [optional]
**pending_edit** | Option<[**crate::models::TdMembership**](TD_Membership.md)> |  | [optional]
**approver** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**custom_fields** | [**::std::collections::HashMap<String, crate::models::CfValue>**](CFValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


