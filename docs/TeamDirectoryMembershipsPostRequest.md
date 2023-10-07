# TeamDirectoryMembershipsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**member** | [**crate::models::ProfileIdentifier**](ProfileIdentifier.md) |  | 
**team_id** | **String** |  | 
**role_id** | **String** |  | 
**lead** | Option<**bool**> |  | [optional][default to false]
**manager** | Option<[**crate::models::ProfileIdentifier**](ProfileIdentifier.md)> |  | [optional]
**active_since** | Option<**String**> |  | [optional]
**active_till** | Option<**String**> |  | [optional]
**previous_membership_id** | Option<**String**> |  | [optional]
**requires_approval** | Option<**bool**> |  | [optional][default to false]
**custom_field_values** | Option<[**Vec<crate::models::CustomFieldInputValue>**](CustomFieldInputValue.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


