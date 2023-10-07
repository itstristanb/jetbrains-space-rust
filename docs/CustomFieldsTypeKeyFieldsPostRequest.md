# CustomFieldsTypeKeyFieldsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**r#type** | [**crate::models::CfType**](CFType.md) |  | 
**constraint** | Option<[**crate::models::CfConstraint**](CFConstraint.md)> |  | [optional]
**required** | **bool** |  | 
**private** | **bool** |  | 
**access** | Option<[**crate::models::AccessType**](AccessType.md)> |  | [optional]
**default_value** | [**crate::models::CfInputValue**](CFInputValue.md) |  | 
**open_enum_values_modification** | Option<[**crate::models::CfEnumValuesModification**](CFEnumValuesModification.md)> |  | [optional]
**cf_parameters** | Option<[**crate::models::CfCreateParameters**](CFCreateParameters.md)> |  | [optional]
**scope** | [**crate::models::ExtendedTypeScope**](ExtendedTypeScope.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


