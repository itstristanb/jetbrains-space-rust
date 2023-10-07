# AbsenceRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**member** | [**crate::models::TdMemberProfile**](TD_MemberProfile.md) |  | 
**icon** | **String** |  | 
**reason** | Option<[**crate::models::AbsenceReasonRecord**](AbsenceReasonRecord.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**since** | **String** |  | 
**till** | **String** |  | 
**location** | Option<[**crate::models::TdLocation**](TD_Location.md)> |  | [optional]
**available** | **bool** |  | 
**approval** | Option<[**crate::models::AbsenceApproval**](AbsenceApproval.md)> |  | [optional]
**category** | Option<**String**> |  | [optional]
**custom_fields** | [**::std::collections::HashMap<String, crate::models::CfValue>**](CFValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


