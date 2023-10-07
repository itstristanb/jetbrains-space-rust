# CustomFieldsV2EntityTypeFieldsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**r#type** | [**crate::models::CustomFieldType**](CustomFieldType.md) |  | 
**multivalued** | Option<**bool**> |  | [optional][default to false]
**parameters** | Option<[**crate::models::CfCreateParameters**](CFCreateParameters.md)> |  | [optional]
**required** | Option<**bool**> |  | [optional][default to false]
**default_value** | Option<[**crate::models::CfInputValue**](CFInputValue.md)> |  | [optional]
**constraint** | Option<[**crate::models::CfConstraint**](CFConstraint.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**order** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


