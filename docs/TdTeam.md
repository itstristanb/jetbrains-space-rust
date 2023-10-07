# TdTeam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**parent** | Option<[**crate::models::TdTeam**](TD_Team.md)> |  | [optional]
**emails** | Option<**Vec<String>**> |  | [optional]
**channel_id** | Option<**String**> |  | [optional]
**archived** | **bool** |  | 
**disbanded** | Option<**bool**> |  | [optional]
**disbanded_at** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**default_manager** | Option<[**crate::models::TdMemberProfile**](TD_MemberProfile.md)> |  | [optional]
**memberships** | [**Vec<crate::models::TdMembership>**](TD_Membership.md) |  | 
**custom_fields** | [**::std::collections::HashMap<String, crate::models::CfValue>**](CFValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


