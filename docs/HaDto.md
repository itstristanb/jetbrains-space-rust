# HaDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**fields** | [**Vec<crate::models::HaDtoField>**](HA_DtoField.md) |  | 
**hierarchy_role** | [**crate::models::HierarchyRole**](HierarchyRole.md) |  | 
**hierarchy_role2** | [**crate::models::HierarchyRole2**](HierarchyRole2.md) |  | 
**extends** | Option<[**crate::models::HaDto**](HA_Dto.md)> |  | [optional]
**implements** | [**Vec<crate::models::HaDto>**](HA_Dto.md) |  | 
**inheritors** | [**Vec<crate::models::HaDto>**](HA_Dto.md) |  | 
**description** | Option<[**crate::models::HaDescription**](HA_Description.md)> |  | [optional]
**deprecation** | Option<[**crate::models::HaDeprecation**](HA_Deprecation.md)> |  | [optional]
**experimental** | Option<[**crate::models::HaExperimental**](HA_Experimental.md)> |  | [optional]
**record** | **bool** |  | 
**feature_flag** | Option<**String**> |  | [optional]
**errors_field** | Option<[**crate::models::HaField**](HA_Field.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


